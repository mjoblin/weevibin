<script lang="ts">
    import { currentScreen } from "./lib/state";

    import Settings from "./lib/screens/Settings.svelte";
    import Main from "./lib/screens/Main.svelte";
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