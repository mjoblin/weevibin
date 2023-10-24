<script lang="ts">
    import { vibinState } from "./state";
</script>

<div class="MiscState">
    <div class="Source">
        {$vibinState.source?.description_locale || $vibinState.source?.description}
    </div>
    <div>
        {$vibinState.display.format}
    </div>
    <div>
        {$vibinState.display.playback_source}
    </div>
</div>

<style>
    .MiscState {
        display: flex;
        gap: 10px;
        font-size: 0.8em;
        color: #7e7e7e;
        align-items: center;
    }

    .Source {
        height: 1rem;
        padding: 0 7px;
        font-size: 7px;
        background-color: #333b72;
        border-radius: 2rem;
        text-transform: uppercase;
        justify-content: center;
        align-items: center;
        line-height: 1.1rem;
        color: #c0c0ff;
    }
</style>