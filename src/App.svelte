<script lang="ts">
    import { currentScreen } from "./lib/state";

    import Settings from "./lib/screens/Settings.svelte";
    import Main from "./lib/screens/Main.svelte";

    // TODO: UI:
    // TODO:     Create constants (colors, etc)
    // TODO:     Clean up top-level styles.css
    // TODO:     Consider root font size, and replace any "rem" with "em"
    // TODO: XXX Rename <MiscDetails>
    // TODO:     Different colors for different audio sources
    // TODO:     Improve display of stream codec
    // TODO: XXX Implement <Settings>
    // TODO:     What is the TypeScript type for Tabler icons
    // TODO:     Decide how to show connection status on <Main> (only show if not "Connected"?)
    // TODO:     Consider flow for starting weevibin without a vibin host set
    // TODO:     Research persisting app settings (vibin host; whether vibin host has been set or not)
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
        padding: 0 5px;
    }
</style>