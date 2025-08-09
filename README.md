# Terminal Audio Player
Written in Rust with audio playback handled by [Rodio](https://github.com/RustAudio/rodio) and [Ratatui](https://ratatui.rs/) for interface.
![example_img](example_img/firefly_1.png)
## Features
- Play, Pause, Rewind, and Forward.
- Volume control from 0-200%
- Track Looping
- File Dialog

### Formats supported by Rodio
- FLAC
- MP3
- Vorbis (ogg)
- WAV

It can still play other formats by converting formats not supported by Rodio to FLAC using [rust_ffmpeg](https://github.com/RustNSparks/ffmpeg-suite-rs).
### Tested Converted Formats
- Opus
- OGA

### Planned Features
- [ ] Track Queuing
- [ ] Playlists
- [ ] Music Library
