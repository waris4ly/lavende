# Filters & DSP

One of Lavende's most powerful features is its internal Digital Signal Processing (DSP) engine. Unlike older bot implementations that spin up external FFmpeg processes to apply audio filters (causing high CPU usage), Lavende manipulates the audio stream natively in Rust.

## Using the FilterManager

Every player has a `FilterManager` attached to it. Changes made here apply instantly to the active audio stream.

### 15-Band Equalizer

Boost or cut specific frequency bands (ranging from `0` to `14`).

```javascript
// Example: Bassboost
player.filterManager.equalizerBands = [
    { band: 0, gain: 0.25 }, // Low frequencies
    { band: 1, gain: 0.20 },
    { band: 2, gain: 0.10 }
];
await player.filterManager.applyPlayerFilters();
```

### Nightcore (Speed & Pitch)

Adjust the time-stretching and pitch-shifting without affecting each other.

```javascript
// Speed up the track and increase the pitch
await player.filterManager.setSpeed(1.2);
await player.filterManager.setPitch(1.3);

// Track state for your own logic
player.filterManager.filters.nightcore = true;
```

### Vaporwave (Slowed + Reverb emulation)

Slow down the track and lower the pitch to create a vaporwave aesthetic.

```javascript
await player.filterManager.setSpeed(0.85);
await player.filterManager.setPitch(0.8);
```

### 3D Audio Rotation

Create an 8D-audio effect by panning the sound around the listener. The parameter dictates the Hz (speed of rotation).

```javascript
await player.filterManager.toggleRotation(0.3);
```

### Stereo / Mono output

Force the audio output channels.

```javascript
await player.filterManager.setAudioOutput('mono');
await player.filterManager.setAudioOutput('stereo'); // default
```

## Resetting Filters

To instantly clear all active DSP effects and return to the raw stream:

```javascript
await player.filterManager.resetFilters();
```
