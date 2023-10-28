<script lang="ts">
    import tinycolor, { type ColorInput } from "tinycolor2";

    import { appState, type ConnectionStatus } from "../state.ts";

    export let hideIfConnected: boolean = true;

    const statusColors: Record<ConnectionStatus, ColorInput> = {
        Connected: "green",
        Connecting: "yellow",
        Disconnected: "red",
        Disconnecting: "yellow",
    }

    $: connectionStatus = $appState.vibin_connection.state;
    $: statusDisplay =
        connectionStatus === "Disconnected" ?
            "Not connected"
            : ["Connecting", "Disconnecting"].includes(connectionStatus) ?
            `${connectionStatus}...`
            : connectionStatus;
    $: statusColor = statusColors[connectionStatus] || "gray";
    $: statusColorBright = tinycolor(statusColor).lighten(30).toString();

    $: cssVarStyles =
        `--status-color:${statusColor};` +
        `--status-color-bright:${statusColorBright}`;
</script>

{#if !hideIfConnected || connectionStatus !== "Connected"}
    <div class="WebSocketConnectionStatus" style={cssVarStyles}>
        <div class={"statusLight" + `${["Connecting", "Disconnecting"].includes(connectionStatus) ? " lightAnimation" : ""}`}></div>
        <span class="statusText">{statusDisplay}</span>
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
            background-color: var(--status-color);
        }
        50% {
            transform: scale(1.25);
            background-color: var(--status-color-bright);
        }
    }

    .lightAnimation {
        animation: animateLight 1.5s infinite;
    }
</style>