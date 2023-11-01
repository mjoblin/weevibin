<script lang="ts">
    import tinycolor, { type ColorInput } from "tinycolor2";

    import { type ConnectionStatus, appState, isConnected } from "../state.ts";

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
        <div class={"status-light" + `${["Connecting", "Disconnecting"].includes(connectionStatus) ? " light-animation" : ""}`}></div>
        <span class="status-text">{statusDisplay}</span>
        {#if $isConnected}
            <span class="happy">😃</span>
        {/if}
    </div>
{/if}

<style>
    .WebSocketConnectionStatus {
        display: flex;
        gap: 0.6em;
        align-items: center;
        color: var(--text-dim);

        & .status-light {
            border-radius: 50%;
            width: 0.7em;
            height: 0.7em;
            background-color: var(--status-color);
        }

        & .status-text {
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

    .light-animation {
        animation: animateLight 1.5s infinite;
    }

    @keyframes animateHappy {
        0%, 100% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.5);
        }
    }

    .happy {
        margin-left: 5px;
        font-size: 1.5em;
        animation: animateHappy 0.5s ease-in-out;
    }
</style>