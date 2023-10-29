<script lang="ts">
    import { playheadPosition, vibinState } from "../state.ts";
    import { colorFromCssVar } from "../utils.ts";

    $: progress = $playheadPosition && $vibinState.active_track?.duration ?
        ($playheadPosition / $vibinState.active_track.duration) * 100 : 0;

    $: canSeek = $vibinState.transport?.active_controls.includes("seek");

    $: progressColor = canSeek ? colorFromCssVar("--accent-color-bright") : "#6c6f76";
    $: progressRemainingColor = colorFromCssVar("--background-mid");

    $: cssVarStyles =
        `--progress:${progress}%;` +
        `--progress-color:${progressColor};` +
        `--progress-remaining-color:${progressRemainingColor};`;

    const leadingZeros = new RegExp("^00:");

    /**
     * Convert a duration in seconds into "hh:mm:ss", without the hh: if it would have been "00:".
     */
    const prettyDuration = (duration: number | undefined) =>
        typeof duration === "number" ?
            new Date(duration * 1000).toISOString().substring(11, 19).replace(leadingZeros, "")
            : "--:--";
</script>

<div class="Playhead">
    <span class="time">{prettyDuration($playheadPosition)}</span>
    <input
        style={cssVarStyles}
        class:can-seek={canSeek}
        disabled={!canSeek}
        type="range"
        min="0"
        max={$vibinState.active_track?.duration}
        step="1"
        bind:value={$playheadPosition}
        on:click
    />
    <span class="time">{prettyDuration($vibinState.active_track?.duration)}</span>
</div>

<style>
    .Playhead {
        flex-grow: 1;
        display: flex;
        align-items: center;
        gap: 5px;
    }

    .time {
        font-size: 0.7em;
        color: var(--text-mid);
        white-space: nowrap;
    }

    input[type="range"] {
        appearance: none;
        cursor: not-allowed;
        width: 100%;
        height: 3px;
        border: none;
        padding: 0;
        border-radius: 2px;
        background: linear-gradient(
            to right,
            var(--progress-color) 0%,
            var(--progress-color) var(--progress),
            var(--progress-remaining-color) var(--progress),
            var(--progress-remaining-color) 100%
        );
    }

    input[type="range"]::-webkit-slider-thumb {
        appearance: none;
    }

    input[type="range"]::-webkit-slider-runnable-track  {
        appearance: none;
        box-shadow: none;
        border: none;
    }

    .can-seek {
        & {
            cursor: pointer;
        }

        &::-webkit-slider-thumb {
            cursor: ew-resize;
            height: 10px;
            width: 4px;
            border-radius: 2px;
            background: var(--root-color);
            transition: background .3s ease-in-out;
        }

        &::-webkit-slider-runnable-track {
            cursor: pointer;
        }
    }
</style>