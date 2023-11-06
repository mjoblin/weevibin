import { derived, writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

import type {
    ActiveTrack,
    Amplifier,
    Position,
    Power,
    Source,
    SourceClass,
    StreamerDisplay,
    Transport
} from "./vibin_types.ts";
import {
    type VibinHostDetails,
    getPersistedVibinHostDetails,
    setPersistedVibinHaveConnected,
    setPersistedVibinHost,
} from "./persisted_state.ts";

// UI application screens
type Screen = "main" | "settings";

// State of the Rust WebSocket connection to the Vibin backend
export type ConnectionStatus = "Connected" | "Connecting" | "Disconnected" | "Disconnecting";

// weevibin application state
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

// All known state information from the Vibin backend
export type VibinState = {
    power?: Power,
    streamer_power?: Power,
    amplifier?: Amplifier,
    display: StreamerDisplay,
    transport?: Transport,
    source?: Source,
    active_track?: ActiveTrack,
}

// ------------------------------------------------------------------------------------------------
// Exported Svelte state
// ------------------------------------------------------------------------------------------------

const DEFAULT_VIBIN_STATE = { display: {} };

export let currentScreen = writable<Screen>("main");

export let uiInitialized = writable<boolean>(false);

export let appErrorState = writable<AppError>();

export let appState = writable<AppState>({ vibin_connection: { state: "Disconnected" } });

export let vibinState = writable<VibinState>(DEFAULT_VIBIN_STATE);

export let playheadPosition = writable<number | null>(null);

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

// "System Power" represents both the streamer (always present) and the amplifier (optionally
// present). The system is on when the streamer is on and the amplifier (if present) is on.
// "Streamer Power" represents just the streamer (always present). This allows the UI to detect
// when the streamer is on while the (optional) amplifier is off.
export const isSystemPowerOn = derived(vibinState, ($vibinState) => $vibinState.power === "on");
export const isStreamerPowerOn = derived(vibinState, ($vibinState) => $vibinState.streamer_power === "on");

export const isConnected = derived(appState, ($appState) => $appState.vibin_connection.state === "Connected");

export const isPlaying = derived(vibinState, ($vibinState) => $vibinState.transport?.play_state === "play");

// ------------------------------------------------------------------------------------------------

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
