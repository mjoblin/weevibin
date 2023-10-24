<script lang="ts">
    import {
        IconArrowsShuffle,
        IconPlayerPauseFilled,
        IconPlayerPlayFilled,
        IconPlayerStopFilled,
        IconPlayerTrackNextFilled,
        IconPlayerTrackPrevFilled,
        IconRepeat,
    } from "@tabler/icons-svelte";

    import { isPlaying, vibinState } from "./state";
    import {
        togglePlayback,
        nextTrack,
        pause,
        play,
        previousTrack,
        stop,
        toggleRepeat,
        toggleShuffle,
    } from "./vibinApi";
    import IconButton from "./IconButton.svelte";
    import ToggleButton from "./ToggleButton.svelte";

    $: canPauseOrStop =
        $isPlaying &&
        $vibinState.transport?.active_controls.some((active_control) =>
            ["pause", "stop", "toggle_playback"].includes(active_control)
        );

    $: canPlayOrResume =
        !$isPlaying &&
        $vibinState.transport?.active_controls.some((active_control) =>
            ["play", "toggle_playback"].includes(active_control)
        );

    $: canPause = $vibinState.transport?.active_controls.includes("pause");
    $: canTogglePlayback = $vibinState.transport?.active_controls.includes("toggle_playback");
</script>

<div class="TransportControls">
    <!-- Previous track -->
    <IconButton
        icon={IconPlayerTrackPrevFilled}
        disabled={!$vibinState.transport?.active_controls?.includes("previous")}
        on:click={() => previousTrack()}
    />

    <!-- Play/pause/stop/toggle -->
    {#if $isPlaying}
        {#if canTogglePlayback || canPause}
            <IconButton
                icon={IconPlayerPauseFilled}
                disabled={!canPauseOrStop}
                size={28}
                on:click={() => canTogglePlayback ? togglePlayback() : pause()}
            />
        {:else}
            <IconButton
                icon={IconPlayerStopFilled}
                disabled={!canPauseOrStop}
                size={28}
                on:click={() => stop()}
            />
        {/if}
    {:else}
        <!-- TODO: Add check for playing a Preset Id -->
        <IconButton
            icon={IconPlayerPlayFilled}
            disabled={!canPlayOrResume}
            size={28}
            on:click={() => canTogglePlayback ? togglePlayback() : play()}
        />
    {/if}

    <!-- Next track -->
    <IconButton
        icon={IconPlayerTrackNextFilled}
        disabled={!$vibinState.transport?.active_controls?.includes("next")}
        on:click={() => nextTrack()}
    />

    <!-- Repeat and Shuffle toggles -->
    <div class="Toggles">
        <ToggleButton
            isOn={$vibinState.transport?.repeat === "all"}
            icon={IconRepeat}
            disabled={!$vibinState.transport?.active_controls.includes("repeat")}
            on:click={() => toggleRepeat()}
        />
        <ToggleButton
            isOn={$vibinState.transport?.shuffle === "all"}
            icon={IconArrowsShuffle}
            disabled={!$vibinState.transport?.active_controls.includes("shuffle")}
            on:click={() => toggleShuffle()}
        />
    </div>
</div>

<style>
    .TransportControls {
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .Toggles {
        display: flex;
        flex-direction: column;
    }
</style>