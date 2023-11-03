<script lang="ts">
    import { isConnected, vibinState } from "../state.ts";
    import { isUrlOk } from "../utils.ts";

    let isArtImageOk: boolean = true;

    $: artUrl = $vibinState.display.art_url;
    $: haveTextDetails = $vibinState.display.line1 || $vibinState.display.line2 || $vibinState.display.line3;

    $: artUrl && isUrlOk(artUrl).then((isOk) => isArtImageOk = isOk);
</script>

<div class="TrackInfo">
    <div
        class={"art" + `${!(artUrl && isArtImageOk) ? " art-unavailable" : ""}`}
        style={`background-image: ${artUrl && isArtImageOk ? `url(${artUrl})` : undefined}`}
    />
    <div class="details">
        {#if haveTextDetails}
            <span class="details-line1">{$vibinState.display.line1 || ""}</span>
            <span class="details-line2">{$vibinState.display.line2 || ""}</span>
            <span class="details-line3">{$vibinState.display.line3 || ""}</span>
        {:else}
            <div class="delayed-display" style="display: flex; flex-direction: column">
                <span class="details-line1">No track details</span>
                {#if !$isConnected}
                    <span class="details-line2">Not connected to Vibin; configure in settings.</span>
                {/if}
            </div>
        {/if}
    </div>
</div>

<style>
    :root {
        --art-size: 60px;
    }

    .TrackInfo {
        display: flex;
        gap: 0.7em;
        align-items: flex-start;
        max-width: 100%;
        padding: 3px;
        overflow: hidden;
        white-space: nowrap;
    }

    .art {
        min-width: var(--art-size);
        max-width: var(--art-size);
        min-height: var(--art-size);
        max-height: var(--art-size);
        background-size: contain;
        background-repeat: no-repeat;
        background-position: center center;
        border-radius: 3px;
    }

    .art-unavailable {
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: var(--background-mid);
    }

    .art-unavailable::after {
        content: "no art";
        text-transform: uppercase;
        color: var(--text-dim);
        font-size: 0.5em;
        font-weight: 600;
    }

    .details {
        display: flex;
        flex-direction: column;
        overflow: hidden;
        padding-right: 0.3em;
        white-space: nowrap;

        & span {
            text-align: left;
            line-height: 1.30;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        & .details-line1 {
            margin-top: 0.4em;
            font-weight: 600;
            font-size: 0.9em;
        }

        & .details-line2 {
            font-size: 0.7em;
        }

        & .details-line3 {
            color: var(--text-dim);
            font-size: 0.7em;
        }
    }

    @keyframes delayedDisplayAnimation {
        0% {
            opacity: 0;
        }
        99% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }

    .delayed-display {
        animation: delayedDisplayAnimation 3s forwards;
    }
</style>