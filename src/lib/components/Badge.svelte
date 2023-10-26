<script lang="ts">
    export let color = "#333b72";

    $: cssVarStyles = `--color:${color}`;
</script>

<div class="Badge" style={cssVarStyles}>
    <slot />
</div>

<style>
    .Badge {
        height: 1rem;
        padding: 0 7px;
        font-size: 7px;
        background-color: var(--color);
        border-radius: 2rem;
        text-transform: uppercase;
        justify-content: center;
        align-items: center;
        line-height: 1.1rem;
        color: #c0c0ff;
    }
</style>