<script lang="ts">
    import { IconPower } from "@tabler/icons-svelte";

    import { isConnected, vibinState } from "../../state.ts";
    import { powerOff, powerOn } from "../../vibinApi.ts";
    import IconButton from "./IconButton.svelte";

    $: isSystemOn = $vibinState.power === "on";
    $: bgColor = isSystemOn ? "#c92333" : "#1872c2";
    $: console.log("ON", isSystemOn);
    $: console.log("BG", bgColor);
</script>

<IconButton
    disabled={!$isConnected}
    icon={IconPower}
    size={18}
    backgroundColor={bgColor}
    withBackground={true}
    on:click={isSystemOn ? powerOff : powerOn}
/>
