<script lang="ts">
    import { IconHome } from "@tabler/icons-svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    import { appErrorState, appState, currentScreen } from "../state.ts";
    import IconButton from "../components/buttons/IconButton.svelte";
    import WebSocketConnectionStatus from "../components/WebSocketConnectionStatus.svelte";

    let error = undefined;
    let vibinServer = "192.168.2.101:8080";

    $: connecting = $appState.vibin_connection.state === "Connecting";

    const setVibinServer = async () => {
        error = undefined;

        const wsUrl = new URL(`${/^wss?:\/\//.test(vibinServer) ? "" : "ws://"}${vibinServer}`);
        wsUrl.port = wsUrl.port ? wsUrl.port : "8080";
        wsUrl.pathname = wsUrl.pathname === "/" ? "/ws" : wsUrl.pathname;

        try {
            await invoke("set_vibin_server", {vibinServer: wsUrl});
        } catch (e) {
            // @ts-ignore
            error = e;
        }
    }
</script>

<div class="Settings">
    <div style="display: flex; align-items: center; gap: 10px">
        <h1 style="margin: 0">Settings</h1>
        <IconButton
            icon={IconHome}
            size={18}
            on:click={() => $currentScreen = "main"}
        />
    </div>

    <div style="display: flex; flex-direction: row; gap: 10px">
        <div style="display: flex; align-items: flex-end; gap: 10px">
            <label>Vibin host
                <input type="text" bind:value={vibinServer}/>
            </label>
            <div style="display: flex; gap: 30px">
                <button disabled={connecting} on:click={setVibinServer}>Connect</button>
                <WebSocketConnectionStatus hideIfConnected={false} />
            </div>
        </div>
    </div>

    {#if $appState.vibin_connection.state === "Disconnected" && $appState.vibin_connection.message}
        <div class="error">
            <span>{$appState.vibin_connection.message}</span>
        </div>
    {/if}
</div>

<style>
    .Settings {
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
</style>