use std::sync::{Arc, Mutex};

use futures::future;
use futures::task::Poll;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex as TauriMutex;
use tauri::{AppHandle, Manager};
use tokio;
use tokio::time;
use tokio_tungstenite::connect_async;
use tungstenite;

use crate::state::{
    Amplifier,
    AppState,
    AppStateMutex,
    Message,
    Source,
    StreamerDisplay,
    StreamerSources,
    TransportState,
    VibinConnectionState,
    VibinStateMutex,
};

// TODO: Handle WebSocket connection errors for bubbling back to the UI, e.g.:
//   Io(Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" })
// TODO: Handle initial state (e.g. ensure full transport state is known at startup, before next
//  track starts.

// These represent messages received from the Vibin WebSocket server.

#[derive(Deserialize)]
struct VibinMessage {
    // id: String,
    // client_id: String,
    // time: isize,
    #[serde(rename = "type")]
    msg_type: String,
    payload: serde_json::Value,
}

#[derive(Deserialize)]
struct StreamerPayload {
    sources: Option<StreamerSources>,
    display: Option<StreamerDisplay>,
}

#[derive(Deserialize)]
struct AmplifierPayload {
    mute: Option<String>,
    volume: Option<f32>,
}

#[derive(Deserialize)]
struct SystemPayload {
    power: Option<String>,
    streamer: StreamerPayload,
    amplifier: Option<AmplifierPayload>,
}

#[derive(Serialize, Deserialize)]
struct TransportStatePayload {
    pub play_state: Option<String>,
    pub active_controls: Vec<String>,
    pub repeat: Option<String>,
    pub shuffle: Option<String>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize)]
pub struct AppError {
    pub message: String,
}

// ------------------------------------------------------------------------------------------------

trait CustomEmitters {
    fn emit_app_state(&self, app_state: &AppState);
    fn emit_error(&self, error_message: &str);
}

impl CustomEmitters for AppHandle {
    fn emit_app_state(&self, app_state: &AppState) {
        self.emit_all(&Message::AppState.to_string(), app_state).unwrap();
        println!("SENT: {:?}", app_state);
    }

    fn emit_error(&self, error_message: &str) {
        self.emit_all(
            &Message::Error.to_string(),
            AppError {
                message: error_message.into(),
            },
        )
        .unwrap();
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct WebSocketManager {
    pub connection: Arc<TauriMutex<WebSocketConnection>>,
    pub vibin_server: Box<String>,
    pub stop_flag: Arc<Mutex<bool>>,
    pub app_state_mutex: AppStateMutex,
    pub vibin_state_mutex: VibinStateMutex,
    pub app_handle: AppHandle,

    pub is_started: bool,
}

impl WebSocketManager {
    pub fn new(
        connection: Arc<TauriMutex<WebSocketConnection>>,
        vibin_server: Box<String>,
        stop_flag: Arc<Mutex<bool>>,
        app_state_mutex: AppStateMutex,
        vibin_state_mutex: VibinStateMutex,
        app_handle: AppHandle,
    ) -> Self {
        WebSocketManager {
            connection,
            vibin_server,
            stop_flag,
            app_state_mutex,
            vibin_state_mutex,
            app_handle,

            is_started: false,
        }
    }

    pub fn start(&mut self) {
        if self.is_started == true {
            println!("WebSocketManager is already started; ignoring start request");
            return;
        }

        self.is_started = true;

        println!("WebSocketManager has been started");
        let self_clone = self.clone();
        *self.stop_flag.lock().unwrap() = false;

        tauri::async_runtime::spawn(async move {
            println!("WebSocketManager is starting the WebSocketConnection");
            self_clone
                .connection
                .lock()
                .await
                .start(
                    &self_clone.vibin_server.clone(),
                    &self_clone.stop_flag.clone(),
                    &self_clone.app_state_mutex,
                    &self_clone.vibin_state_mutex,
                    self_clone.app_handle.clone(),
                )
                .await;

            println!("WebSocketManager start() of WebSocketConnection has completed");
        });
    }

    pub async fn stop(&mut self) {
        println!("WebSocketManager requesting WebSocketConnection disconnect");
        *self.stop_flag.lock().unwrap() = true;

        println!("WebSocketManager waiting for disconnect");

        while self.app_state_mutex.lock().unwrap().vibin_connection
            != VibinConnectionState::Disconnected
        {
            time::sleep(time::Duration::from_millis(100)).await;
        }

        // wait_for_disconnected.await;
        println!("WebSocketManager has detected WebSocketConnection disconnect");

        self.is_started = false;
    }
}

unsafe impl Send for WebSocketManager {}

pub type WebSocketManagerMutex = Arc<TauriMutex<WebSocketManager>>;

pub struct WebSocketConnection {
    pub stop_flag: Option<Arc<Mutex<bool>>>,
    pub vibin_server: String,
}

unsafe impl Send for WebSocketConnection {}

impl WebSocketConnection {
    fn process_message(
        &self,
        vibin_msg: VibinMessage,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
    ) {
        let mut system_state = vibin_state_mutex.lock().unwrap();
        let mut send_update_to_client = false;

        match vibin_msg.msg_type.as_str() {
            "System" => {
                let system_payload: SystemPayload =
                    serde_json::from_value(vibin_msg.payload).unwrap();

                system_state.power = system_payload.power;

                if let Some(amplifier) = system_payload.amplifier {
                    system_state.amplifier = Some(Amplifier {
                        mute: amplifier.mute,
                        volume: amplifier.volume,
                    });
                }

                if let Some(display) = system_payload.streamer.display {
                    system_state.display.line1 = display.line1;
                    system_state.display.line2 = display.line2;
                    system_state.display.line3 = display.line3;
                    system_state.display.format = display.format;
                    system_state.display.playback_source = display.playback_source;
                    system_state.display.art_url = display.art_url;
                }

                if let Some(sources) = system_payload.streamer.sources {
                    system_state.source = Some(sources.active);
                }

                send_update_to_client = true;
            }
            "TransportState" => {
                let transport_payload: TransportStatePayload =
                    serde_json::from_value(vibin_msg.payload).unwrap();

                system_state.transport = Some(TransportState {
                    play_state: transport_payload.play_state,
                    active_controls: transport_payload.active_controls,
                    repeat: transport_payload.repeat,
                    shuffle: transport_payload.shuffle,
                });
            }
            "Position" => {}
            _ => {}
        }

        if send_update_to_client {
            app_handle
                .emit_all(&Message::VibinState.to_string(), &(*system_state))
                .unwrap();
        }
    }

    async fn handle_websocket(
        &self,
        app_state_mutex: &AppStateMutex,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
    ) -> Result<(), tungstenite::Error> {
        {
            // NOTE: app_state_mutex is locked inside a block to ensure the lock is released before
            //  the subsequent .await call. Ref: https://tokio.rs/tokio/tutorial/shared-state
            let mut app_state = app_state_mutex.lock().unwrap();

            if app_state.vibin_connection != VibinConnectionState::Disconnected {
                println!(
                    "Connection state is not {:?}; not proceeding with Vibin WebSocket connection",
                    VibinConnectionState::Disconnected
                );
                return Ok(());
            }

            app_state.vibin_connection = VibinConnectionState::Connecting;
            app_handle.emit_app_state(&app_state);
        }

        // let url = url::Url::parse("ws://192.168.2.101:8080/ws").unwrap();
        let url = url::Url::parse(self.vibin_server.as_str()).unwrap();
        let (ws_stream, _) = connect_async(&url).await?;

        {
            let mut app_state = app_state_mutex.lock().unwrap();

            println!(
                "Connected to Vibin WebSocket server: {:?}",
                self.vibin_server
            );
            app_state.vibin_connection = VibinConnectionState::Connected;
            app_handle.emit_app_state(&app_state);
        }

        // This is a read-only websocket connection, so we ignore the write stream.
        let (_, read) = ws_stream.split();

        let stop_reading = future::poll_fn(|_context| {
            if *self.stop_flag.as_ref().unwrap().lock().unwrap() == true {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        });

        read.take_until(stop_reading)
            .for_each(|message| async {
                let msg = message.unwrap();

                if msg.is_ping() {
                    println!("Got ping");
                } else {
                    match msg.into_text() {
                        Ok(message_text) => match serde_json::from_str::<VibinMessage>(&message_text) {
                            Ok(vibin_msg) => {
                                let app_handle = app_handle.clone();
                                self.process_message(vibin_msg, vibin_state_mutex, app_handle);
                            }
                            Err(e) => app_handle.emit_error(&format!(
                                "Could not deserialize WebSocket message; error: {:?} :: message: {}",
                                e, message_text
                            )),
                        },
                        Err(e) => {
                            println!("MESSAGE TAKE ERROR: {:?}", e);
                            app_handle.emit_error(&format!(
                                "Could not extract text from WebSocket message: {:?}",
                                e
                            ))
                        },
                    }
                }
            })
            .await;

        // TODO: Investigate what happens when the WebSocket connection is disrupted.
        let mut app_state = app_state_mutex.lock().unwrap();
        app_state.vibin_connection = VibinConnectionState::Disconnected;
        app_handle.emit_app_state(&app_state);

        println!("Vibin WebSocket reader has completed");

        Ok(())
    }

    pub async fn start(
        &mut self,
        vibin_server: &str,
        stop_flag: &Arc<Mutex<bool>>,
        app_state_mutex: &AppStateMutex,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
    ) {
        println!("WebSocketConnection::start has been called");
        self.vibin_server = vibin_server.to_owned();

        self.stop_flag = Some(stop_flag.clone());
        *self.stop_flag.as_ref().unwrap().lock().unwrap() = false;

        loop {
            match self
                .handle_websocket(app_state_mutex, vibin_state_mutex, app_handle.clone())
                .await
            {
                Ok(_) => {
                    println!("WebSocketConnection handle_websocket90 has eneded");
                    app_state_mutex.lock().unwrap().set_disconnected();
                    *self.stop_flag.as_ref().unwrap().lock().unwrap() = false;

                    // println!("WebSocket Manager will attempt reconnect in 5 seconds");
                    // tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                    break;
                }
                Err(e) => match e {
                    tungstenite::Error::ConnectionClosed | tungstenite::Error::Io(_) => {
                        println!("WebSocket manager error: {:?}", e);
                        app_state_mutex.lock().unwrap().set_disconnected();

                        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                    }
                    _ => {
                        println!("Unhandled WebSocket manager error: {:?}", e);
                        return;
                    }
                },
            }
        }

        println!("WebSocketConnection start() has completed");
    }
}
