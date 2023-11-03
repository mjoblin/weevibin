# `weevibin`

`weevibin` is a System Tray application for the [`vibin`](https://github.com/mjoblin/vibin)
StreamMagic music streaming controller. `weevibin` is a scaled-back sibling of the full
[`vibinui`](https://github.com/mjoblin/vibinui) Web interface. `weevibin` and `vibinui` can
be run independently.

<img alt="Interface" src="https://github.com/mjoblin/media/blob/main/weevibin/images/interface.png" width="400" />

#### In use

<img alt="In use" src="https://github.com/mjoblin/media/blob/main/weevibin/video/weevibin_usage.gif" width="425" />

## Features

`weevibin` supports:

* Display of current track information.
* Transport controls (play, pause, repeat, shuffle, track seek, etc).
* Amplifier controls (mute, volume).
* Power on/off.
* Shows when streamer is in standby mode.
* Various audio sources (local media, AirPlay, Internet Radio, etc).
* Ability to set the Vibin host.

## Screenshots

### Local media source

<img alt="Local media source" src="https://github.com/mjoblin/media/blob/main/weevibin/images/source_local_media.png" width="400" />

### AirPlay source

<img alt="AirPlay source" src="https://github.com/mjoblin/media/blob/main/weevibin/images/source_airplay.png" width="400" />

### Internet Radio source

<img alt="Internet Radio source" src="https://github.com/mjoblin/media/blob/main/weevibin/images/source_internet_radio.png" width="400" />

### Settings

<img alt="Settings" src="https://github.com/mjoblin/media/blob/main/weevibin/images/settings.png" width="400" />

### Standby mode

<img alt="Standby mode" src="https://github.com/mjoblin/media/blob/main/weevibin/images/standby_mode.png" width="400" />

## Developers

`weevibin` is a [Tauri] application, using [Svelte] for the UI.

### Local setup

See the [Tauri Prerequisites] for local development.

To develop in [RustRover]:

1. Clone the repository.
2. Run `npm install`.
3. In the repository root, run `npm run dev`. This will run the Vite half of the app.
4. In RustRover:
   * Attach the `Cargo.toml`.
   * Create a "Cargo" Run Configuration, set the "Working directory" to `weevibin/src-tauri`, and
     the "Command" to `run --no-default-features`.
   * Start the Run Configuration.

The application can also be developed in VS Code, although this has not been tested. See
[Debugging in VS Code] for more details.

### Building

The application can be built with `npm run tauri build`. For this to complete successfully, first
edit `tauri.conf.json` and change the `tauri.bundle.identifier` to something other than
`"com.tauri.app"`.

### Application structure

The application is made up of:

* A WebSocket connection to the Vibin backend, to receive information and updates related to
  playback state.
* HTTP REST requests to invoke actions on the Vibin backend (pause, play, next track, mute, etc).
* Local storage for the Vibin host name.

#### WebSocket

The Rust side receives regular Vibin updates over a WebSocket connection to the Vibin backend.
These updates come in via a few different Vibin message types (`"CurrentlyPlaying"`, `"Position"`,
`"System"`, `"TransportState"`). The Rust side uses this information to update a single
`VibinState` struct. The `VibinState` struct is essentially a simplified representation of the
full Vibin state, containing just the pieces useful for `weevibin`. Updates to the `VibinState`
struct are sent to the Svelte/UI half of the application using Tauri messaging. The Svelte UI can
then use this information to render the UI.

> NOTE: The WebSocket connection to `vibin` is managed on the Rust side using `tokio-tungstenite`. It
could be managed in JavaScript instead, but I wanted to learn more Rust -- and this way the UI can
focus on presentation.

#### REST

The Svelte UI issues REST requests directly to the Vibin backend (i.e. without going through Rust)
to perform actions like pause, play, next track, volume controls, etc. If these actions result in
backend Vibin state changes (like new track details), then these changes will be received via the
WebSocket flow.


[//]: # "--- Links -------------------------------------------------------------------------------"

[Tauri Prerequisites]: https://tauri.app/v1/guides/getting-started/prerequisites
[Tauri]: https://tauri.app
[Svelte]: https://svelte.dev
[RustRover]: https://www.jetbrains.com/rust
[Debugging in VS Code]: https://tauri.app/v1/guides/debugging/vs-code
