<script lang="ts">
    import { IconSettings } from "@tabler/icons-svelte";

    import { currentScreen } from "../../state.ts";
    import IconButton from "./IconButton.svelte";
</script>

<IconButton
    icon={IconSettings}
    size={18}
    on:click={() => $currentScreen = "settings"}
/>
