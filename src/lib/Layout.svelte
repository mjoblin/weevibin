<script>
    import MiscDetails from "./MiscDetails.svelte";
    import TrackInfo from "./TrackInfo.svelte";
    import TransportControls from "./TransportControls.svelte";
</script>

<div class="Layout">
    <TrackInfo />
    <TransportControls />
    <MiscDetails />
</div>

<style>
    .Layout {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }
</style>