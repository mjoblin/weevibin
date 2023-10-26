<script lang="ts">
    import { IconHome } from "@tabler/icons-svelte";

    import { currentScreen } from "../state.ts";
    import IconButton from "../components/buttons/IconButton.svelte";
</script>

<div class="Settings">
    <h1>Settings</h1>

    <IconButton
        icon={IconHome}
        size={18}
        on:click={() => $currentScreen = "main"}
    />
</div>

<style>
    .Settings {

    }
</style>