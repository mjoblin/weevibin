<script lang="ts">
    import { IconVolume, IconVolumeOff } from "@tabler/icons-svelte";

    import { vibinState } from "./state";
    import { toggleMute } from "./vibinApi";
    import ToggleButton from "./ToggleButton.svelte";
</script>

{#if $vibinState.amplifier}
    <div class="Volume">
        <ToggleButton
            icon={$vibinState.amplifier.mute === "off" ? IconVolume : IconVolumeOff}
            size={14}
            on:click={() => toggleMute()}
        />
    </div>
{/if}

<style>
    .Volume {
        display: flex;
        flex-direction: row;
        flex-grow: 1;
        gap: 10px;
        align-items: center;
    }
</style>