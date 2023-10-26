<script lang="ts">
    import { IconQuestionMark } from "@tabler/icons-svelte";
    import tinycolor from "tinycolor2";

    export let icon = IconQuestionMark;
    export let disabled: boolean = false;
    export let size: number = 20;
    export let withBackground: boolean = false;
    export var color: string | undefined = undefined;
    export var backgroundColor: string | undefined = undefined;

    // TODO: Why doesn't the following work?
    // $: color = color ?? withBackground ? "#ffffff" : "#a6a7ab";
    $: {
        if (typeof color === "undefined") {
            color = withBackground ? "#ffffff" : "#a6a7ab";
        }
    }

    // $: backgroundColor = backgroundColor ?? withBackground ? "#1872c2" : "transparent";
    $: {
        if (typeof backgroundColor === "undefined") {
            backgroundColor = withBackground ? "#1872c2" : "transparent";
        }
    }

    let colorMax = "#ffffff";

    $: colorDim = tinycolor(color).darken(20).toString();
    $: colorBright = tinycolor(color).lighten(20).toString();

    $: backgroundColorHover =
        withBackground ? tinycolor(backgroundColor).darken(10).toString() : "#3e3e3e";

    // $: padding = withBackground ? "6px 7px" : "3px 5px";
    // $: padding = withBackground ? ".25em .25em" : "3px 5px";
    // $: padding = withBackground ? ".35em .35em" : ".3em .3em";

    let padding = ".3em .3em";

    $: cssVarStyles =
        `--color:${color};` +
        `--color-dim:${colorDim};` +
        `--color-bright:${colorBright};` +
        `--color-max:${colorMax};` +
        `--background-color:${backgroundColor};` +
        `--background-color-hover:${backgroundColorHover};` +
        `--padding:${padding};`;
</script>

<div>
    <button type="button" style={cssVarStyles} {disabled} on:click>
        <svelte:component this={icon} {size}/>
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
            cursor: not-allowed;
        }
    }
</style>