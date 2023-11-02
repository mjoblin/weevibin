<script lang="ts">
    import { IconQuestionMark } from "@tabler/icons-svelte";
    import tinycolor from "tinycolor2";

    import { colorFromCssVar } from "../../utils.ts";

    type Variant = "filled" | "outline" | "subtle";

    // TODO: Investigate using CSS's "filter: brightness(1.2);" for hover

    export let icon = IconQuestionMark;
    export let variant: Variant = "subtle";
    export let disabled: boolean = false;
    export let size: number = 20;
    export var color: string | undefined = undefined;

    let textDim = colorFromCssVar("--text-dim");
    let textMax = colorFromCssVar("--text-max");
    let accentColor = colorFromCssVar("--accent-color");
    let backgroundDim = colorFromCssVar("--background-dim");

    // Establish core color (and its dimmed flavor). This core color will be used for the button
    // background and border, in different ways depending on variant.
    $: if (typeof color === "undefined") {
        color = variant === "subtle" ? textDim : accentColor;
    }

    $: colorDim = tinycolor(color).darken(20).toString();

    // Each button variant might have a different color for its icon, background, and border.
    $: iconColor = color;
    $: backgroundColor = color;
    $: backgroundColorHover = color;
    $: borderColor = color;
    $: borderColorHover = color;

    $: if (variant === "filled") {
        iconColor = textMax;
        backgroundColor = color;
        const backgroundDimmed = tinycolor(backgroundColor).darken(5).toString();
        backgroundColorHover = backgroundDimmed;
        borderColor = color;
        borderColorHover = backgroundDimmed;
    } else if (variant === "outline") {
        iconColor = color;
        backgroundColor = "transparent";
        backgroundColorHover = "transparent";
        borderColor = color;
        borderColorHover = color;
    } else if (variant === "subtle") {
        iconColor = color;
        backgroundColor = "transparent";
        backgroundColorHover = backgroundDim;
        borderColor = "transparent";
        borderColorHover = "transparent";
    }

    let padding = ".3em .3em";

    $: cssVarStyles =
        `--color:${color};` +
        `--color-dim:${colorDim};` +
        `--background-color:${backgroundColor};` +
        `--background-color-hover:${backgroundColorHover};` +
        `--border:1px solid ${borderColor};` +
        `--border-hover:1px solid ${borderColorHover};` +
        `--padding:${padding};`;
</script>

<div>
    <button type="button" style={cssVarStyles} {disabled} on:click>
        <div class="button-content">
            <svelte:component this={icon} {size} color={iconColor} />
            <slot />
        </div>
    </button>
</div>

<style>
    button {
        color: var(--color);
        background-color: var(--background-color);
        box-sizing: border-box;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        border: var(--border);
        border-radius: 4px;
        font-family: inherit;
        font-size: 1em;
        padding: var(--padding);
        cursor: pointer;
        text-align: center;
        line-height: 1.1;

        transition: 120ms all ease-in-out;

        &:hover:enabled {
            background-color: var(--background-color-hover);
            border: var(--border-hover);
        }

        &:disabled {
            color: var(--color-dim);
            cursor: not-allowed;
        }
    }

    .button-content {
        display: flex;
        align-items: center;
        gap: 5px;
    }
</style>