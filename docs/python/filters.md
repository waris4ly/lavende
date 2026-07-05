# Filters & DSP

Lavende's native core avoids relying on `ffmpeg` for audio manipulation, drastically reducing CPU cycles. The `FilterManager` updates the active audio stream instantly.

## Filter Manager

### Equalizers

```python
player.filter_manager.equalizer_bands = [
    {"band": 0, "gain": 0.25},
    {"band": 1, "gain": 0.30}
]
await player.filter_manager.apply_player_filters()
```

### Nightcore & Vaporwave

Use `set_speed` and `set_pitch` to manipulate audio playback.

```python
# Nightcore
await player.filter_manager.set_speed(1.18)
await player.filter_manager.set_pitch(1.3)

# Vaporwave
await player.filter_manager.set_speed(0.85)
await player.filter_manager.set_pitch(0.8)
```

### Spatial Audio

Create a 3D audio experience with rotation.

```python
await player.filter_manager.toggle_rotation(0.3)
```

### Reset Filters

```python
await player.filter_manager.reset_filters()
```
