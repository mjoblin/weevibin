export const sendVibinCommand = async (endpoint: string) => {
    const response = await fetch(
        `http://192.168.2.101:8080/api${endpoint}`,
        {
            method: "POST",
        }
    );
}

// System

export const powerOn = async () => await sendVibinCommand("/system/power/on");
export const powerOff = async () => await sendVibinCommand("/system/power/off");

// Transport

export const togglePlayback = async () => await sendVibinCommand("/transport/toggle_playback");
export const nextTrack = async () => await sendVibinCommand("/transport/next");
export const pause = async () => await sendVibinCommand("/transport/pause");
export const play = async () => await sendVibinCommand("/transport/play");
export const previousTrack = async () => await sendVibinCommand("/transport/previous");
export const stop = async () => await sendVibinCommand("/transport/stop");
export const toggleRepeat = async () => await sendVibinCommand("/transport/repeat");
export const toggleShuffle = async () => await sendVibinCommand("/transport/shuffle");
export const seek = async (target: number) => await sendVibinCommand(`/transport/seek?target=${target}`);

// Amplifier
export const toggleMute = async () => await sendVibinCommand("/system/amplifier/mute/toggle");
export const volumeUp = async () => await sendVibinCommand("/system/amplifier/volume/up");
export const volumeDown = async () => await sendVibinCommand("/system/amplifier/volume/down");
export const volumeSet = async (level: number) => await sendVibinCommand(`/system/amplifier/volume/${level}`);
