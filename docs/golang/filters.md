# Filters & DSP

Lavende for Go uses a CGO boundary to execute complex DSP algorithms inside the Rust core, providing lightning-fast audio manipulation.

## Filter Manager

The `FilterManager` is accessible from the `Player` struct.

### Equalizers

Apply granular EQ tuning.

```go
player.FilterManager.EqualizerBands = []lavende.EqBand{
    {Band: 0, Gain: 0.25}, 
    {Band: 1, Gain: 0.15},
}
player.FilterManager.ApplyPlayerFilters()
```

### Nightcore & Vaporwave (Time Stretching)

Time stretching manipulates the playback speed and pitch simultaneously or independently.

```go
// Nightcore Example
player.FilterManager.SetSpeed(1.18)
player.FilterManager.SetPitch(1.3)

// Vaporwave Example
player.FilterManager.SetSpeed(0.85)
player.FilterManager.SetPitch(0.8)
```

### 3D Spatial Audio

Simulates sound rotating around the listener's head.

```go
player.FilterManager.ToggleRotation(0.3)
```

### Resetting

Resetting the filters will wipe the state from the Rust engine immediately.

```go
player.FilterManager.ResetFilters()
```
