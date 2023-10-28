<script lang="ts">
    import tinycolor from "tinycolor2";

    export let color = "#333b72";

    // Defaults.
    let backgroundColor = color;
    let foregroundColor = "#f0f0f0";
    let badgeLuminance = 0.5;

    $: if (color.startsWith("--")) {
        // The color is a --css-variable, so get its value.
        const computedStyles = getComputedStyle(document.body);
        backgroundColor = computedStyles.getPropertyValue(color) || "gray";
    }

    // Have the foreground be light or dark based on background luminance
    $: badgeLuminance = tinycolor(backgroundColor).getLuminance();
    $: foregroundColor = badgeLuminance < 0.3 ? "#f3f3f3" : "#090909";

    $: cssVarStyles = `--color:${backgroundColor};--text-color:${foregroundColor}`;
</script>

<div class="Badge" style={cssVarStyles}>
    <slot />
</div>

<style>
    .Badge {
        height: 1rem;
        padding: 0 7px;
        font-size: 8px;
        font-weight: 500;
        background-color: var(--color);
        border-radius: 2rem;
        text-transform: uppercase;
        justify-content: center;
        align-items: center;
        line-height: 1.1rem;
        color: var(--text-color);
    }
</style>