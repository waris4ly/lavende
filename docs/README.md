# Lavende Ecosystem Documentation

Lavende is a lightning-fast, high-performance native audio processing library for Discord bots. It acts as a complete, zero-overhead replacement for traditional external audio nodes (like Lavalink), executing directly inside your bot's process while abstracting the heavy lifting to a highly optimized Rust core.

## Documentation Structure

This documentation is meticulously structured to cover the entire Lavende ecosystem, from the core architectural concepts to the specifics of each language wrapper.

- **[Architecture & Core Concepts](./architecture.md)**
  Understand how the Rust core handles UDP voice streams, the FFI boundary, and the DSP engine.

- **Language Wrappers**
  Detailed, component-level documentation for each officially supported language.
  - **[Node.js](./node/README.md)** (TypeScript / JavaScript)
  - **[Python](./python/README.md)**
  - **[Golang](./golang/README.md)**

## Why Lavende?

Most Discord bots rely on a separate Java process (Lavalink) to handle audio. This creates operational complexity, high memory usage, and latency due to WebSocket IPC overhead. 

Lavende embeds a native Rust audio engine directly into your bot process using Foreign Function Interfaces (FFI) or CGO. 

### Benefits
- **Zero Network Overhead**: Audio logic runs in the same process as your bot.
- **Minimal Footprint**: Operates on a fraction of the memory required by a JVM.
- **Advanced Native DSP**: Real-time audio filters (Nightcore, Vaporwave, Bassboost, Equalizer, 3D Rotation) processed natively without requiring `ffmpeg`.
- **Identical API Surface**: The API is standardized across Node.js, Python, and Golang. Switching languages does not require re-learning the audio library.

Choose your language above to get started.
