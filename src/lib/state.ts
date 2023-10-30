import { derived, writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

import {
    type VibinHostDetails,
    getPersistedVibinHostDetails,
    setPersistedVibinHaveConnected,
    setPersistedVibinHost,
} from "./persisted_state.ts";

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

export let uiInitialized = writable<boolean>(false);

export let currentScreen = writable<Screen>("main");

export let appErrorState = writable<AppError>();

export let appState = writable<AppState>({ vibin_connection: { state: "Disconnected" } });

export let vibinState = writable<VibinState>(DEFAULT_VIBIN_STATE);

/**
 * Create a Svelte writable which wraps the persisted Vibin host details.
 */
async function createVibinHostState() {
    const { subscribe, set } = writable<VibinHostDetails>(await getPersistedVibinHostDetails());

    return {
        subscribe,
        setHostName: async (host: string) => {
            await setPersistedVibinHost(host)
            set(await getPersistedVibinHostDetails());
        },
        setHaveConnected: async (haveConnected?: boolean) => {
            await setPersistedVibinHaveConnected(haveConnected)
            set(await getPersistedVibinHostDetails());
        },
    };
}

export const vibinHost = await createVibinHostState();

export const isPowerOn = derived(vibinState, ($vibinState) => $vibinState.power === "on");

export const isConnected = derived(appState, ($appState) => $appState.vibin_connection.state === "Connected");

export const isPlaying = derived(vibinState, ($vibinState) => $vibinState.transport?.play_state === "play");

export let playheadPosition = writable<number | null>(null);

// Track the last seen audio source
let lastSeenSourceClass: SourceClass | undefined = undefined;

/**
 * Initialize the state-related components of the application.
 *
 *   - Set up listeners to receive the various message types emitted from Rust. These message
 *     payloads are used to populate Svelte state.
 *   - Invoke Rust's on_ui_ready command when the UI is ready to receive messages.
 */
const initialize = async () => {
    await listen<AppState>("AppState", (message) => {
        appState.set(message.payload);
        playheadPosition.set(null);

        if (message.payload.vibin_connection.state === "Connected") {
            vibinHost.setHaveConnected();
        } else {
            // When we're not connected to the Vibin WebSocket server, we want to reset all the
            // Vibin state to ensure the UI enters a "no information known" state.
            vibinState.set(DEFAULT_VIBIN_STATE);

            // If we're disconnected from the Vibin WebSocket server with an error message,
            // then we want to persist the fact that we haven't connected successfully. This
            // can be used to drive how the UI behaves on startup/etc.
            const connInfo = message.payload.vibin_connection;

            if (connInfo.state === "Disconnected" && connInfo.message) {
                vibinHost.setHaveConnected(false);
            }
        }
    });

    await listen<VibinState>("VibinState", (message) => {
        vibinState.set(message.payload);

        if (message.payload.source?.class !== lastSeenSourceClass) {
            // When the last seen audio source changes, the playhead position from the previous
            // source is no longer valid.
            lastSeenSourceClass = message.payload.source?.class;
            playheadPosition.set(null);
        }
    });

    await listen<Position>("Position", (message) => {
        playheadPosition.set(message.payload.position);
    });

    await listen<AppError>("Error", (message) => {
        appErrorState.set(message.payload);
    });

    // Inform Rust that the UI is ready. This means that the message listeners are all primed.
    await invoke("on_ui_ready");

    // Inform the rest of the UI that the UI is initialized. This will enable follow-on actions
    // such as connecting to the WebSocket server.
    uiInitialized.set(true);
}

await initialize();
