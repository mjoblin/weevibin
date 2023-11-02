<script lang="ts">
    import tinycolor from "tinycolor2";

    import { colorFromCssVar } from "../../utils.ts";
    import IconButton from "./IconButton.svelte";
    import { IconPlayerTrackNextFilled } from "@tabler/icons-svelte";
    import { vibinState } from "../../state.ts";
    import { nextTrack } from "../../vibin_api.ts";

    export let icon;
    export let disabled: boolean = false;
    export let isOn: boolean = false;
    export let size: number = 12;

    let onColor = tinycolor(colorFromCssVar("--accent-color-bright")).brighten(20).toString();
</script>

<IconButton {icon} {size} {disabled} color={isOn ? onColor : undefined} on:click />
