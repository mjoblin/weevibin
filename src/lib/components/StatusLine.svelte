<script lang="ts">
    import AudioFormat from "./AudioFormat.svelte";
    import AudioSource from "./AudioSource.svelte";
    import PowerButton from "./buttons/PowerButton.svelte";
    import Settings from "./buttons/SettingsButton.svelte";
    import WebSocketConnectionStatus from "./WebSocketConnectionStatus.svelte";
</script>

<div class="MiscState">
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
    .MiscState {
        display: flex;
        gap: 10px;
        font-size: 0.8em;
        color: #9e9e9e;
        justify-content: space-between;
    }

    .lhs {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 10px;
    }

    .rhs {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 10px;
    }
</style>