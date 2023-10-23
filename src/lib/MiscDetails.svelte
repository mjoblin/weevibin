<script>
    import { vibinState } from "./state.ts";
</script>

<div class="MiscState">
    <div class="Source">
        {$vibinState.source?.description_locale || $vibinState.source?.description}
    </div>
    <div>
        {$vibinState.display.format}
    </div>
</div>

<style>
    .MiscState {
        display: flex;
        gap: 10px;
        font-size: 0.8em;
        color: #7e7e7e;
    }

    .Source {
        padding: 0px 10px;
        border-radius: 15px;
        background-color: #333b72;
    }
</style>