<script lang="ts">
    import { IconQuestionMark } from "@tabler/icons-svelte";
    import tinycolor from "tinycolor2";

    export let icon = IconQuestionMark;
    export let disabled: boolean = false;
    export let size: number = 20;
    export let color: string = "#a6a7ab";

    let colorMax = "#ffffff";
    let colorDim = tinycolor(color).darken(20).toString();
    let colorBright = tinycolor(color).lighten(20).toString();

    $: cssVarStyles =
        `--color:${color};--color-dim:${colorDim};--color-bright:${colorBright};--color-max:${colorMax}`;
</script>

<button type="button" style={cssVarStyles} on:click {disabled}>
    <svelte:component this={icon} {size}/>
</button>

<style>
    :root {
        --bg-color-hover: #3e3e3e;
    }

    button {
        color: var(--color);

        box-sizing: border-box;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        border: none;
        border-radius: 3px;
        background-color: transparent;
        font-family: inherit;
        font-size: 1em;
        padding: 3px 5px;
        cursor: pointer;
        text-align: center;
        line-height: 1.1;

        transition: 120ms all ease-in-out;

        &:hover:enabled {
            background-color: var(--bg-color-hover);
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