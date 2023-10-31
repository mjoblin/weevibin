<script lang="ts">
    import { appState, currentScreen, uiInitialized, vibinHost } from "./lib/state";

    import { connectToVibin } from "./lib/utils.ts";
    import Settings from "./lib/screens/Settings.svelte";
    import Main from "./lib/screens/Main.svelte";

    let haveAttemptedStartupVibinConnect = false;

    // If we haven't connected successfully at startup, then automatically switch to the settings
    // screen so the user knows that the host setting needs tweaking.
    $: if (!$vibinHost.haveConnected && !haveAttemptedStartupVibinConnect) {
        $currentScreen = "settings";
    }

    // At startup, automatically attempt to connect to the last-known host (if it was previously
    // successfully connected to).
    $: if (
        !haveAttemptedStartupVibinConnect &&
        $uiInitialized &&
        $appState.vibin_connection.state === "Disconnected" &&
        $vibinHost.haveConnected
    ) {
        haveAttemptedStartupVibinConnect = true;
        connectToVibin($vibinHost.host);
    }

    // TODO: UI:
    // TODO:     What is the TypeScript type for Tabler icons
    //
    // TODO: Rust:
    // TODO: XXX Look into controlling stop_reading()'s poll delay
    // TODO:     Reconnect WebSocket when it loses its connection
</script>

<main class="Application">
    {#if $currentScreen === "main"}
        <Main />
    {:else if $currentScreen === "settings"}
        <Settings />
    {/if}
</main>

<style>
    .Application {
        padding: 0 var(--app-padding);
    }
</style>