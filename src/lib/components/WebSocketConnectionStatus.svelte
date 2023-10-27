<script lang="ts">
    import { appState, type ConnectionStatus } from "../state.ts";

    export let hideIfConnected: boolean = true;

    const statusColors: Record<ConnectionStatus, string> = {
        Connected: "green",
        Connecting: "yellow",
        Disconnected: "red",
    }

    $: connectionStatus = $appState.vibin_connection;
    $: cssVarStyles = `--status-color:${statusColors[connectionStatus] || "transparent"}`
</script>

{#if !hideIfConnected || connectionStatus !== "Connected"}
    <div class="WebSocketConnectionStatus" style={cssVarStyles}>
        <div class="statusLight"></div>
        <span class="statusText">{$appState.vibin_connection}</span>
    </div>
{/if}

<style>
    .WebSocketConnectionStatus {
        display: flex;
        flex-direction: row;
        gap: 5px;
        align-items: center;
        color: #9e9e9e;

        & .statusLight {
            border-radius: 50%;
            width: 0.7em;
            height: 0.7em;
            background-color: var(--status-color);
        }

        & .statusText {
            text-transform: lowercase;
            font-size: 0.8em;
        }
    }
</style>