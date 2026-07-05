# The Manager

The `LavendeManager` acts as the primary controller for your Go audio sessions.

## Initialization

You instantiate the manager by providing a configuration struct. Crucially, you must inject a function that takes a raw payload and sends it back to the Discord Gateway.

```go
import (
    "github.com/bwmarrin/discordgo"
    "lavende"
)

var manager *lavende.LavendeManager

func initAudio(s *discordgo.Session) {
    opts := lavende.LavendeManagerOptions{
        SendToShard: func(guildId string, payload interface{}) {
            // Write payload to websocket
            _ = s.GatewayWriteStruct(payload)
        },
    }
    opts.Client.Id = s.State.User.ID
    opts.Client.Username = &s.State.User.Username

    manager = lavende.NewLavendeManager(opts)
    manager.Init(nil)
}
```

## Routing Gateway Events

Because Lavende operates entirely within your bot process, it needs access to Voice state updates. If using `discordgo`, you simply listen to the respective events and marshal them into `manager.SendRawData`.

```go
s.AddHandler(func(s *discordgo.Session, v *discordgo.VoiceStateUpdate) {
    if manager != nil && v.UserID == s.State.User.ID {
        manager.SendRawData(map[string]interface{}{
            "t": "VOICE_STATE_UPDATE",
            "d": map[string]interface{}{
                "user_id":    v.UserID,
                "guild_id":   v.GuildID,
                "session_id": v.SessionID,
                "channel_id": v.ChannelID,
            },
        })
    }
})

s.AddHandler(func(s *discordgo.Session, v *discordgo.VoiceServerUpdate) {
    if manager != nil {
        manager.SendRawData(map[string]interface{}{
            "t": "VOICE_SERVER_UPDATE",
            "d": map[string]interface{}{
                "guild_id": v.GuildID,
                "token":    v.Token,
                "endpoint": v.Endpoint,
            },
        })
    }
})
```

Once the Rust core receives both the state update and the server update, it automatically establishes the UDP socket and performs the secret key exchange with Discord.
