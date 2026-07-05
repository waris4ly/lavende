# The Manager

The `LavendeManager` acts as the control plane for the library.

## Initialization

Setup the manager during your bot's startup phase (e.g., `on_ready`).

```python
import discord
from discord.ext import commands
from lavende import LavendeManager

bot = commands.Bot(command_prefix="!", intents=discord.Intents.all())
manager = None

async def send_to_shard(guild_id: str, payload: dict):
    guild = bot.get_guild(int(guild_id))
    shard_id = guild.shard_id if guild else 0
    ws = bot.shards.get(shard_id) if hasattr(bot, 'shards') else bot.ws
    if ws:
        await ws.send_as_json(payload)

@bot.event
async def on_ready():
    global manager
    manager = LavendeManager(
        send_to_shard=send_to_shard,
        client={"id": str(bot.user.id), "username": bot.user.name}
    )
    manager.init()
```

## Routing Gateway Events

Lavende must intercept raw Discord Voice socket events (`VOICE_STATE_UPDATE` and `VOICE_SERVER_UPDATE`).

```python
@bot.event
async def on_socket_raw_receive(msg):
    if manager:
        import json
        try:
            packet = json.loads(msg.decode('utf-8') if isinstance(msg, bytes) else msg)
            await manager.send_raw_data(packet)
        except Exception:
            pass
```

Without capturing these events, Lavende will not be able to identify the correct UDP endpoint to stream audio to.
