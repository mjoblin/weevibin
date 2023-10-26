<script lang="ts">
    import {
        IconArrowNarrowDown,
        IconArrowsDown,
        IconArrowNarrowUp,
        IconArrowsUp,
        IconVolume,
        IconVolumeOff
    } from "@tabler/icons-svelte";

    import { vibinState } from "../state.ts";
    import { toggleMute, volumeDown, volumeSet, volumeUp } from "../vibinApi.ts";
    import Arc from "./Arc.svelte";
    import IconButton from "./buttons/IconButton.svelte";
    import ToggleButton from "./buttons/ToggleButton.svelte";

    // Approach to drawing the arc:
    // http://www.independent-software.com/drawing-progress-arc-in-pure-css-using-skewed-rectangles.html

    const bigVolumeChangeAmount = 0.05;

    $: volume = $vibinState.amplifier?.volume || 0;
    $: isMuted = $vibinState.amplifier?.mute === "on";

    $: volumeDisplay = Math.round(($vibinState.amplifier?.volume || 0) * 100);
    $: volumeBigUp = Math.min(volume + bigVolumeChangeAmount, 1.0);
    $: volumeBigDown = Math.max(volume - bigVolumeChangeAmount, 0.0);
</script>

{#if $vibinState.amplifier}
    <div class="Volume">
        <ToggleButton
            icon={$vibinState.amplifier.mute === "off" ? IconVolume : IconVolumeOff}
            size={14}
            on:click={toggleMute}
        />

        <Arc radius={20} thickness={3} progress={360 * volume}>
            <span class="currentVolumeLevel">{volumeDisplay}</span>
        </Arc>

        <div class="upDownButtons">
            <div class="buttonPair">
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
    .Volume {
        display: flex;
        flex-direction: row;
        gap: 5px;
        align-items: center;
    }

    .currentVolumeLevel {
        font-size: 12px;
        font-weight: 600;
        color: #d3d3d3;
    }

    .upDownButtons {
        display: flex;
        flex-direction: row;
        gap: 0;
    }

    .buttonPair {
        display: flex;
        flex-direction: column;
    }
</style>