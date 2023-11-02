<script lang="ts">
    import { IconPower } from "@tabler/icons-svelte";
    import tinycolor from "tinycolor2";

    import { isConnected, isPowerOn } from "../../state.ts";
    import { powerOff, powerOn } from "../../vibin_api.ts";
    import { colorFromCssVar } from "../../utils.ts";
    import IconButton from "./IconButton.svelte";

    let accentColorBright = colorFromCssVar("--accent-color-bright");
    let accentColorBrighter = tinycolor(accentColorBright).brighten(20).toString();

    $: color = isPowerOn ? accentColorBrighter : accentColorBright;
</script>

<IconButton
    variant={$isPowerOn ? "outline" : "filled"}
    {color}
    disabled={!$isConnected}
    icon={IconPower}
    size={18}
    on:click={$isPowerOn ? powerOff : powerOn}
/>
