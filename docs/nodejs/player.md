# Player & Queue

A `Player` represents a guild's voice connection and audio session. 

## Creating a Player

To start playing audio in a guild, create a player instance via the `manager`.

```javascript
let player = manager.players.get(guildId);

if (!player) {
    player = manager.createPlayer({
        guildId: guildId,
        voiceChannelId: voiceChannelId,
        textChannelId: textChannelId,
        volume: 100 // 0 to 1000
    });
}
```

## Resolving Audio

Lavende has a built-in resolver that queries APIs (like YouTube) natively. Use the `search` method to retrieve tracks.

```javascript
const result = await player.search("URL or Query", message.author);

if (result.loadType === 'empty') {
    return console.log("No results.");
}

// Add to the internal queue
if (result.loadType === 'playlist') {
    player.queue.add(result.tracks);
} else {
    player.queue.add(result.tracks[0]);
}
```

## Playback Execution

Once tracks are in the queue, initialize the connection and trigger playback.

```javascript
if (!player.playing) {
    await player.connect();
    await player.play();
}
```

## Controlling the Stream

The `Player` object provides standard methods to control the stream lifecycle:

```javascript
await player.pause(true);   // Pause
await player.resume();      // Resume
await player.skip();        // Skip current track
await player.destroy();     // Disconnect and wipe queue
await player.seek(30000);   // Seek to 30 seconds
await player.setVolume(50); // Adjust volume on the fly
```

## Event Callbacks

Listen to player events to notify users in text channels.

```javascript
player.on('trackStart', (p, track) => {
    console.log(`Started: ${track.info.title}`);
});

player.on('trackEnd', (p, track, reason) => {
    // Reason could be 'finished', 'stopped', 'replaced', etc.
    console.log(`Ended: ${track.info.title}`);
});

player.on('queueEnd', (p) => {
    p.destroy(); // Always clean up when the queue finishes
});

player.on('error', (p, error) => {
    console.error("Lavende Core Error:", error);
});
```
