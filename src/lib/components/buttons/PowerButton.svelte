<script lang="ts">
    import { IconPower } from "@tabler/icons-svelte";

    import { isConnected, isPowerOn } from "../../state.ts";
    import { powerOff, powerOn } from "../../vibin_api.ts";
    import { colorFromCssVar } from "../../utils.ts";
    import IconButton from "./IconButton.svelte";

    let color = colorFromCssVar("--accent-color-bright");
</script>

<IconButton
    variant={$isPowerOn ? "outline" : "filled"}
    {color}
    disabled={!$isConnected}
    icon={IconPower}
    size={18}
    on:click={$isPowerOn ? powerOff : powerOn}
/>
