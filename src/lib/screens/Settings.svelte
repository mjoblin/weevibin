<script lang="ts">
    import { IconArrowLeft } from "@tabler/icons-svelte";

    import { appState, currentScreen, vibinHost } from "../state.ts";
    import { connectToVibin } from "../utils.ts";
    import IconButton from "../components/buttons/IconButton.svelte";
    import WebSocketConnectionStatus from "../components/WebSocketConnectionStatus.svelte";

    let vibinHostNameSetting = $vibinHost.host;

    // There are two error sources:
    //  1. Errors coming from invoking set_vibin_server() on the Rust side.
    //  2. Errors from Rust's attempt to connect to the Vibin WebSocket server; these will be found
    //     in appState.vibin_connection.message when the .state is "Disconnected".

    // This error variable is for holding any set_vibin_server() exceptions.
    let error: string | undefined = undefined;

    // There are two phases of vibin server setting -- invoking the set change request from the
    // Rust side; and then waiting for connection status updates via appState. If the connection
    // state is not "Connected" or "Disconnected" then we consider it to be in a processing state.
    let isInvokingConnectRequest = false;
    $: isConnectionStateProcessing = !["Connected", "Disconnected"].includes($appState.vibin_connection.state);

    const setVibinServer = async () => {
        if (!vibinHostNameSetting) {
            return;
        }

        await vibinHost.setHostName(vibinHostNameSetting);

        isInvokingConnectRequest = true;
        error = undefined;

        try {
            await connectToVibin(vibinHostNameSetting);
        } catch (e) {
            error = `${e}`;
        } finally {
            isInvokingConnectRequest = false
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
            on:click={() => $currentScreen = "main"}
        >
            main
        </IconButton>
    </div>

    <div style="display: flex; gap: 10px">
        <div style="display: flex; align-items: flex-end; gap: 10px">
            <label>Vibin host
                <input type="text" autofocus bind:value={vibinHostNameSetting} on:keydown={handleKeyDown}/>
            </label>
            <div style="display: flex; gap: 30px">
                <button
                    disabled={isInvokingConnectRequest || isConnectionStateProcessing}
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
        font-weight: bold;
        color: var(--alert-color);
    }

    @keyframes growShrinkAnimation {
        0%, 100% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.1);
        }
    }
</style>