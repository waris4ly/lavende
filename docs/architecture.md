# Architecture & Core Concepts

The Lavende ecosystem is fundamentally split into two layers: the **Rust Core Engine** and the **Language Wrappers**.

## The Rust Core Engine

At the heart of Lavende is `liblavende` (or `lavende_go.a` for Golang), a high-performance native library written in Rust.

### Responsibilities
1. **Discord Voice UDP Streaming**: It establishes UDP connections to Discord's voice servers, handles RTP packet encryption (Sodium/XChaCha20), and maintains the streaming heartbeat.
2. **Audio Decoding**: It decodes Opus, MP3, FLAC, and other formats natively.
3. **Digital Signal Processing (DSP)**: It applies real-time audio manipulation (Nightcore, Vaporwave, Equalizers, Bassboost, 3D Rotation).
4. **Track Resolving**: It interfaces with external APIs (like YouTube and SoundCloud) to resolve queries into streamable audio formats.

## The FFI Layer (Foreign Function Interface)

Because Rust cannot natively run inside Node.js, Python, or Golang without a bridge, Lavende uses FFI (and CGO for Golang).

When you call `player.play()` in Node.js, the wrapper executes a C-binding `lavende_player_play(player_ptr)`, passing the pointer down to the Rust core. This ensures that the heavy lifting (audio encoding and network I/O) happens in Rust, preventing the Node.js event loop or Python GIL from blocking.

## The Manager System

Across all language wrappers, the architecture dictates a `LavendeManager`.

The Manager acts as the orchestrator. Because Lavende handles voice connections, it needs to know when Discord sends Gateway events like `VOICE_STATE_UPDATE` and `VOICE_SERVER_UPDATE`.

You pipe these raw JSON events into the `LavendeManager`. The manager decodes them, identifies the Guild ID, and routes the server endpoint and token to the respective `Player` instance in the Rust core.

## The Player System

A `Player` is an instance attached to a specific Discord Guild. 

- **State Management**: It tracks whether it is playing, paused, or buffering.
- **Queue**: A sub-component that manages single tracks or playlists.
- **FilterManager**: A sub-component that allows applying dynamic audio filters on the fly.
- **Event Emitters**: It emits lifecycle events back across the FFI boundary (`trackStart`, `trackEnd`, `queueEnd`, `error`), allowing your bot to react (e.g., sending a "Now Playing" message).
