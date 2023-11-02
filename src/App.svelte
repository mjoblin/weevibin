<script lang="ts">
    import { appState, currentScreen, uiInitialized, vibinHost } from "./lib/state";

    import { connectToVibin, logger } from "./lib/utils.ts";
    import Settings from "./lib/screens/Settings.svelte";
    import Main from "./lib/screens/Main.svelte";

    logger.info("Initializing UI");

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
        border-radius: 7px;
        border: 1px solid #51545a;
        padding: 12px 12px 9px 12px;
        color: #f6f6f6;
        background-color: #2f2f2f;
    }
</style>