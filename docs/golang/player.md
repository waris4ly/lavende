# Player & Queue

The Player handles the state of an active audio session in a specific guild.

## Creating a Player

When a user initiates a command, check `manager.Players`. If it doesn't exist, allocate one.

```go
var player *lavende.Player

if pVal, ok := manager.Players.Load(guildID); ok {
    player = pVal.(*lavende.Player)
} else {
    sd := true
    player = manager.CreatePlayer(lavende.PlayerOptions{
        GuildId:        guildID,
        VoiceChannelId: voiceChannelID,
        TextChannelId:  &textChannelID,
        SelfDeaf:       &sd,
        Volume:         100,
    })
    
    // Register events
    player.On("trackStart", onTrackStart)
    player.On("trackEnd", onTrackEnd)
    player.On("queueEnd", onQueueEnd)
}
```

## Searching Audio

Use the resolver to query APIs. It returns a structured response indicating if it found a single track, a playlist, or nothing.

```go
res, err := player.Search("query or URL", requester)

if err != nil || res == nil || res.LoadType == "empty" {
    fmt.Println("No tracks found.")
    return
}

if res.LoadType == "playlist" {
    player.Queue.AddMultiple(res.Tracks, nil)
} else {
    player.Queue.AddSingle(res.Tracks[0], nil)
}
```

## Playing

If the player isn't actively streaming, connect to the voice channel and initiate playback.

```go
if !player.Playing {
    player.Connect()
    player.Play(nil)
}
```

## Controls

Control methods mutate the internal C state of the Rust Core seamlessly.

```go
player.Pause(true)
player.Resume()
player.Skip()
player.Destroy(nil)
player.Seek(30000)
player.SetVolumeObj(50)
```

## Events

The dispatcher dynamically casts arguments. Define handlers that explicitly cast to the known pointer types.

```go
func onTrackStart(args ...interface{}) {
    player := args[0].(*lavende.Player)
    track := args[1].(*lavende.Track)
    // Handle track start
}
```
