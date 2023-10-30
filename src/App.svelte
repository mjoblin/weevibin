<script lang="ts">
    import { appState, currentScreen, uiInitialized, vibinHost } from "./lib/state";

    import { connectToVibin } from "./lib/utils.ts";
    import Settings from "./lib/screens/Settings.svelte";
    import Main from "./lib/screens/Main.svelte";

    let haveAttemptedStartupVibinConnect = false;

    // If we haven't connected successfully at startup, then automatically switch to the settings
    // screen so the user knows that the host setting needs tweaking.
    $: if (!$vibinHost.haveConnected) {
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
    // TODO: XXX Create constants (colors, etc)
    // TODO: XXX Clean up top-level styles.css
    // TODO: XXX Consider root font size, and replace any "rem" with "em"
    // TODO: XXX Rename <MiscDetails>
    // TODO: XXX Different colors for different audio sources
    // TODO: XXX Improve display of stream codec
    // TODO: XXX Implement <Settings>
    // TODO:     What is the TypeScript type for Tabler icons
    // TODO: XXX Decide how to show connection status on <Main> (only show if not "Connected"?)
    // TODO: XXX Research persisting app settings (vibin host; whether vibin host has been set or not)
    // TODO: XXX Consider flow for starting weevibin without a vibin host set
    // TODO: XXX Improve display details when disconnected
    // TODO:     Investigate using CSS's "filter: brightness(1.2);" for hover
    //
    // TODO: Rust:
    // TODO:    Look into controlling stop_reading()'s poll delay
    // TODO:    Reconnect WebSocket when it loses its connection
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