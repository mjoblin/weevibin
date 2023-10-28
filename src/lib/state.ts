import { derived, writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

// These are the TypeScript equivalents of the Rust structs defined in src-tauri/src/state.rs

export type ConnectionStatus = "Connected" | "Connecting" | "Disconnected" | "Disconnecting";

type AppState = {
    vibin_connection: {
        state: ConnectionStatus;
        message?: string;
    };
};

type AppErrorCategory = "WebSocket";

type AppError = {
    category: AppErrorCategory;
    message: string;
};

type Power = "on" | "off";

type PlayStatus =
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

type RepeatState = "off" | "all";

type ShuffleState = "off" | "all";

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

type Source = {
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

type ActiveTrack = {
    title: string;
    artist: string;
    album: string;
    art_url: string;
    duration: number;
}

type Amplifier = {
    mute?: Power,
    volume?: number,
}

type StreamerDisplay = {
    line1?: string,
    line2?: string,
    line3?: string,
    format?: string,
    playback_source?: string,
    art_url?: string,
}

type Transport = {
    play_state: PlayStatus,
    active_controls: TransportAction[],
    repeat: RepeatState,
    shuffle: ShuffleState,
}

type VibinState = {
    power?: Power,
    amplifier?: Amplifier,
    display: StreamerDisplay,
    transport?: Transport,
    source?: Source,
    active_track?: ActiveTrack,
}

type Position = {
    position: number,
}

type Screen = "main" | "settings";

const DEFAULT_VIBIN_STATE = { display: {} };

export let currentScreen = writable<Screen>("main");

export let appErrorState = writable<AppError>();

export let appState = writable<AppState>({ vibin_connection: { state: "Disconnected" } });

export let vibinState = writable<VibinState>(DEFAULT_VIBIN_STATE);

export const isPowerOn = derived(vibinState, ($vibinState) => $vibinState.power === "on");

export const isConnected = derived(appState, ($appState) => $appState.vibin_connection.state === "Connected");

export const isPlaying = derived(vibinState, ($vibinState) => $vibinState.transport?.play_state === "play");

export let playheadPosition = writable<number | null>(null);

let lastSourceClass: SourceClass | undefined = undefined;

const initialize = async () => {
    await listen<AppState>("AppState", (message) => {
        // console.log("AppState", message.payload);
        appState.set(message.payload);
        playheadPosition.set(null);

        if (message.payload.vibin_connection.state !== "Connected") {
            vibinState.set(DEFAULT_VIBIN_STATE);
        }
    });

    await listen<VibinState>("VibinState", (message) => {
        // console.log("VibinState", message.payload);
        vibinState.set(message.payload);

        if (message.payload.source?.class !== lastSourceClass) {
            lastSourceClass = message.payload.source?.class;
            playheadPosition.set(null);
        }
    });

    await listen<Position>("Position", (message) => {
        playheadPosition.set(message.payload.position);
    });

    await listen<AppError>("Error", (message) => {
        // console.log("Error", message.payload);
        appErrorState.set(message.payload);
    });

    await invoke("on_ui_ready");
}

await initialize();
