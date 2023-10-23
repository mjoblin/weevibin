use std::fmt;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

// These represent information to be sent to the front-end via message channels.

#[derive(Debug)]
pub enum Message {
    AppState,
    VibinState,
    Error,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

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
    pub id: String,
    pub name: String,
    pub default_name: String,
    pub class: String,
    pub nameable: bool,
    pub ui_selectable: bool,
    pub description: String,
    pub description_locale: String,
    pub preferred_order: isize,
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

#[derive(Clone, Serialize)]
pub struct VibinState {
    pub power: Option<String>,
    pub amplifier: Option<Amplifier>,
    pub display: StreamerDisplay,
    pub transport: Option<TransportState>,
    pub source: Option<Source>,
}

impl VibinState {
    pub fn new() -> VibinState {
        VibinState {
            power: Some("off".into()),
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
        }
    }
}

pub type VibinStateMutex = Arc<Mutex<VibinState>>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum VibinConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppState {
    pub vibin_connection: VibinConnectionState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            vibin_connection: VibinConnectionState::Disconnected,
        }
    }

    pub fn set_disconnected(&mut self) {
        self.vibin_connection = VibinConnectionState::Disconnected;
    }
}

pub type AppStateMutex = Arc<Mutex<AppState>>;
