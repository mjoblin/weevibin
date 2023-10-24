<script lang="ts">
    import IconButton from "./IconButton.svelte";

    export let icon;
    export let disabled: boolean = false;
    export let isOn: boolean = false;
    export let size: number = 12;

    let onColor = "#339af0";
    let offColor = "#a6a7ab";
</script>

<IconButton {icon} bind:disabled={disabled} {size} color={isOn ? onColor : offColor} on:click />

