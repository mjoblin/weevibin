import { invoke } from "@tauri-apps/api/tauri";
import * as logger from "tauri-plugin-log-api";

import { DEFAULT_VIBIN_PORT } from "./consts.ts";

/**
 * Request (from Rust) a connection to the Vibin WebSocket server at `host`.
 *
 * This only gets as far as invoking set_vibin_server, which will return _before_ the connection
 * attempt completes on the Rust side. This means that the success/failure of the connection
 * attempt will not be known until later (via AppState.vibin_connection.state).
 */
const connectToVibin = async (host: string) => {
    const wsUrl = new URL(`${/^wss?:\/\//.test(host) ? "" : "ws://"}${host}`);
    wsUrl.port = wsUrl.port ? wsUrl.port : `${DEFAULT_VIBIN_PORT}`;
    wsUrl.pathname = wsUrl.pathname === "/" ? "/ws" : wsUrl.pathname;

    await invoke("set_vibin_server", { vibinServer: wsUrl });
}

/**
 * Get the color value for the given cssVarName.
 *
 * Expects the CSS var's value to be in #rrggbb[aa] format.
 */
const colorFromCssVar = (cssVarName: string): string | undefined => {
    const cssValue = getComputedStyle(document.body).getPropertyValue(cssVarName);

    if (cssValue.match(/^#[a-fA-F0-9]{6,8}$/)) {
        return cssValue;
    }

    return undefined;
}

export {
    colorFromCssVar,
    connectToVibin,
    logger,
};
