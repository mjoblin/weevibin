<script lang="ts">
    import AudioFormat from "./AudioFormat.svelte";
    import AudioSource from "./AudioSource.svelte";
    import PowerButton from "./buttons/PowerButton.svelte";
    import Settings from "./buttons/SettingsButton.svelte";
    import WebSocketConnectionStatus from "./WebSocketConnectionStatus.svelte";
</script>

<div class="StatusLine">
    <div class="lhs">
        <AudioSource />
        <AudioFormat />
    </div>

    <div class="rhs">
        <WebSocketConnectionStatus />
        <Settings/>
        <PowerButton/>
    </div>
</div>

<style>
    .StatusLine {
        display: flex;
        font-size: 0.8em;
        justify-content: space-between;
    }

    .lhs {
        display: flex;
        align-items: center;
        gap: 1em;
    }

    .rhs {
        display: flex;
        align-items: center;
        gap: 1em;
    }
</style>