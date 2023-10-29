<script lang="ts">
    import { IconQuestionMark } from "@tabler/icons-svelte";
    import tinycolor from "tinycolor2";

    import { colorFromCssVar } from "../../utils.ts";

    // TODO: Investigate using CSS's "filter: brightness(1.2);" for hover

    export let icon = IconQuestionMark;
    export let disabled: boolean = false;
    export let size: number = 20;
    export let withBackground: boolean = false;
    export var color: string | undefined = undefined;
    export var backgroundColor: string | undefined = undefined;

    let textDim = colorFromCssVar("--text-dim");
    let textMax = colorFromCssVar("--text-max");
    let accentColorBright = colorFromCssVar("--accent-color-bright");
    let backgroundDim = colorFromCssVar("--background-dim");

    // TODO: Why doesn't the following work?
    // $: color = color ?? withBackground ? textMax : textDim;
    $: {
        if (typeof color === "undefined") {
            color = withBackground ? textMax : textDim;
        }
    }

    $: {
        if (typeof backgroundColor === "undefined") {
            backgroundColor = withBackground ? accentColorBright : "transparent";
        }
    }

    let colorMax = textMax;

    $: colorDim = tinycolor(color).darken(20).toString();
    $: colorBright = tinycolor(color).lighten(20).toString();

    $: backgroundColorHover =
        withBackground ? tinycolor(backgroundColor).darken(10).toString() : backgroundDim;
    $: backgroundColorDisabled = withBackground ? backgroundDim : "transparent";

    let padding = ".3em .3em";

    $: cssVarStyles =
        `--color:${color};` +
        `--color-dim:${colorDim};` +
        `--color-bright:${colorBright};` +
        `--color-max:${colorMax};` +
        `--background-color:${backgroundColor};` +
        `--background-color-hover:${backgroundColorHover};` +
        `--background-color-disabled:${backgroundColorDisabled};` +
        `--padding:${padding};`;
</script>

<div>
    <button type="button" style={cssVarStyles} {disabled} on:click>
        <div class="button-content">
            <svelte:component this={icon} {size}/>
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
        border: none;
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
            color: var(--color-bright);
        }

        &:active:enabled {
            color: var(--color-max);
        }

        &:disabled {
            color: var(--color-dim);
            background-color: var(--background-color-disabled);
            cursor: not-allowed;
        }
    }

    .button-content {
        display: flex;
        align-items: center;
        gap: 5px;
    }
</style>