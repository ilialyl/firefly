use std::{collections::VecDeque, fs::File, path::Path, sync::LazyLock, time::Duration};

use color_eyre::eyre::{OptionExt, Result};
use rodio::{ChannelCount, SampleRate, Source, decoder::symphonia::SeekError, source};
use symphonia::core::{
    audio::SampleBuffer,
    codecs::{CODEC_TYPE_NULL, CodecRegistry, Decoder, DecoderOptions},
    errors::Error,
    formats::{FormatOptions, FormatReader, SeekMode, SeekTo},
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
    units::Time,
};
use symphonia_adapter_libopus::OpusDecoder;

static CODEC_REGISTRY: LazyLock<CodecRegistry> = LazyLock::new(|| {
    let mut r = CodecRegistry::new();
    r.register_all::<OpusDecoder>();
    r
});

pub struct OpusSource {
    channels: usize,
    sample_rate: u32,
    buffer: VecDeque<f32>,
    sample_buf: Option<SampleBuffer<f32>>,
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
            .ok_or_eyre("no track found")?;

        let dec_opts: DecoderOptions = Default::default();

        let decoder = CODEC_REGISTRY.make(&track.codec_params, &dec_opts)?;

        let track_id = track.id;

        let duration = match (track.codec_params.n_frames, track.codec_params.sample_rate) {
            (Some(frames), Some(rate)) => {
                Some(Duration::from_secs_f64(frames as f64 / rate as f64))
            }
            _ => None,
        };

        Ok(OpusSource {
            channels: track
                .codec_params
                .channels
                .ok_or_eyre("missing channel info")?
                .count(),
            sample_rate: track
                .codec_params
                .sample_rate
                .ok_or_eyre("missing sample rate info")?,
            sample_buf: None,
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
                Err(Error::IoError(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return None;
                }
                Err(e) => {
                    log::error!("OpusSource Error: {e}");
                    return None;
                }
            };

            while !self.format.metadata().is_latest() {
                self.format.metadata().pop();
            }

            if packet.track_id() != self.track_id {
                continue;
            }

            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    let buf = self.sample_buf.get_or_insert_with(|| {
                        SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec())
                    });

                    buf.copy_interleaved_ref(decoded);

                    self.buffer.extend(buf.samples().iter().copied());
                }
                Err(Error::IoError(_)) => continue,
                Err(Error::DecodeError(_)) => continue,
                Err(Error::ResetRequired) => {
                    self.sample_buf = None;
                    self.decoder.reset();
                    continue;
                }
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
        self.channels as u16
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
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
            other => other.map_err(SeekError::Demuxer),
        }?;

        self.decoder.reset();

        Ok(())
    }
}
