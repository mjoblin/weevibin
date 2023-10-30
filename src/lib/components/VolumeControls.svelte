<script lang="ts">
    import {
        IconArrowNarrowDown,
        IconArrowsDown,
        IconArrowNarrowUp,
        IconArrowsUp,
        IconVolume,
        IconVolumeOff
    } from "@tabler/icons-svelte";

    import { isPowerOn, vibinState } from "../state.ts";
    import { toggleMute, volumeDown, volumeSet, volumeUp } from "../vibinApi.ts";
    import { colorFromCssVar } from "../utils.ts";
    import Arc from "./Arc.svelte";
    import IconButton from "./buttons/IconButton.svelte";
    import ToggleButton from "./buttons/ToggleButton.svelte";

    const bigVolumeChangeAmount = 0.05;

    $: volume = $vibinState.amplifier?.volume || 0;
    $: isMuted = $vibinState.amplifier?.mute === "on";

    $: volumeDisplay = Math.round(($vibinState.amplifier?.volume || 0) * 100);
    $: volumeBigUp = Math.min(volume + bigVolumeChangeAmount, 1.0);
    $: volumeBigDown = Math.max(volume - bigVolumeChangeAmount, 0.0);
</script>

{#if $vibinState.amplifier && $isPowerOn}
    <div class="VolumeControls">
        <ToggleButton
            icon={$vibinState.amplifier.mute === "off" ? IconVolume : IconVolumeOff}
            size={14}
            on:click={toggleMute}
        />

        <Arc
            radius={20}
            thickness={4}
            progress={360 * volume}
            color="orange"
            trackColor={colorFromCssVar("--background-mid")}
        >
            <span class="current-volume-level">{volumeDisplay}</span>
        </Arc>

        <div class="up-down-buttons">
            <div class="button-pair">
                <IconButton
                    disabled={isMuted}
                    size={10}
                    icon={IconArrowNarrowUp}
                    on:click={volumeUp}
                />
                <IconButton
                    disabled={isMuted}
                    size={10}
                    icon={IconArrowNarrowDown}
                    on:click={volumeDown}
                />
            </div>
            <div>
                <IconButton
                    disabled={isMuted}
                    size={10}
                    icon={IconArrowsUp}
                    on:click={() => volumeSet(volumeBigUp)}
                />
                <IconButton
                    disabled={isMuted}
                    size={10}
                    icon={IconArrowsDown}
                    on:click={() => volumeSet(volumeBigDown)}
                />
            </div>
        </div>
    </div>
{/if}

<style>
    .VolumeControls {
        display: flex;
        gap: 0.35em;
        align-items: center;
    }

    .current-volume-level {
        font-size: 0.75em;
        font-weight: 600;
        opacity: 0.8;
        color: var(--text-normal);
    }

    .up-down-buttons {
        display: flex;
        gap: 0;
    }

    .button-pair {
        display: flex;
        flex-direction: column;
    }
</style>