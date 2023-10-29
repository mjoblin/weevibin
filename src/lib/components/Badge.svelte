<script lang="ts">
    import tinycolor from "tinycolor2";

    import { colorFromCssVar } from "../utils.ts";

    export let color = colorFromCssVar("--accent-color");

    // Defaults.
    let backgroundColor = color;
    let foregroundColor = colorFromCssVar("--text-max")
    let badgeLuminance = 0.5;

    $: if (color.startsWith("--")) {
        backgroundColor = colorFromCssVar(color) || "gray";
    }

    // Have the foreground be light or dark based on background luminance
    $: badgeLuminance = tinycolor(backgroundColor).getLuminance();
    $: foregroundColor = badgeLuminance < 0.3 ? foregroundColor : colorFromCssVar("--text-min");

    $: cssVarStyles = `--color:${backgroundColor};--text-color:${foregroundColor}`;
</script>

<div class="Badge" style={cssVarStyles}>
    <slot />
</div>

<style>
    .Badge {
        height: 2em;
        padding: 0 1em;
        font-size: 0.65em;
        font-weight: 500;
        background-color: var(--color);
        border-radius: 100vh; /* Large border-radius forces 50% of container height */
        text-transform: uppercase;
        justify-content: center;
        align-items: center;
        line-height: 2.1em;
        color: var(--text-color);
    }
</style>