<script lang="ts">
    import { vibinState } from "../state.ts";
    import Badge from "./Badge.svelte";
    import PowerButton from "./buttons/PowerButton.svelte";
    import Settings from "./buttons/SettingsButton.svelte";
    import Status from "./Status.svelte";

    // The "source" is the streamer's current input (AirPlay, local media, internet radio, etc); and
    // the "playback source" is the associated stream source (computer/phone name for AirPlay, NAS
    // name for local media, etc).
    $: sourceDescription = $vibinState.source?.description_locale || $vibinState.source?.description || "unknown";
    $: playbackSource = $vibinState.display.playback_source;
    $: playbackSourceDisplay = playbackSource?.toLowerCase() === sourceDescription.toLowerCase() ? "" : playbackSource;
</script>

<div class="MiscState">
    <div class="lhs">
        <Badge>
            <div class="sourceDetails">
                <span>{sourceDescription}</span>
                {#if playbackSourceDisplay}
                    <span style="font-weight: bold">{playbackSourceDisplay}</span>
                {/if}
            </div>
        </Badge>
        <div class="formatDetails">
            {$vibinState.display.format}
        </div>
        <Status/>
    </div>

    <div class="rhs">
        <Settings/>
        <PowerButton/>
    </div>
</div>

<style>
    .MiscState {
        display: flex;
        gap: 10px;
        font-size: 0.8em;
        color: #7e7e7e;
        justify-content: space-between;
    }

    .sourceDetails {
        display: flex;
        flex-direction: row;
        gap: 0.8em;
    }

    .formatDetails {
        font-size: 9px;
     }

    .lhs {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 10px;
    }

    .rhs {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 10px;
    }
</style>