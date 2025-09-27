# Firefly, Terminal Audio Player
Written in Rust with audio playback handled by [Rodio](https://github.com/RustAudio/rodio) and UI built with [Ratatui](https://ratatui.rs/).
![example_img](example_img/firefly_v0-6-0.gif)
(Showcase as of v0.6.0)
## Features (v0.6.1)
- Play, Pause, Rewind, and Seek.
- Persistent Playlists
- Volume control from 0-200%
- Track Looping
- File Dialog
- Track and Directory queuing
- Queue arrangement
- Queue Shuffle
- Skip Forward or Backward
- Metadata display (Title, Artist, Album, Year, Bit Depth, Sample Rate, Bitrate)

### Formats Supported
- FLAC ([Symphonia](https://github.com/pdeljanov/Symphonia))
- MP3 ([Symphonia](https://github.com/pdeljanov/Symphonia))
- WAV ([Symphonia](https://github.com/pdeljanov/Symphonia))
- Opus ([opus-rs](https://github.com/SpaceManiac/opus-rs))

It can still play other formats by converting formats not supported by Rodio to FLAC using [rust_ffmpeg](https://github.com/RustNSparks/ffmpeg-suite-rs).

Temporary FLAC files stay on disk for reuse, which can be cleared by running `cargo run --release -- clean` or `firefly clean`

### Planned Features
- [x] Playlists (v0.6.0)
- [ ] YTDLP integration
- [ ] Music Library
- [ ] Graphical user interface (GUI)

## Usage
### Windows
- [Prebuilt Binary](https://github.com/ilialyl/firefly/releases/latest) (Extract first)
#### Optional Dependencies
- [FFmpeg](https://ffmpeg.org/) (If you want to play unsupported file types.)

### Fedora Linux
Build from source.
#### Requirements
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [wayland-devel](https://packages.fedoraproject.org/pkgs/wayland/wayland-devel/)
- [alsa-lib-devel](https://packages.fedoraproject.org/pkgs/alsa-lib/alsa-lib-devel/)
- [opus-devel](https://packages.fedoraproject.org/pkgs/opus/opus-devel/)
- [FFmpeg](https://ffmpeg.org/) (only if the file type you want to play isn't listed above)
#### Steps
1. Clone this repository
2. run `cargo run --release --bin firefly`

### Others
Building from source is possible but instruction is omitted due to unknown list of dependencies.

## Known Issue
- Rewinding can be slow on systems that use [ALSA](https://www.alsa-project.org/wiki/Main_Page).

## Bug Report
If you find any bugs, you can open an [issue](https://github.com/ilialyl/firefly/issues). I will get to it as soon as possible.
