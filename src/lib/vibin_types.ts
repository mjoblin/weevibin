// Types specific to the Vibin back-end.
// These types closely match the Rust structs defined in src-tauri/src/state.rs

export type Power = "on" | "off";

export type PlayStatus =
    | "buffering"
    | "connecting"
    | "no_signal"
    | "not_ready"
    | "pause"
    | "play"
    | "ready"
    | "stop";

export type TransportAction =
    | "next"
    | "pause"
    | "play"
    | "previous"
    | "repeat"
    | "seek"
    | "shuffle"
    | "stop"
    | "toggle_playback";

export type RepeatState = "off" | "all";

export type ShuffleState = "off" | "all";

export type SourceClass =
    "digital.coax" |
    "digital.toslink" |
    "digital.usb" |
    "stream.media" |
    "stream.radio" |
    "stream.service.airplay" |
    "stream.service.cast" |
    "stream.service.roon" |
    "stream.service.spotify" |
    "stream.service.tidal";

export type Source = {
    id: string;
    name: string;
    default_name: string;
    class: SourceClass;
    nameable: boolean;
    ui_selectable: boolean;
    description: string;
    description_locale: string;
    preferred_order: number;
};

export type ActiveTrack = {
    title: string;
    artist: string;
    album: string;
    art_url: string;
    duration: number;
}

export type Amplifier = {
    mute?: Power,
    volume?: number,
}

export type StreamerDisplay = {
    line1?: string,
    line2?: string,
    line3?: string,
    format?: string,
    playback_source?: string,
    art_url?: string,
}

export type Transport = {
    play_state: PlayStatus,
    active_controls: TransportAction[],
    repeat: RepeatState,
    shuffle: ShuffleState,
}

export type Position = {
    position: number,
}
