<script lang="ts">
    import { IconPower } from "@tabler/icons-svelte";
    import tinycolor from "tinycolor2";

    import { isConnected, isSystemPowerOn } from "../../state.ts";
    import { powerOff, powerOn } from "../../vibin_api.ts";
    import { colorFromCssVar } from "../../utils.ts";
    import IconButton from "./IconButton.svelte";

    let accentColorBright = colorFromCssVar("--accent-color-bright");
    let accentColorBrighter = tinycolor(accentColorBright).brighten(20).toString();

    $: color = isSystemPowerOn ? accentColorBrighter : accentColorBright;
</script>

<IconButton
    variant={$isSystemPowerOn ? "outline" : "filled"}
    {color}
    disabled={!$isConnected}
    icon={IconPower}
    size={18}
    on:click={$isSystemPowerOn ? powerOff : powerOn}
/>
