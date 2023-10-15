use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

// These represent information to be sent to the front-end via message channels.

#[derive(Clone, Serialize, Deserialize)]
pub struct StreamerDisplay {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub format: Option<String>,
    pub playback_source: Option<String>,
    pub art_url: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct Amplifier {
    pub mute: Option<String>,
    pub volume: Option<f32>,
}

#[derive(Clone, Serialize)]
pub struct VibinState {
    pub power: Option<String>,
    pub amplifier: Option<Amplifier>,
    pub display: StreamerDisplay,
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
