use std::io;
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
    ActiveTrack,
    Amplifier,
    AppState,
    AppStateMutex,
    Message,
    Position,
    StreamerDisplay,
    StreamerSources,
    TransportState,
    VibinConnectionState::{Connected, Connecting, Disconnected, Disconnecting},
    VibinStateMutex,
};

// TODO: Handle WebSocket connection errors for bubbling back to the UI, e.g.:
//   Io(Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" })
// TODO: Handle initial state (e.g. ensure full transport state is known at startup, before next
//  track starts.

// These structs represent messages received from the Vibin WebSocket server. The weevibin
// WebSocket client waits for various Vibin messages types (System, TransportState, Position),
// and uses their contents to update the shared VibinState struct -- which is then sent to the
// UI for display.

#[derive(Deserialize)]
struct VibinMessage {
    // id: String,
    // client_id: String,
    // time: isize,
    #[serde(rename = "type")]
    msg_type: String,
    payload: serde_json::Value,
}

// ------------------------------------------------------------------------------------------------
// System message

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

// ------------------------------------------------------------------------------------------------
// TransportState message

#[derive(Serialize, Deserialize)]
struct TransportStatePayload {
    pub play_state: Option<String>,
    pub active_controls: Vec<String>,
    pub repeat: Option<String>,
    pub shuffle: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Position message

#[derive(Serialize, Deserialize)]
struct PositionPayload {
    pub position: isize,
}

// ------------------------------------------------------------------------------------------------
// CurrentlyPlaying message

#[derive(Serialize, Deserialize)]
struct StreamWS {
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct FormatWS {
    pub sample_format: Option<String>,
    pub mqa: Option<String>,
    pub codec: Option<String>,
    pub lossless: Option<bool>,
    pub sample_rate: Option<isize>,
    pub bit_depth: Option<isize>,
    pub encoding: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ActiveTrackWS {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub art_url: Option<String>,
    pub duration: Option<isize>,
}

#[derive(Serialize, Deserialize)]
struct CurrentlyPlayingPayload {
    pub album_media_id: Option<String>,
    pub track_media_id: Option<String>,
    pub active_track: ActiveTrackWS,
    pub format: FormatWS,
    pub stream: StreamWS,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize)]
pub enum AppErrorCategory {
    WebSocket,
}

#[derive(Clone, Serialize)]
pub struct AppError {
    pub category: AppErrorCategory,
    pub message: String,
}

// ------------------------------------------------------------------------------------------------

trait CustomEmitters {
    fn emit_app_state(&self, app_state: &AppState);
    fn emit_websocket_error(&self, error_message: &str);
}

impl CustomEmitters for AppHandle {
    fn emit_app_state(&self, app_state: &AppState) {
        self.emit_all(&Message::AppState.to_string(), app_state).unwrap();
        println!("SENT: {:?}", app_state);
    }

    fn emit_websocket_error(&self, error_message: &str) {
        self.emit_all(
            &Message::Error.to_string(),
            AppError {
                category: AppErrorCategory::WebSocket,
                message: error_message.into(),
            },
        )
        .unwrap();
    }
}

// ------------------------------------------------------------------------------------------------

enum VibinWebSocketError {
    WebSocketError(tungstenite::Error),
    CustomError(String),
}

#[derive(Clone)]
pub struct WebSocketManager {
    pub connection: Arc<TauriMutex<WebSocketConnection>>,
    pub vibin_host: Option<Box<String>>,
    pub stop_flag: Arc<Mutex<bool>>,
    pub app_state_mutex: AppStateMutex,
    pub vibin_state_mutex: VibinStateMutex,
    pub app_handle: AppHandle,

    pub is_started: Arc<Mutex<bool>>,
}

impl WebSocketManager {
    pub fn new(
        connection: Arc<TauriMutex<WebSocketConnection>>,
        vibin_host: Option<Box<String>>,
        stop_flag: Arc<Mutex<bool>>,
        app_state_mutex: AppStateMutex,
        vibin_state_mutex: VibinStateMutex,
        app_handle: AppHandle,
    ) -> Self {
        WebSocketManager {
            connection,
            vibin_host,
            stop_flag,
            app_state_mutex,
            vibin_state_mutex,
            app_handle,

            is_started: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self) {
        if *self.is_started.lock().unwrap() == true {
            println!("WebSocketManager is already started; ignoring start request");
            return;
        }

        if self.vibin_host.is_none() {
            println!("WebSocketManager not starting; no vibin host specified");
            return;
        }

        *self.is_started.lock().unwrap() = true;

        println!("WebSocketManager has been started");
        let mut self_clone = self.clone();
        *self.stop_flag.lock().unwrap() = false;

        tauri::async_runtime::spawn(async move {
            println!("WebSocketManager is starting the WebSocketConnection");
            self_clone
                .connection
                .lock()
                .await
                .start(
                    &self_clone.vibin_host.unwrap().clone(),
                    &self_clone.stop_flag.clone(),
                    &self_clone.app_state_mutex,
                    &self_clone.vibin_state_mutex,
                    self_clone.app_handle.clone(),
                )
                .await;

            println!("WebSocketManager start() of WebSocketConnection has completed");

            // self_clone.app_state_mutex.lock().unwrap().vibin_connection = Disconnected(None);
            *self_clone.is_started.lock().unwrap() = false;
            self_clone.app_handle.emit_app_state(&self_clone.app_state_mutex.lock().unwrap());
        });

        println!("End of start function");
    }

    pub async fn stop(&mut self) {
        match self.app_state_mutex.lock().unwrap().vibin_connection {
            Connected(_) => {},
            _ => {
                println!("WebSocketManager not connected; ignoring stop() request");
                return;
            }
        }

        println!("WebSocketManager requesting WebSocketConnection disconnect");

        self.app_state_mutex.lock().unwrap().vibin_connection = Disconnecting;
        self.app_handle.emit_app_state(&self.app_state_mutex.lock().unwrap());
        *self.stop_flag.lock().unwrap() = true;

        println!("WebSocketManager waiting for disconnect");

        let mut is_disconnected = false;

        while !is_disconnected {
            let connection_state = self.app_state_mutex.lock().unwrap().vibin_connection.clone();
            match connection_state {
                Disconnected(_) => is_disconnected = true,
                _ => time::sleep(time::Duration::from_millis(100)).await,
            }
        }

        println!("WebSocketManager has detected WebSocketConnection disconnect");

        self.app_state_mutex.lock().unwrap().vibin_connection = Disconnected(None);
        *self.is_started.lock().unwrap() = false;
    }
}

unsafe impl Send for WebSocketManager {}

pub type WebSocketManagerMutex = Arc<TauriMutex<WebSocketManager>>;

pub struct WebSocketConnection {
    pub stop_flag: Option<Arc<Mutex<bool>>>,
    pub vibin_host: String,
}

unsafe impl Send for WebSocketConnection {}

impl WebSocketConnection {
    fn process_message(
        &self,
        vibin_msg: VibinMessage,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
    ) {
        let mut vibin_state = vibin_state_mutex.lock().unwrap();
        let mut send_update_to_client = false;

        match vibin_msg.msg_type.as_str() {
            "System" => {
                let system_payload: SystemPayload =
                    serde_json::from_value(vibin_msg.payload).unwrap();

                vibin_state.power = system_payload.power;

                if let Some(amplifier) = system_payload.amplifier {
                    vibin_state.amplifier = Some(Amplifier {
                        mute: amplifier.mute,
                        volume: amplifier.volume,
                    });
                }

                if let Some(display) = system_payload.streamer.display {
                    vibin_state.display.line1 = display.line1;
                    vibin_state.display.line2 = display.line2;
                    vibin_state.display.line3 = display.line3;
                    vibin_state.display.format = display.format;
                    vibin_state.display.playback_source = display.playback_source;
                    vibin_state.display.art_url = display.art_url;
                }

                if let Some(sources) = system_payload.streamer.sources {
                    vibin_state.source = Some(sources.active);
                }

                send_update_to_client = true;
            }
            "TransportState" => {
                let transport_payload: TransportStatePayload =
                    serde_json::from_value(vibin_msg.payload).unwrap();

                vibin_state.transport = Some(TransportState {
                    play_state: transport_payload.play_state,
                    active_controls: transport_payload.active_controls,
                    repeat: transport_payload.repeat,
                    shuffle: transport_payload.shuffle,
                });

                send_update_to_client = true;
            }
            "CurrentlyPlaying" => {
                let currently_playing: CurrentlyPlayingPayload =
                    serde_json::from_value(vibin_msg.payload).unwrap();

                vibin_state.active_track = Some(ActiveTrack {
                    title: currently_playing.active_track.title,
                    artist: currently_playing.active_track.artist,
                    album: currently_playing.active_track.album,
                    art_url: currently_playing.active_track.art_url,
                    duration: currently_playing.active_track.duration,
                });

                send_update_to_client = true;
            }
            "Position" => {
                let position_payload: PositionPayload =
                    serde_json::from_value(vibin_msg.payload).unwrap();

                app_handle
                    .emit_all(&Message::Position.to_string(), Position {
                        position: position_payload.position,
                    })
                    .unwrap();
            }
            _ => {}
        }

        if send_update_to_client {
            app_handle
                .emit_all(&Message::VibinState.to_string(), &(*vibin_state))
                .unwrap();
        }
    }

    async fn handle_websocket(
        &self,
        app_state_mutex: &AppStateMutex,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
    ) -> Result<(), VibinWebSocketError> {
        {
            // NOTE: app_state_mutex is locked inside a block to ensure the lock is released before
            //  the subsequent .await call. Ref: https://tokio.rs/tokio/tutorial/shared-state
            let mut app_state = app_state_mutex.lock().unwrap();

            match app_state.vibin_connection {
                Disconnected(_) => {},
                _ => {
                    let err =
                        "Connection state is not Disconnected; not proceeding with Vibin WebSocket connection";

                    println!("{}", &err);
                    app_handle.emit_websocket_error(&err);

                    return Ok(());
                },
            }

            // if app_state.vibin_connection != Disconnected {
            //     let err = format!(
            //         "Connection state is not {:?}; not proceeding with Vibin WebSocket connection",
            //         Disconnected
            //     );
            //
            //     println!("{}", &err);
            //     app_handle.emit_websocket_error(&err);
            //
            //     return Ok(());
            // }
        }

        // let url = url::Url::parse("ws://192.168.2.101:8080/ws").unwrap();
        // let url = url::Url::parse(self.vibin_host.as_str()).unwrap();

        let url = match url::Url::parse(self.vibin_host.as_str()) {
            Ok(url) => url,
            Err(e) => {
                app_handle.emit_websocket_error(&format!("Vibin host URL parsing error: {:?}", e));
                return Ok(());
            }
        };

        {
            let mut app_state = app_state_mutex.lock().unwrap();
            app_state.vibin_connection = Connecting(self.vibin_host.clone());
            app_handle.emit_app_state(&app_state);
        }

        let connect_timeout = tokio::time::Duration::from_secs(5);
        let connect_attempt = connect_async(&url);

        let (ws_stream, _) = match tokio::time::timeout(connect_timeout, connect_attempt).await {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => {
                let error_message = match e {
                    tungstenite::Error::Io(e) => format!("{}", e.to_string().replace(r#"\""#, "")),
                    _ => format!("{:?}", e),
                };

                let error = format!("Connection error: {:?}", error_message);
                app_handle.emit_websocket_error(&error);
                return Err(VibinWebSocketError::CustomError(error));
            }
            Err(e) => {
                // Timeout
                let error = format!("Timed out connecting to: {url}");
                app_handle.emit_websocket_error(&error);
                return Err(VibinWebSocketError::CustomError(error));
            }
        };

        {
            let mut app_state = app_state_mutex.lock().unwrap();

            println!(
                "Connected to Vibin WebSocket server: {:?}",
                self.vibin_host
            );
            app_state.vibin_connection = Connected(self.vibin_host.clone());
            app_handle.emit_app_state(&app_state);
        }

        // This is a read-only websocket connection, so we ignore the write stream.
        let (_, read) = ws_stream.split();

        // TODO: Look into controlling stop_reading()'s poll delay
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
                            Err(e) => app_handle.emit_websocket_error(&format!(
                                "Could not deserialize WebSocket message; error: {:?} :: message: {}",
                                e, message_text
                            )),
                        },
                        Err(e) => {
                            println!("MESSAGE TAKE ERROR: {:?}", e);
                            app_handle.emit_websocket_error(&format!(
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
        app_state.vibin_connection = Disconnected(None);
        app_handle.emit_app_state(&app_state);

        println!("Vibin WebSocket reader has completed");

        Ok(())
    }

    pub async fn start(
        &mut self,
        vibin_host: &str,
        stop_flag: &Arc<Mutex<bool>>,
        app_state_mutex: &AppStateMutex,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
    ) {
        println!("WebSocketConnection::start has been called");
        self.vibin_host = vibin_host.to_owned();

        self.stop_flag = Some(stop_flag.clone());
        *self.stop_flag.as_ref().unwrap().lock().unwrap() = false;

        loop {
            match self
                .handle_websocket(app_state_mutex, vibin_state_mutex, app_handle.clone())
                .await
            {
                Ok(_) => {
                    println!("WebSocketConnection handle_websocket() has ended");
                    app_state_mutex.lock().unwrap().set_disconnected(None);
                    *self.stop_flag.as_ref().unwrap().lock().unwrap() = false;

                    // println!("WebSocket Manager will attempt reconnect in 5 seconds");
                    // tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                    break;
                }
                Err(e) => match e {
                    VibinWebSocketError::WebSocketError(e) => match e {
                        tungstenite::Error::ConnectionClosed | tungstenite::Error::Io(_) => {
                            let prior_connection_state = app_state_mutex.lock().unwrap().vibin_connection.clone();

                            let error = format!("IO error: {:?}", e);
                            app_state_mutex.lock().unwrap().set_disconnected(Some(error.clone()));

                            println!("WebSocket manager error: {:?}", &error);
                            app_handle.emit_websocket_error(&error);

                            match prior_connection_state {
                                Connected(_) => {},
                                // If we're here then it's likely that we attempted to connect using a
                                // vibin host provided by the user, but the connection failed so we
                                // want to break out.
                                _ => break,
                            }

                            println!("Will attempt reconnect in 5 seconds");
                            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                        }
                        _ => {
                            let error = format!("Unknown error: {:?}", e);
                            app_state_mutex.lock().unwrap().set_disconnected(Some(error.clone()));
                            app_handle.emit_websocket_error(&error);

                            println!("Unhandled WebSocket manager error: {:?}", error);
                            break;
                        }
                    },
                    VibinWebSocketError::CustomError(e) => {
                        app_state_mutex.lock().unwrap().set_disconnected(Some(e.clone()));
                        app_handle.emit_websocket_error(&e);

                        println!("WebSocket manager error: {:?}", e);
                        break;
                    }
                },
            }
        }

        println!("WebSocketConnection start() has completed");
    }
}
