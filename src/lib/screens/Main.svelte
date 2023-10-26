<script lang="ts">
    import { seek } from "../vibinApi.ts";
    import MiscDetails from "../components/MiscDetails.svelte";
    import Playhead from "../components/Playhead.svelte";
    import TrackInfo from "../components/TrackInfo.svelte";
    import TransportControls from "../components/TransportControls.svelte";
    import VolumeControls from "../components/VolumeControls.svelte";

    const handleSeek = async (e: MouseEvent) => {
        const targetSecs = parseInt((e.target as HTMLInputElement).value);
        await seek(targetSecs);
    }
</script>

<div class="Layout">
    <div class="Playing">
        <TrackInfo />
        <VolumeControls />
    </div>
    <div class="Controls">
        <TransportControls />
        <Playhead on:click={handleSeek} />
    </div>
    <MiscDetails />
</div>

<style>
    .Layout {
        display: flex;
        flex-direction: column;
    }

    .Playing {
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .Controls {
        display: flex;
        flex-direction: row;
        gap: 10px;
        width: 100%;
        align-content: center;
    }
</style>