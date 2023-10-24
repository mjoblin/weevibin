<script lang="ts">
    import MiscDetails from "./MiscDetails.svelte";
    import Playhead from "./Playhead.svelte";
    import TrackInfo from "./TrackInfo.svelte";
    import TransportControls from "./TransportControls.svelte";
    import VolumeControls from "./VolumeControls.svelte";
    import { seek } from "./vibinApi.ts";

    const handleSeek = async (e: MouseEvent) => {
        const targetSecs = parseInt((e.target as HTMLInputElement).value);
        await seek(targetSecs);
    }
</script>

<div class="Layout">
    <TrackInfo />
    <div class="Controls">
        <TransportControls />
        <Playhead on:click={handleSeek} />
    </div>
    <MiscDetails />
    <VolumeControls />
</div>

<style>
    .Layout {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    .Controls {
        display: flex;
        flex-direction: row;
        gap: 10px;
        width: 100%;
        align-content: center;
    }
</style>