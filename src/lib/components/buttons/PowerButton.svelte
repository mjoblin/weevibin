<script lang="ts">
    import { IconPower } from "@tabler/icons-svelte";

    import { isConnected, vibinState } from "../../state.ts";
    import { powerOff, powerOn } from "../../vibin_api.ts";
    import IconButton from "./IconButton.svelte";

    $: isSystemOn = $vibinState.power === "on";
    $: bgColor = isSystemOn ? "#e92333" : "#0862b2";  // TODO: --alert-color and --accent-color
</script>

<IconButton
    disabled={!$isConnected}
    icon={IconPower}
    size={18}
    backgroundColor={bgColor}
    withBackground={true}
    on:click={isSystemOn ? powerOff : powerOn}
/>
