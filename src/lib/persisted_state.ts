import { Store } from "tauri-plugin-store-api";

import { DEFAULT_VIBIN_HOST } from "./consts.ts";

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
    host: DEFAULT_VIBIN_HOST,
    haveConnected: false,
};

const persistedStore = new Store("weevibin.dat");

export const getPersistedVibinHostDetails = async (): Promise<VibinHostDetails> => {
    // NOTE: persistedStore.has(<key>) does not return falsey if the store file doesn't exist
    if (!await persistedStore.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY)) {
        await persistedStore.set(VIBIN_HOST_DETAILS_KEY, VIBIN_HOST_DETAILS_DEFAULT);
        await persistedStore.save();
    }

    const hostDetails = await persistedStore.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY);

    return hostDetails as VibinHostDetails;
}

export const setPersistedVibinHost = async (host: string) => {
    const state = await persistedStore.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY);

    if (state) {
        await persistedStore.set(VIBIN_HOST_DETAILS_KEY, { ...state, host });
        await persistedStore.save();
    }
}

export const setPersistedVibinHaveConnected = async (haveConnected?: boolean) => {
    const state = await persistedStore.get<VibinHostDetails>(VIBIN_HOST_DETAILS_KEY);

    if (state) {
        const connected = haveConnected ?? true;
        await persistedStore.set(VIBIN_HOST_DETAILS_KEY, { ...state, haveConnected: connected });
        await persistedStore.save();
    }
}
