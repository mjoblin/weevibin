<script lang="ts">
    import { IconArrowLeft } from "@tabler/icons-svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    import { appState, currentScreen } from "../state.ts";
    import IconButton from "../components/buttons/IconButton.svelte";
    import WebSocketConnectionStatus from "../components/WebSocketConnectionStatus.svelte";

    let vibinServer = "192.168.2.101:8080";

    // There are two error sources:
    //  1. Errors coming from invoking set_vibin_server() on the Rust side.
    //  2. Errors from Rust's attempt to connect to the Vibin WebSocket server; these will be found
    //     in appState.vibin_connection.message when the .state is "Disconnected".

    // This error variable is for holding any set_vibin_server() exceptions.
    let error: string | undefined = undefined;

    // There are two phases of vibin server setting -- invoking the set change request from the
    // Rust side; and then waiting for connection status updates via appState. If the connection
    // state is not "Connected" or "Disconnected" then we consider it to be in a processing state.
    let isInvokingSetServer = false;
    $: isConnectionStateProcessing = !["Connected", "Disconnected"].includes($appState.vibin_connection.state);

    const setVibinServer = async () => {
        isInvokingSetServer = true;
        error = undefined;

        const wsUrl = new URL(`${/^wss?:\/\//.test(vibinServer) ? "" : "ws://"}${vibinServer}`);
        wsUrl.port = wsUrl.port ? wsUrl.port : "8080";
        wsUrl.pathname = wsUrl.pathname === "/" ? "/ws" : wsUrl.pathname;

        try {
            await invoke("set_vibin_server", { vibinServer: wsUrl });
        } catch (e) {
            error = `${e}`;
        } finally {
            isInvokingSetServer = false;
        }
    }

    const handleKeyDown = async (event: KeyboardEvent) => (event.key === "Enter") && await setVibinServer();
</script>

<div class="SettingsScreen">
    <div style="display: flex; align-items: center; justify-content: space-between">
        <h1 style="margin: 0">Settings</h1>
        <IconButton
            icon={IconArrowLeft}
            size={18}
            color="#d3d3d3"
            on:click={() => $currentScreen = "main"}
        >
            back
        </IconButton>
    </div>

    <div style="display: flex; flex-direction: row; gap: 10px">
        <div style="display: flex; align-items: flex-end; gap: 10px">
            <label>Vibin host
                <input type="text" autofocus bind:value={vibinServer} on:keydown={handleKeyDown}/>
            </label>
            <div style="display: flex; gap: 30px">
                <button
                    disabled={isInvokingSetServer || isConnectionStateProcessing}
                    on:click={setVibinServer}
                >
                    Connect
                </button>

                <WebSocketConnectionStatus hideIfConnected={false}/>
            </div>
        </div>
    </div>

    {#if error || ($appState.vibin_connection.state === "Disconnected" && $appState.vibin_connection.message)}
        <div class="error">
            <span>{error || $appState.vibin_connection.message}</span>
        </div>
    {/if}
</div>

<style>
    .SettingsScreen {
        font-size: 0.8em;
    }

    label, input {
        display: block;
    }

    .error {
        margin-top: 10px;
        color: #f32323;
    }

    @keyframes growShrinkAnimation {
        0%, 100% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.1);
        }
    }

    button:disabled {
        background-color: dimgrey;
        border-color: dimgrey;
        color: #c0c0c0;
        cursor: not-allowed;
    }
</style>