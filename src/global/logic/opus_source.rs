use std::{collections::VecDeque, fs::File, num::NonZero, path::Path, sync::Arc, time::Duration};

use color_eyre::eyre::Result;
use rodio::{ChannelCount, SampleRate, Source, decoder::symphonia::SeekError, source};
use symphonia::core::{
    audio::{AudioBufferRef, Signal},
    codecs::{CODEC_TYPE_NULL, CodecRegistry, Decoder, DecoderOptions},
    errors::Error,
    formats::{FormatOptions, FormatReader, SeekMode, SeekTo},
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
    units::Time,
};
use symphonia_adapter_libopus::OpusDecoder;

pub struct OpusSource {
    channels: usize,
    sample_rate: u32,
    buffer: VecDeque<f32>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    format: Box<dyn FormatReader>,
    duration: Option<Duration>,
}

impl OpusSource {
    pub fn new(path: &Path) -> Result<Self> {
        let src = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe().format(
            Hint::new().with_extension("opus"),
            mss,
            &fmt_opts,
            &meta_opts,
        )?;

        let format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .unwrap();

        let dec_opts: DecoderOptions = Default::default();

        let mut codec_registry = CodecRegistry::new();
        codec_registry.register_all::<OpusDecoder>();

        let decoder = codec_registry.make(&track.codec_params, &dec_opts)?;

        let track_id = track.id;

        let duration = match (track.codec_params.n_frames, track.codec_params.sample_rate) {
            (Some(frames), Some(rate)) => {
                Some(Duration::from_secs_f64(frames as f64 / rate as f64))
            }
            _ => None,
        };

        Ok(OpusSource {
            channels: track.codec_params.channels.unwrap().count(),
            sample_rate: track.codec_params.sample_rate.unwrap(),
            buffer: VecDeque::new(),
            decoder,
            track_id,
            format,
            duration,
        })
    }
}

impl Iterator for OpusSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(sample) = self.buffer.pop_front() {
                return Some(sample);
            }

            let packet = match self.format.next_packet() {
                Ok(p) => p,
                Err(_) => return None, // The last packet.
            };

            while !self.format.metadata().is_latest() {
                self.format.metadata().pop();
            }

            if packet.track_id() != self.track_id {
                continue;
            }

            match self.decoder.decode(&packet) {
                Ok(decoded) => match decoded {
                    AudioBufferRef::F32(buf) => {
                        for i in 0..buf.frames() {
                            for c in 0..self.channels {
                                self.buffer.push_back(buf.chan(c)[i]);
                            }
                        }
                    }
                    _ => {
                        unimplemented!()
                    }
                },
                Err(Error::IoError(_)) => continue,
                Err(Error::DecodeError(_)) => continue,
                Err(e) => {
                    // An unrecoverable error.
                    panic!("{}", e);
                }
            }
        }
    }
}

impl Source for OpusSource {
    fn channels(&self) -> ChannelCount {
        NonZero::new(self.channels as u16).unwrap()
    }

    fn sample_rate(&self) -> SampleRate {
        NonZero::new(self.sample_rate).unwrap()
    }

    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.duration
    }

    fn try_seek(&mut self, pos: Duration) -> std::result::Result<(), rodio::source::SeekError> {
        self.buffer.clear();

        match self.format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time: Time::from(pos),
                track_id: Some(self.track_id),
            },
        ) {
            Err(Error::SeekError(symphonia::core::errors::SeekErrorKind::ForwardOnly)) => {
                return Err(source::SeekError::SymphoniaDecoder(
                    SeekError::RandomAccessNotSupported,
                ));
            }
            other => other.map_err(Arc::new).map_err(SeekError::Demuxer),
        }?;

        self.decoder.reset();

        Ok(())
    }
}
