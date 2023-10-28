<script lang="ts">
    import { vibinState } from "../state.ts";
    import Badge from "./Badge.svelte";

    // The "source" is the streamer's current input (AirPlay, local media, internet radio, etc); and
    // the "playback source" is the associated stream source (computer/phone name for AirPlay, NAS
    // name for local media, etc).
    $: sourceDescription = $vibinState.source?.description_locale || $vibinState.source?.description;
    $: playbackSource = $vibinState.display.playback_source;
    $: playbackSourceDisplay = playbackSource?.toLowerCase() === sourceDescription?.toLowerCase() ? "" : playbackSource;
</script>

{#if sourceDescription}
    <Badge>
        <div class="sourceDetails">
            <span>{sourceDescription}</span>
            {#if playbackSourceDisplay}
                <span style="font-weight: bold">{playbackSourceDisplay}</span>
            {/if}
        </div>
    </Badge>
{/if}

<style>
    .sourceDetails {
        display: flex;
        flex-direction: row;
        gap: 0.8em;
    }
</style>