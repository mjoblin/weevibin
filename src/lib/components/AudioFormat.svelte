<script lang="ts">
    import { vibinState } from "../state.ts";

    $: format = $vibinState.display.format;
</script>

{#if format}
    <div class="AudioFormat">
        {format}
    </div>
{/if}

<style>
    .AudioFormat {
        font-size: 9px;
    }
</style>