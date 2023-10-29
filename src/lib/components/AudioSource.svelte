<script lang="ts">
    import { type SourceClass, vibinState } from "../state.ts";
    import Badge from "./Badge.svelte";

    const sourceClassColor: Record<SourceClass, string> = {
        "digital.coax": "--goldenrod",
        "digital.toslink": "--sea-green",
        "digital.usb": "--robin-egg-blue",
        "stream.media": "--cerulean",
        "stream.radio": "--tomato",
        "stream.service.airplay": "--moss-green",
        "stream.service.cast": "--burnt-umber",
        "stream.service.roon": "--rose-quartz",
        "stream.service.spotify": "--taupe-gray",
        "stream.service.tidal": "--space-cadet",
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
        <div class="AudioSource">
            <span>{sourceDescription}</span>
            {#if playbackSourceDisplay}
                <span style="font-weight: bold">{playbackSourceDisplay}</span>
            {/if}
        </div>
    </Badge>
{/if}

<style>
    .AudioSource {
        display: flex;
        gap: 0.6em;
    }
</style>