<script lang="ts">
    import { seek } from "../vibinApi.ts";
    import Playhead from "../components/Playhead.svelte";
    import StatusLine from "../components/StatusLine.svelte";
    import TrackInfo from "../components/TrackInfo.svelte";
    import TransportControls from "../components/TransportControls.svelte";
    import VolumeControls from "../components/VolumeControls.svelte";

    const handleSeek = async (e: MouseEvent) => {
        const targetSecs = parseInt((e.target as HTMLInputElement).value);
        await seek(targetSecs);
    }
</script>

<div class="MainScreen">
    <div class="now-playing">
        <TrackInfo />
        <VolumeControls />
    </div>
    <div class="playback-controls">
        <TransportControls />
        <Playhead on:click={handleSeek} />
    </div>
    <StatusLine />
</div>

<style>
    .MainScreen {
        display: flex;
        flex-direction: column;
    }

    .now-playing {
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .playback-controls {
        display: flex;
        flex-direction: row;
        gap: 10px;
        width: 100%;
        align-content: center;
    }
</style>