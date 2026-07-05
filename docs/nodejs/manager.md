# The Manager

The `LavendeManager` is the central component of the library. It acts as the bridge between Discord's Gateway and the native Rust audio engine.

## Initialization

You must instantiate the manager once your bot is ready. It requires your bot's client ID, username, and a callback function to send Voice State Update packets back to the Discord gateway.

```javascript
const { LavendeManager } = require('lavende');

client.once('ready', () => {
    manager = new LavendeManager({
        sendToShard: (guildId, payload) => {
            // Find the shard handling this guild and send the payload
            client.guilds.cache.get(guildId)?.shard?.send(payload);
        },
        client: {
            id: client.user.id,
            username: client.user.username
        }
    });
    
    manager.init();
});
```

## Routing Gateway Events

Lavende does not connect to the Discord Gateway on its own. It relies on your bot to forward raw WebSocket voice events. Specifically, it needs `VOICE_STATE_UPDATE` and `VOICE_SERVER_UPDATE` to establish a UDP connection.

Pipe raw events from your client into the manager:

```javascript
client.on('raw', (packet) => {
    if (manager) {
        manager.sendRawData(packet);
    }
});
```

Without this step, the Rust core will never receive the endpoint and token required to connect to the voice server, and playback will silently hang.
