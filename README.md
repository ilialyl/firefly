# Firefly, Terminal Audio Player
Written in Rust with audio playback handled by [Rodio](https://github.com/RustAudio/rodio) and UI built with [Ratatui](https://ratatui.rs/).
![example_img](example_img/firefly_v0-10-0.gif)
(Showcase as of v0.10.0, using [Kitty](https://sw.kovidgoyal.net/kitty/) with Atelier Dune Dark theme.)
## Features (v0.11.0)
- Play, Pause, Rewind, and Seek.
- Persistent Playlists
- Volume Control from 0-200%
- Track Looping
- Pick Files from File Dialog or Command-Line Argument.
- Track and Directory queuing
- Queue Arrangement, Shuffling, Clearing
- Skip Forward or Backward
- Metadata Display
- [Linux] MPRIS Support: Media control with metadata, including cover art. KDE Connect compatible.

### Formats Supported
- FLAC ([Symphonia](https://github.com/pdeljanov/Symphonia))
- MP3 ([Symphonia](https://github.com/pdeljanov/Symphonia))
- WAV ([Symphonia](https://github.com/pdeljanov/Symphonia))
- Opus ([opus-rs](https://github.com/SpaceManiac/opus-rs))

It can still play other formats by converting formats not supported by Rodio to FLAC using [rust_ffmpeg](https://github.com/RustNSparks/ffmpeg-suite-rs).

Temporary FLAC files stay on disk for reuse, which can be cleared by running `firefly clean`

### Roadmap - open to suggestions
- [x] Playlists (v0.6.0)
- [x] Album art display for supported terminals (v0.7.0)
- [x] UI overhaul (v0.7.0)
- [x] Proper MPRIS control (v0.11.0)
- [ ] A special session where you can control an existing session (queue tracks, etc.). 

## Installation
### Windows and Linux
Download a [Prebuilt Binary](https://github.com/ilialyl/firefly/releases/latest) or use [cargo binstall](https://crates.io/crates/cargo-binstall) to automatically install it for you (requires [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)):
```
# Install cargo-binstall if not already
cargo install cargo-binstall

# Install firefly
cargo binstall firefly_music

# Launch the app
firefly
```
### Optional Dependencies
- [FFmpeg](https://ffmpeg.org/) - if you want to play unsupported file types.

## Build from source
### Fedora Linux
#### Dependencies
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [wayland-devel](https://packages.fedoraproject.org/pkgs/wayland/wayland-devel/)
- [alsa-lib-devel](https://packages.fedoraproject.org/pkgs/alsa-lib/alsa-lib-devel/)
- [opus-devel](https://packages.fedoraproject.org/pkgs/opus/opus-devel/)
- [FFmpeg](https://ffmpeg.org/) (only if the file type you want to play isn't listed above)
```
# Install dependencies
sudo dnf install wayland-devel alsa-lib-devel rust opus-devel cargo git

# Clone the repository
git clone https://github.com/ilialyl/firefly && cd firefly

# Build and run (optimized)
cargo run --release
```

## CLI Examples
```
# Launch with a single file or directory
firefly with example.mp3
firefly with music/

# Launch with multiple files or directories
firefly with example.mp3 music/

# Launch with multiple files using [wildcards](https://www.malikbrowne.com/blog/a-beginners-guide-glob-patterns/.) (Only on Bash and Z shell)
shopt -s globstar # enable globstar
firefly with ./**/*.opus
```

## Known Issue
- Seeking an OPUS track is slow.
- Wezterm (Windows) does not display cover art.
- Does not work with WSL as no audio output is connected.

## Bug Report
If you find any bugs, you can open an [issue](https://github.com/ilialyl/firefly/issues). I will get to it as soon as possible.
