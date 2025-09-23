use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::time::Duration;

use color_eyre::eyre::Result;
use ogg::reading::PacketReader;
use opus::{Channels as OpusChannels, Decoder as OpusDecoder};
use rodio::source::Source;

// This file was vibe-coded.

// A Source that decodes Ogg-Opus packets on-the-fly and yields f32 samples.
pub struct OpusOggSource<R: Read + Seek> {
    packet_reader: PacketReader<R>,
    decoder: OpusDecoder,
    channels: u16,
    sample_rate: u32,
    // decoded samples (interleaved) awaiting iteration
    decoded_buf: Vec<f32>,
    decoded_pos: usize,
    finished: bool,
}

impl<R: Read + Seek> OpusOggSource<R> {
    /// Create from a reader (e.g. BufReader<File>).
    /// This will read the OpusHead and OpusTags header packets and initialize the decoder.
    pub fn new(reader: R) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut pr = PacketReader::new(reader);

        // Read ID header (OpusHead)
        let id_packet = pr.read_packet()?.ok_or("unexpected EOF reading OpusHead")?;
        if id_packet.data.len() < 10 || &id_packet.data[0..8] != b"OpusHead" {
            return Err("not an Ogg Opus stream (missing OpusHead)".into());
        }
        // OpusHead layout: "OpusHead" (8 bytes), version (1), channel_count (1), ...
        let channels = id_packet.data[9] as u16;
        // Skip comment header (OpusTags)
        let _tags = pr.read_packet()?; // ignore contents

        // Opus decoders always operate at 48 kHz internally
        let sample_rate = 48_000;
        let opus_channels = if channels == 1 {
            OpusChannels::Mono
        } else {
            OpusChannels::Stereo
        };

        let decoder = OpusDecoder::new(sample_rate, opus_channels)?;

        Ok(Self {
            packet_reader: pr,
            decoder,
            channels,
            sample_rate,
            decoded_buf: Vec::new(),
            decoded_pos: 0,
            finished: false,
        })
    }

    /// Fill decoded_buf with the next decoded packet's samples (interleaved).
    fn decode_next_packet(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        loop {
            match self.packet_reader.read_packet()? {
                Some(pkt) => {
                    if pkt.data.is_empty() {
                        continue;
                    }
                    // Determine number of samples per channel in this packet (optional; used for sizing)
                    let nb = match self.decoder.get_nb_samples(&pkt.data) {
                        Ok(n) => n,
                        Err(_) => 960, // fallback
                    };
                    let needed = nb * (self.channels as usize);
                    let mut out = vec![0.0f32; needed];

                    // decode_float returns number of samples per channel decoded
                    let decoded = self.decoder.decode_float(&pkt.data, &mut out, false)?;
                    let total = decoded * (self.channels as usize);
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

impl<R: Read + Seek> Iterator for OpusOggSource<R> {
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

        // decode next packet; if none -> end
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

impl<R: Read + Seek> Source for OpusOggSource<R> {
    fn current_span_len(&self) -> Option<usize> {
        // number of samples left in current decoded buffer
        let rem = self.decoded_buf.len().saturating_sub(self.decoded_pos);
        Some(rem).filter(|&v| v != 0) // should not return 0 unless no more data
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None // unknown for streaming / unless you scan the whole file
    }
}

pub fn get_opus_source(file: File) -> Box<dyn Source<Item = f32> + Send> {
    let reader = BufReader::new(file);
    let source = OpusOggSource::new(reader).unwrap();

    Box::new(source)
}
