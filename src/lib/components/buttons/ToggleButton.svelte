<script lang="ts">
    import tinycolor from "tinycolor2";

    import { colorFromCssVar } from "../../utils.ts";
    import IconButton from "./IconButton.svelte";

    let onColor = tinycolor(colorFromCssVar("--accent-color-bright")).brighten(20).toString();

    export let icon;
    export let disabled: boolean = false;
    export let isOn: boolean = false;
    export let size: number = 12;
</script>

<IconButton
    variant="subtle"
    {icon}
    {size}
    color={isOn ? onColor : undefined}
    bind:disabled={disabled}
    on:click
/>

