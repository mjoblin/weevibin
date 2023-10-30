import { Store } from "tauri-plugin-store-api";

// The Vibin WebSocket server details are persisted and re-used between sessions.
//
//   host - the Vibin host name
//   haveConnected - whether the last attempt to connect to `host` was successful

export type VibinHostDetails = {
    host: string;
    haveConnected: boolean;
}

const VIBIN_HOST_DETAILS_KEY = "vibin-host-details";
const VIBIN_HOST_DETAILS_DEFAULT: VibinHostDetails = {
    host: "vibin.local",
    haveConnected: false,
};

const persisted_store = new Store("weevibin.dat");

export const getPersistedVibinHostDetails = async (): Promise<VibinHostDetails> => {
    // NOTE: persisted_store.has(<key>) does not return falsey if the store file doesn't exist
    if (!await persisted_store.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY)) {
        await persisted_store.set(VIBIN_HOST_DETAILS_KEY, VIBIN_HOST_DETAILS_DEFAULT);
        await persisted_store.save();
    }

    const host_details = await persisted_store.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY);

    return host_details as VibinHostDetails;
}

export const setPersistedVibinHost = async (host: string) => {
    const state = await persisted_store.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY);

    if (state) {
        await persisted_store.set(VIBIN_HOST_DETAILS_KEY, { ...state, host });
        await persisted_store.save();
    }
}

export const setPersistedVibinHaveConnected = async (haveConnected?: boolean) => {
    const state = await persisted_store.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY);

    if (state) {
        const connected = haveConnected ?? true;
        await persisted_store.set(VIBIN_HOST_DETAILS_KEY, { ...state, haveConnected: connected });
        await persisted_store.save();
    }
}
