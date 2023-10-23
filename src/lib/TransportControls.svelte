<script lang="ts">
    import {
        IconPlayerPauseFilled,
        IconPlayerPlayFilled,
        IconPlayerStopFilled,
        IconPlayerTrackNextFilled,
        IconPlayerTrackPrevFilled,
    } from "@tabler/icons-svelte";

    import { isPlaying, vibinState } from "./state.ts";
    import IconButton from "./IconButton.svelte";

    const sendVibinCommand = async (endpoint: string) => {
        const response = await fetch(
            `http://192.168.2.101:8080/api${endpoint}`,
            {
                method: "POST",
            }
        );
    }

    const togglePlayback = async () => await sendVibinCommand("/transport/toggle_playback");
    const nextTrack = async () => await sendVibinCommand("/transport/next");
    const pause = async () => await sendVibinCommand("/transport/pause");
    const play = async () => await sendVibinCommand("/transport/play");
    const previousTrack = async () => await sendVibinCommand("/transport/previous");
    const stop = async () => await sendVibinCommand("/transport/stop");

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
</div>

<style>
    .TransportControls {
        display: flex;
        flex-direction: row;
        align-items: center;
    }
</style>