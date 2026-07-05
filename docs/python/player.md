# Player & Queue

A `Player` tracks the audio session of a specific Discord Guild.

## Creating a Player

When a command is received, fetch or instantiate the player.

```python
player = manager.players.get(guild_id)

if not player:
    player = manager.create_player({
        "guild_id": guild_id,
        "voice_channel_id": voice_channel_id,
        "text_channel_id": text_channel_id,
        "volume": 100
    })

    # Event hooks
    player.on('track_start', on_track_start)
    player.on('track_end', on_track_end)
    player.on('queue_end', on_queue_end)
    player.on('error', on_error)
```

## Resolving Audio

Pass a URL or search query to the `search` method.

```python
res = await player.search("URL or Query", requester_object)
load_type = res.get("loadType")
tracks = res.get("tracks", [])

if load_type == "empty" or not tracks:
    return print("No tracks found.")

if load_type == "playlist":
    player.queue.add(tracks)
else:
    player.queue.add(tracks[0])
```

## Starting Playback

If the player is currently idle, trigger the connect and play flow.

```python
if not player.playing:
    await player.connect()
    await player.play()
```

## Controlling the Stream

The `Player` provides intuitive asynchronous control methods.

```python
await player.pause(True)
await player.resume()
await player.skip()
await player.destroy()
await player.seek(30000)
await player.set_volume(50)
```

## Handling Events

Lavende triggers callbacks asynchronously. 

```python
async def on_track_start(player, track):
    print(f"Now playing: {track.info.title}")

async def on_track_end(player, track, reason):
    print(f"Finished playing {track.info.title}. Reason: {reason}")

async def on_queue_end(player):
    print("Queue ended.")
    await player.destroy()
```
