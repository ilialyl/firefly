use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Duration;

use color_eyre::eyre::Result;
use ogg::reading::PacketReader;
use opus::{Channels as OpusChannels, Decoder as OpusDecoder};
use rodio::source::{SeekError, Source};

// This file was vibe-coded.

// opus_seek_example.rs
// A simple Ogg-Opus decoder + rodio Source with a basic (linear) seek implementation.
//
// Notes:
// - This implementation requires creating the source from a filesystem path
//   (OpusOggSource::from_path). Seeking is implemented by reopening the file,
//   decoding from the start and skipping decoded samples until the target.
//   That makes seeking correct but linear in time (not fast random access).
// - This example does NOT implement full RFC7845 mapping-family handling or
//   pre-skip trimming — it's suitable for many common .opus files (mono/stereo).
// - Use OpusOggSource::seek_seconds(seconds) to seek to a time before appending
//   to a rodio::Sink. Seeking while the source is owned by a Sink is possible
//   only if you control playback (remove/reappend).

/// A Source that decodes Ogg-Opus packets on-the-fly and supports a simple seek.
pub struct OpusOggSource {
    path: PathBuf,
    packet_reader: PacketReader<BufReader<File>>,
    decoder: OpusDecoder,
    channels: u16,
    sample_rate: u32, // always 48000 for Opus decoding
    // decoded samples (interleaved) awaiting iteration
    decoded_buf: Vec<f32>,
    decoded_pos: usize,
    finished: bool,
}

impl OpusOggSource {
    /// Create from a path. We read the OpusHead and OpusTags and initialize decoder state.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut pr = PacketReader::new(reader);

        // Read ID header (OpusHead)
        let id_packet = pr.read_packet()?.ok_or("unexpected EOF reading OpusHead")?;
        if id_packet.data.len() < 10 || &id_packet.data[0..8] != b"OpusHead" {
            return Err("not an Ogg Opus stream (missing OpusHead)".into());
        }
        let channels = id_packet.data[9] as u16;

        // Skip comment header (OpusTags)
        let _ = pr.read_packet()?;

        // Opus decoders operate at 48 kHz internally
        let sample_rate = 48_000;
        let opus_channels = if channels == 1 {
            OpusChannels::Mono
        } else {
            OpusChannels::Stereo
        };

        let decoder = OpusDecoder::new(sample_rate, opus_channels)?;

        Ok(Self {
            path,
            packet_reader: pr,
            decoder,
            channels,
            sample_rate,
            decoded_buf: Vec::new(),
            decoded_pos: 0,
            finished: false,
        })
    }

    /// Decode the next available packet and fill decoded_buf.
    fn decode_next_packet(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        loop {
            match self.packet_reader.read_packet()? {
                Some(pkt) => {
                    if pkt.data.is_empty() {
                        continue;
                    }

                    // Estimate number of samples per channel for sizing (best-effort)
                    let nb = self.decoder.get_nb_samples(&pkt.data).unwrap_or(960);

                    let needed = nb * (self.channels as usize);
                    let mut out = vec![0.0f32; needed];

                    // decode_float returns number of samples per channel decoded
                    let decoded_per_chan = self.decoder.decode_float(&pkt.data, &mut out, false)?;
                    let total = decoded_per_chan * (self.channels as usize);
                    out.truncate(total);

                    self.decoded_buf = out;
                    self.decoded_pos = 0;
                    return Ok(true);
                }
                None => {
                    // EOF
                    self.finished = true;
                    return Ok(false);
                }
            }
        }
    }
}

impl Iterator for OpusOggSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.decoded_pos < self.decoded_buf.len() {
            let v = self.decoded_buf[self.decoded_pos];
            self.decoded_pos += 1;
            return Some(v);
        }

        if self.finished {
            return None;
        }

        match self.decode_next_packet() {
            Ok(true) => {
                if self.decoded_pos < self.decoded_buf.len() {
                    let v = self.decoded_buf[self.decoded_pos];
                    self.decoded_pos += 1;
                    Some(v)
                } else {
                    None
                }
            }
            Ok(false) | Err(_) => None,
        }
    }
}

impl Source for OpusOggSource {
    fn current_span_len(&self) -> Option<usize> {
        let rem = self.decoded_buf.len().saturating_sub(self.decoded_pos);
        Some(rem).filter(|&v| v != 0)
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }

    fn try_seek(&mut self, pos: Duration) -> std::result::Result<(), rodio::source::SeekError> {
        let seconds = pos.as_secs_f64();
        let target_samples_per_chan = (seconds * (self.sample_rate as f64)).round() as usize;

        let file = File::open(&self.path).map_err(|_| SeekError::NotSupported {
            underlying_source: "open failed",
        })?;
        let reader = BufReader::new(file);
        let mut pr = PacketReader::new(reader);

        let id_packet = pr
            .read_packet()
            .map_err(|_| SeekError::NotSupported {
                underlying_source: "read header",
            })?
            .ok_or(SeekError::NotSupported {
                underlying_source: "eof header",
            })?;
        if id_packet.data.len() < 10 || &id_packet.data[0..8] != b"OpusHead" {
            return Err(SeekError::NotSupported {
                underlying_source: "bad head",
            });
        }
        let channels = id_packet.data[9] as u16;
        let _ = pr.read_packet();

        let sample_rate = 48_000;
        let opus_channels = if channels == 1 {
            OpusChannels::Mono
        } else {
            OpusChannels::Stereo
        };

        let mut decoder =
            OpusDecoder::new(sample_rate, opus_channels).map_err(|_| SeekError::NotSupported {
                underlying_source: "decoder",
            })?;

        let mut cum_samples_per_chan: usize = 0;
        loop {
            match pr.read_packet() {
                Ok(Some(pkt)) => {
                    if pkt.data.is_empty() {
                        continue;
                    }
                    let nb = decoder.get_nb_samples(&pkt.data).unwrap_or(960);
                    let mut out = vec![0.0f32; nb * (channels as usize)];
                    let decoded_per_chan = decoder
                        .decode_float(&pkt.data, &mut out, false)
                        .map_err(|_| SeekError::NotSupported {
                            underlying_source: "decode",
                        })?;
                    let total = decoded_per_chan * (channels as usize);
                    out.truncate(total);

                    if cum_samples_per_chan + (decoded_per_chan as usize) > target_samples_per_chan
                    {
                        let start_in_packet =
                            target_samples_per_chan.saturating_sub(cum_samples_per_chan);
                        let start_index = start_in_packet * (channels as usize);
                        self.decoded_buf = if start_index < out.len() {
                            out[start_index..].to_vec()
                        } else {
                            Vec::new()
                        };
                        self.decoded_pos = 0;
                        self.packet_reader = pr;
                        self.decoder = decoder;
                        self.channels = channels;
                        self.sample_rate = sample_rate;
                        self.finished = false;
                        return Ok(());
                    } else {
                        cum_samples_per_chan += decoded_per_chan as usize;
                    }
                }
                Ok(None) => {
                    self.decoded_buf.clear();
                    self.decoded_pos = 0;
                    self.packet_reader = pr;
                    self.decoder = decoder;
                    self.channels = channels;
                    self.sample_rate = sample_rate;
                    self.finished = true;
                    return Ok(());
                }
                Err(_) => {
                    return Err(SeekError::NotSupported {
                        underlying_source: "packet read",
                    });
                }
            }
        }
    }
}

pub fn get_opus_source(path: &Path) -> Box<dyn Source<Item = f32> + Send> {
    let source = OpusOggSource::from_path(path).unwrap();

    Box::new(source)
}
