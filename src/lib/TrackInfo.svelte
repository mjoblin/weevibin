<script>
    import { vibinState } from "./state.ts";
</script>

<div class="TrackInfo">
    <div class="art" style="background-image: url('{$vibinState.display.art_url}')" />
    <div class="details">
        <span>{$vibinState.display.line1 || ""}</span>
        <span>{$vibinState.display.line2 || ""}</span>
        <span>{$vibinState.display.line3 || ""}</span>
    </div>
</div>

<style>
    :root {
        --art-size: 60px;
    }

    .TrackInfo {
        display: flex;
        flex-direction: row;
        gap: 10px;
        align-items: center;
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
    }

    .details {
        display: flex;
        flex-direction: column;
        overflow: hidden;
        padding-right: 5px;
        white-space: nowrap;

        & span {
            text-align: left;
            line-height: 1.30;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        & span:first-of-type {
            font-weight: bold;
        }

        & span:nth-of-type(2) {
            font-size: 0.9rem;
        }

        & span:last-of-type {
            color: #a0a0a0;
            font-size: 0.8rem;
        }
    }
</style>