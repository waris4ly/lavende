# Changelog

Each release includes various fixes, improvements, and new features.
The most noteworthy changes, features, and breaking changes are documented here.

## [v1.0.6] - 2026-07-14
### Changed
- Complete refactoring of all legacy monolithic audio source files into clean, modular subdirectories (api, extractor, track, token, reader, etc.) to improve maintainability and decouple codebase components.
- Rewrote let-chain patterns to standard nested conditional patterns to enhance stability and ensure compatibility with standard Rust compilation profiles.

## [v1.0.5] - 2026-07-12
### Added
- Integrated lyrics engine with support for 8 lyrics providers: Genius, LRCLib, Deezer, Musixmatch, Letras.mus, NetEase, Yandex, and YouTube Music.
- Added wrapper functions `loadLyrics`/`load_lyrics` and `loadLyricsBySearch`/`load_lyrics_by_search` in Node.js, Go, Python, and Rust SDKs.
- Added `getLyrics`/`get_lyrics` method to high-level `Player` struct/class in all SDK wrappers.
- Added default `"lyrics"` configurations block to `source.json` files in examples.
- Added a `!lyrics` Discord command to the Node.js example bot.

