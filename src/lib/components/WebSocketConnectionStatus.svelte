<script lang="ts">
    import { appState, type ConnectionStatus } from "../state.ts";

    export let hideIfConnected: boolean = true;

    const statusColors: Record<ConnectionStatus, string> = {
        Connected: "green",
        Connecting: "yellow",
        Disconnected: "red",
        Disconnecting: "yellow",
    }

    $: connectionStatus = $appState.vibin_connection.state;
    $: cssVarStyles = `--status-color:${statusColors[connectionStatus] || "transparent"}`
</script>

{#if !hideIfConnected || connectionStatus !== "Connected"}
    <div class="WebSocketConnectionStatus" style={cssVarStyles}>
        <div class={"statusLight" + `${["Connecting", "Disconnecting"].includes(connectionStatus) ? " lightAnimation" : ""}`}></div>
        <span class="statusText">{$appState.vibin_connection.state}</span>
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

    @keyframes animateLight {
        0%, 100% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.25);
        }
    }

    .lightAnimation {
        animation: animateLight 1s infinite;
    }
</style>