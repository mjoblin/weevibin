<script lang="ts">
    import { vibinState } from "../state.ts";
    import PowerButton from "./buttons/PowerButton.svelte";
    import Settings from "./buttons/SettingsButton.svelte";
    import Status from "./Status.svelte";
</script>

<div class="MiscState">
    <div class="lhs">
        <div class="Source">
            {$vibinState.source?.description_locale || $vibinState.source?.description}
        </div>
        <div>
            {$vibinState.display.format}
        </div>
        <div>
            {$vibinState.display.playback_source}
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
        align-items: center;
        justify-content: space-between;
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

    .lhs {
        display: flex;
        flex-direction: row;
        gap: 10px;
    }

    .rhs {
        display: flex;
        flex-direction: row;
        gap: 10px;
    }
</style>