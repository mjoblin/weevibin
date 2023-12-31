use std::fmt;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

// ===============================================================================================
// These represent information to be sent to the front-end via message channels. It includes:
//
// AppState - The overall weevibin application state.
// VibinState - Information about the current Vibin state (current track, transport details, etc).
// Position - Current track position. This will be emitted frequently (likely once per second).
// Error - Any errors to be reported to the front-end.
// ===============================================================================================

// -----------------------------------------------------------------------------------------------
// Message types

#[derive(Debug)]
pub enum WeeVibinMessage {
    AppState,
    VibinState,
    Position,
    Error,
}

impl fmt::Display for WeeVibinMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

// -----------------------------------------------------------------------------------------------
// Weevibin application state

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum VibinConnectionState {
    Connected(String),
    Connecting(String),
    Disconnected(Option<String>),
    Disconnecting,
}

impl Serialize for VibinConnectionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            VibinConnectionState::Connected(message) => {
                let mut state = serializer.serialize_struct("VibinConnectionState", 2)?;
                state.serialize_field("state", "Connected")?;
                state.serialize_field("message", message)?;
                state.end()
            }
            VibinConnectionState::Connecting(message) => {
                let mut state = serializer.serialize_struct("VibinConnectionState", 2)?;
                state.serialize_field("state", "Connecting")?;
                state.serialize_field("message", message)?;
                state.end()
            },
            VibinConnectionState::Disconnected(message) => {
                let mut state = serializer.serialize_struct("VibinConnectionState", 2)?;
                state.serialize_field("state", "Disconnected")?;
                state.serialize_field("message", message)?;
                state.end()
            },
            VibinConnectionState::Disconnecting => {
                let mut state = serializer.serialize_struct("VibinConnectionState", 2)?;
                state.serialize_field("state", "Disconnecting")?;
                state.serialize_field("message", "")?;
                state.end()
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppState {
    pub vibin_connection: VibinConnectionState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            vibin_connection: VibinConnectionState::Disconnected(None),
        }
    }

    pub fn set_disconnected(&mut self, message: Option<String>) {
        self.vibin_connection = VibinConnectionState::Disconnected(message);
    }
}

pub type AppStateMutex = Arc<Mutex<AppState>>;

// -----------------------------------------------------------------------------------------------
// VibinState
//
// This state collects all the Vibin details into one struct (VibinState). It will be populated
// from various WebSocket message types received by the Rust side of weevibin. The idea is to
// collect all the disparate vibin information into a single struct, which is then sent to the
// UI code (over a message channel) for display.

#[derive(Clone, Serialize, Deserialize)]
pub struct StreamerDisplay {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub format: Option<String>,
    pub playback_source: Option<String>,
    pub art_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<String>,
    pub name: Option<String>,
    pub default_name: Option<String>,
    pub class: Option<String>,
    pub nameable: Option<bool>,
    pub ui_selectable: Option<bool>,
    pub description: Option<String>,
    pub description_locale: Option<String>,
    pub preferred_order: Option<isize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StreamerSources {
    pub active: Source,
    pub available: Vec<Source>,
}

#[derive(Clone, Serialize)]
pub struct Amplifier {
    pub mute: Option<String>,
    pub volume: Option<f32>,
}

// TODO: Consider adding proper types for play_state, active_controls, etc. Keeping them as
//  strings is arguably too loose as they are actually well-defined strings (enums).
#[derive(Clone, Serialize, Deserialize)]
pub struct TransportState {
    pub play_state: Option<String>,
    pub active_controls: Vec<String>,
    pub repeat: Option<String>,
    pub shuffle: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActiveTrack {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub art_url: Option<String>,
    pub duration: Option<isize>,
}

#[derive(Clone, Serialize)]
pub struct VibinState {
    pub power: Option<String>,
    pub streamer_power: Option<String>,
    pub amplifier: Option<Amplifier>,
    pub display: StreamerDisplay,
    pub transport: Option<TransportState>,
    pub source: Option<Source>,
    pub active_track: Option<ActiveTrack>,
}

impl VibinState {
    pub fn new() -> VibinState {
        VibinState {
            power: Some("off".into()),
            streamer_power: Some("off".into()),
            amplifier: None,
            display: StreamerDisplay {
                line1: None,
                line2: None,
                line3: None,
                format: None,
                playback_source: None,
                art_url: None,
            },
            transport: None,
            source: None,
            active_track: None,
        }
    }
}

pub type VibinStateMutex = Arc<Mutex<VibinState>>;

// -----------------------------------------------------------------------------------------------
// Position

#[derive(Clone, Serialize)]
pub struct Position {
    pub position: isize,
}

