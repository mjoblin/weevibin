import { derived, writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

// These are the TypeScript equivalents of the Rust structs defined in src-tauri/src/state.rs

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

type SourceClass =
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

export let vibinState = writable<VibinState>({ display: {} });

export const isPlaying = derived(vibinState, ($vibinState) => $vibinState.transport?.play_state === "play");

export let playheadPosition = writable<number | null>(null);

const initialize = async () => {
    // await listen("AppState", (message) => console.log("AppState", message.payload));

    await listen<VibinState>("VibinState", (message) => {
        // console.log("VibinState", message.payload);
        vibinState.set(message.payload);
    });

    await listen<Position>("Position", (message) => {
        playheadPosition.set(message.payload.position);
    });

    await invoke("on_ui_ready");
}

await initialize();
