<script lang="ts">
    import { type SourceClass, vibinState } from "../state.ts";
    import Badge from "./Badge.svelte";

    const sourceClassColor: Record<SourceClass, string> = {
        "digital.coax": "--brown",
        "digital.toslink": "--brown",
        "digital.usb": "--brown",
        "stream.media": "--blue",
        "stream.radio": "--orange",
        "stream.service.airplay": "--green",
        "stream.service.cast": "--brown",
        "stream.service.roon": "--brown",
        "stream.service.spotify": "--brown",
        "stream.service.tidal": "--brown",
    }

    // The "source" is the streamer's current input (AirPlay, local media, internet radio, etc); and
    // the "playback source" is the associated stream source (computer/phone name for AirPlay, NAS
    // name for local media, etc).
    $: sourceClass = $vibinState.source?.class;
    $: sourceDescription = $vibinState.source?.description_locale || $vibinState.source?.description;
    $: playbackSource = $vibinState.display.playback_source;
    $: playbackSourceDisplay = playbackSource?.toLowerCase() === sourceDescription?.toLowerCase() ? "" : playbackSource;
</script>

{#if sourceDescription}
    <Badge color={sourceClass ? sourceClassColor[sourceClass] : "gray"}>
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
        gap: 0.6em;
    }
</style>