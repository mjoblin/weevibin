use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use futures_util::StreamExt;
use log::{info, warn, error};
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex as TauriMutex;
use tauri::{AppHandle, Manager};
use tokio;
use tokio::time::{Duration, sleep, timeout};
use tokio_tungstenite::connect_async;
use tungstenite;

use crate::average::RunningAverage;
use crate::state::{
    ActiveTrack,
    Amplifier,
    AppState,
    AppStateMutex,
    Position,
    StreamerDisplay,
    StreamerSources,
    TransportState,
    VibinConnectionState::{Connected, Connecting, Disconnected, Disconnecting},
    VibinStateMutex,
    WeeVibinMessage,
};

// ------------------------------------------------------------------------------------------------
// TODO: Fix overall architecture. The current implementation is a bit of a mess. There's a
//  WebSocketManager and WebSocketConnection, which might be OK conceptually -- but they both
//  expect to be able to directly access the application state and vibin state, and it's not clear
//  what owns what (like connection status). Error handling is also unclear, including the purpose
//  of VibinWebSocketError. There's also a lot of Arc<Mutex>> and cloning going on, which got
//  things working but requires a considered analysis. Also consider properly handling the
//  remaining unwrap() calls.
// ------------------------------------------------------------------------------------------------
//
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

// TODO: Rename these structs to address what the "WS" prefix is addressing.

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
        self.emit_all(&WeeVibinMessage::AppState.to_string(), app_state).unwrap();
    }

    fn emit_websocket_error(&self, error_message: &str) {
        self.emit_all(
            &WeeVibinMessage::Error.to_string(),
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
    ClientLostConnectionError,
    ServerClosedConnectionError,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct WebSocketManager {
    pub vibin_host: Option<Box<String>>,
    pub stop_flag: Arc<Mutex<bool>>,
    pub app_state_mutex: AppStateMutex,
    pub vibin_state_mutex: VibinStateMutex,
    pub app_handle: AppHandle,

    pub connection: Arc<TauriMutex<WebSocketConnection>>,
    pub is_started: Arc<Mutex<bool>>,
    pub have_connected: Arc<Mutex<bool>>,
}

impl WebSocketManager {
    pub fn new(
        vibin_host: Option<Box<String>>,
        stop_flag: Arc<Mutex<bool>>,
        app_state_mutex: AppStateMutex,
        vibin_state_mutex: VibinStateMutex,
        app_handle: AppHandle,
    ) -> Self {
        WebSocketManager {
            vibin_host,
            stop_flag,
            app_state_mutex,
            vibin_state_mutex,
            app_handle,

            connection: Arc::new(TauriMutex::new(WebSocketConnection {
                stop_flag: None,
                vibin_host: String::from(""),
            })),
            is_started: Arc::new(Mutex::new(false)),
            have_connected: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_have_connected(&mut self, have_connected: bool) {
        *self.have_connected.lock().unwrap() = have_connected;
    }

    pub fn start(&mut self) {
        if *self.is_started.lock().unwrap() == true {
            warn!("WebSocketManager is already started; ignoring start request");
            return;
        }

        if self.vibin_host.is_none() {
            warn!("WebSocketManager not starting; no vibin host specified");
            return;
        }

        *self.is_started.lock().unwrap() = true;
        *self.have_connected.lock().unwrap() = false;

        info!("WebSocketManager is starting for: {}", match &self.vibin_host {
            Some(inner) => *inner.clone(),
            None => "unknown".to_string(),
        });

        let self_clone = self.clone();

        // TODO: This clone of the manager for passing to the connection feels super unfortunate.
        //  It's only done so that the connection can set the "have_connected" field on the manager
        //  on a successful connection. There must be a better way for the manager to be made aware
        //  of the successful connection.
        let self_clone_for_connection = self.clone();

        *self.stop_flag.lock().unwrap() = false;

        tauri::async_runtime::spawn(async move {
            info!("WebSocketManager is starting the WebSocketConnection");
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
                    self_clone_for_connection,
                )
                .await;

            info!("WebSocketManager start() of WebSocketConnection has completed");

            // self_clone.app_state_mutex.lock().unwrap().vibin_connection = Disconnected(None);
            *self_clone.is_started.lock().unwrap() = false;
            self_clone.app_handle.emit_app_state(&self_clone.app_state_mutex.lock().unwrap());
        });
    }

    pub async fn stop(&mut self) {
        match self.app_state_mutex.lock().unwrap().vibin_connection {
            Connected(_) => {},
            _ => {
                warn!("WebSocketManager not connected; ignoring stop() request");
                return;
            }
        }

        info!("WebSocketManager requesting WebSocketConnection disconnect");

        self.app_state_mutex.lock().unwrap().vibin_connection = Disconnecting;
        self.app_handle.emit_app_state(&self.app_state_mutex.lock().unwrap());
        *self.stop_flag.lock().unwrap() = true;

        info!("WebSocketManager waiting for disconnect");

        let mut is_disconnected = false;

        while !is_disconnected {
            let connection_state = self.app_state_mutex.lock().unwrap().vibin_connection.clone();
            match connection_state {
                Disconnected(_) => is_disconnected = true,
                _ => sleep(Duration::from_millis(100)).await,
            }
        }

        info!("WebSocketManager has detected WebSocketConnection disconnect");

        self.app_state_mutex.lock().unwrap().vibin_connection = Disconnected(None);
        *self.is_started.lock().unwrap() = false;
        *self.have_connected.lock().unwrap() = false;
    }
}

unsafe impl Send for WebSocketManager {}

pub type WebSocketManagerMutex = Arc<TauriMutex<WebSocketManager>>;

// ------------------------------------------------------------------------------------------------

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
                    .emit_all(&WeeVibinMessage::Position.to_string(), Position {
                        position: position_payload.position,
                    })
                    .unwrap();
            }
            _ => {}
        }

        if send_update_to_client {
            app_handle
                .emit_all(&WeeVibinMessage::VibinState.to_string(), &(*vibin_state))
                .unwrap();
        }
    }

    async fn handle_websocket(
        &self,
        app_state_mutex: &AppStateMutex,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
        manager: WebSocketManager,
    ) -> Result<(), VibinWebSocketError> {
        // Don't attempt to connect if we're not Disconnected.
        {
            // NOTE: app_state_mutex is locked inside a block to ensure the lock is released before
            //  the subsequent .await call. Ref: https://tokio.rs/tokio/tutorial/shared-state
            let app_state = app_state_mutex.lock().unwrap();

            match app_state.vibin_connection {
                Disconnected(_) => {},
                _ => {
                    let err =
                        "Connection state is not Disconnected; not proceeding with Vibin WebSocket connection";

                    error!("{}", &err);
                    app_handle.emit_websocket_error(&err);

                    return Ok(());
                },
            }
        }

        // Initiate connection to WebSocket server.
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

        // Detect connection attempt timeouts.
        let connect_timeout = Duration::from_secs(5);
        let connect_attempt = connect_async(&url);

        let (ws_stream, _) = match timeout(connect_timeout, connect_attempt).await {
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
            Err(_) => {
                // Timeout
                let error = format!("Timed out connecting to: {url}");
                app_handle.emit_websocket_error(&error);
                return Err(VibinWebSocketError::CustomError(error));
            }
        };

        // Announce the connection.
        {
            let mut app_state = app_state_mutex.lock().unwrap();

            info!("Connected to Vibin WebSocket server: {:?}",self.vibin_host);
            app_state.vibin_connection = Connected(self.vibin_host.clone());
            app_handle.emit_app_state(&app_state);
        }

        *manager.have_connected.lock().unwrap() = true;

        // This is a read-only websocket connection, so we ignore the write stream.
        let (_, mut read) = ws_stream.split();

        // Read messages forever; but check at regular intervals to see if the stop_flag is set or
        // whether the client seems to have lost its connection.
        let mut interval = tokio::time::interval(Duration::from_secs(2));

        // Track WebSocket server ping times. This is done to establish when the client may have
        // lost the connection (perhaps due to going to sleep). This is different from the server
        // explicitly closing the connection (see tungstenite::Message::Close). The average ping
        // duration is tracked, and if enough time has passed since the last ping then we assume
        // the connection is lost. We wait for at least 2 pings to come in before calculating the
        // average ping delay.
        let mut ping_avg = RunningAverage::new(10);
        let mut last_ping_time = SystemTime::now();
        let mut have_ignored_first_ping = false;
        const START_PING_THRESHOLD_SECS: u64 = 60;  // For use until average ping time is known
        const PING_DURATION_BUFFER_FACTOR: f64 = 1.25;  // 125% is considered too long to wait

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // Do some checks every interval, regardless of incoming messages.

                    // Check if we've been told to stop listening.
                    if *self.stop_flag.as_ref().unwrap().lock().unwrap() == true {
                        info!("WebSocket handler stop_flag detected");
                        break;
                    }

                    // Check if it's been too long since we last received a server ping.
                    match SystemTime::now().duration_since(last_ping_time) {
                        Ok(duration) => {
                            let ping_duration_check = match ping_avg.len() {
                                len if len > 2 => ping_avg.average() * PING_DURATION_BUFFER_FACTOR,
                                _ => START_PING_THRESHOLD_SECS as f64,
                            };

                            if duration.as_secs() as f64 > ping_duration_check {
                                warn!("WebSocket ping not received for {ping_duration_check} secs");

                                return Err(VibinWebSocketError::ClientLostConnectionError);
                            }
                        },
                        Err(e) => error!("WebSocket error determining last ping duration: {:?}", e),
                    }
                },
                Some(next_item) = read.next() => {
                    match next_item {
                        Ok(message) => {
                            match message {
                                tungstenite::Message::Ping(_) => {
                                    let now = SystemTime::now();

                                    // Keep track of how long we're waiting between pings. Ignore the first
                                    // ping because it might throw off the average wait time calculation.
                                    if have_ignored_first_ping {
                                        match now.duration_since(last_ping_time) {
                                            Ok(duration) => ping_avg.add(duration.as_secs() as f64),
                                            _ => {},
                                        };
                                    } else {
                                        have_ignored_first_ping = true;
                                    }

                                    last_ping_time = now;
                                },
                                tungstenite::Message::Close(_) => {
                                    // Explicit server connection close. This is distinct from the client
                                    // losing the connection for other reasons (which is detected by ping
                                    // time checks).
                                    warn!("WebSocket connection has been closed by Vibin");
                                    return Err(VibinWebSocketError::ServerClosedConnectionError);
                                },
                                tungstenite::Message::Text(message_text) => {
                                    // Incoming VibinMessage from WebSocket server.
                                    match serde_json::from_str::<VibinMessage>(&message_text) {
                                        Ok(vibin_msg) => {
                                            let app_handle = app_handle.clone();
                                            self.process_message(vibin_msg, vibin_state_mutex, app_handle);
                                        }
                                        Err(e) => app_handle.emit_websocket_error(&format!(
                                            "Could not deserialize WebSocket message; error: {:?} :: message: {}",
                                            e, message_text
                                        )),
                                    }
                                },
                                unexpected => {
                                    error!("Ignoring unexpected WebSocket message type: {:?}", unexpected);
                                },
                            }
                        },
                        Err(e) => return Err(VibinWebSocketError::WebSocketError(e)),
                    }
                }
            }
        }

        let mut app_state = app_state_mutex.lock().unwrap();
        app_state.vibin_connection = Disconnected(None);
        app_handle.emit_app_state(&app_state);

        info!("Vibin WebSocket reader has completed");

        Ok(())
    }

    pub async fn start(
        &mut self,
        vibin_host: &str,
        stop_flag: &Arc<Mutex<bool>>,
        app_state_mutex: &AppStateMutex,
        vibin_state_mutex: &VibinStateMutex,
        app_handle: AppHandle,
        manager: WebSocketManager,
    ) {
        info!("WebSocketConnection::start has been called: {vibin_host}");
        self.vibin_host = vibin_host.to_owned();

        self.stop_flag = Some(stop_flag.clone());
        *self.stop_flag.as_ref().unwrap().lock().unwrap() = false;

        const RETRY_DELAY_SECS: u64 = 5;

        loop {
            let manager_clone = manager.clone();

            match self
                .handle_websocket(app_state_mutex, vibin_state_mutex, app_handle.clone(), manager_clone)
                .await
            {
                Ok(_) => {
                    info!("WebSocketConnection handle_websocket() has ended successfully");
                    app_state_mutex.lock().unwrap().set_disconnected(None);
                    *self.stop_flag.as_ref().unwrap().lock().unwrap() = false;

                    // This is a successful exit from handle_websocket(), which means we either
                    // stopped reading messages (because we were asked to stop), or we can't connect
                    // in a way that shouldn't be retried, so we want to hard break out.
                    break;
                }
                Err(e) => match e {
                    VibinWebSocketError::WebSocketError(e) => match e {
                        tungstenite::Error::Io(_) => {
                            let error = format!("IO error: {:?}", e);
                            app_state_mutex.lock().unwrap().set_disconnected(Some(error.clone()));

                            error!("WebSocketManager error: {:?}", &error);
                            app_handle.emit_websocket_error(&error);
                        }
                        _ => {
                            let error = format!("Unknown error: {:?}", e);
                            app_state_mutex.lock().unwrap().set_disconnected(Some(error.clone()));
                            app_handle.emit_websocket_error(&error);

                            error!("Unhandled WebSocketManager error: {:?}", error);
                        }
                    },
                    VibinWebSocketError::CustomError(e) => {
                        app_state_mutex.lock().unwrap().set_disconnected(Some(e.clone()));
                        app_handle.emit_websocket_error(&e);

                        error!("WebSocketManager error: {:?}", e);
                    },
                    VibinWebSocketError::ClientLostConnectionError => {
                        let msg = String::from("Client lost connection to WebSocket server");
                        warn!("{msg}");

                        {
                            let mut app_state = app_state_mutex.lock().unwrap();
                            app_state.set_disconnected(Some(msg.clone()));
                            app_handle.emit_app_state(&app_state);
                        }

                        app_handle.emit_websocket_error(&msg);
                    },
                    VibinWebSocketError::ServerClosedConnectionError => {
                        let msg = String::from("WebSocket server closed the connection");
                        warn!("{msg}");

                        {
                            let mut app_state = app_state_mutex.lock().unwrap();
                            app_state.set_disconnected(Some(msg.clone()));
                            app_handle.emit_app_state(&app_state);
                        }

                        app_handle.emit_websocket_error(&msg);
                    }
                },
            }

            // If handle_connection() exited, but we had a previously-valid connection (i.e.
            // have_connected is true) then we want to retry the connection. This is likely to
            // happen if Vibin goes offline temporarily, or if the local machine is coming back
            // from sleep.
            if *manager.have_connected.lock().unwrap() == true {
                info!("Will attempt WebSocket reconnect in {RETRY_DELAY_SECS} seconds");
                sleep(Duration::from_secs(RETRY_DELAY_SECS)).await;
            } else {
                info!("Not attempting WebSocket reconnect");
                break;
            }
        }

        info!("WebSocketConnection start() has completed");
    }
}
