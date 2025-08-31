use color_eyre::eyre::{Result, eyre};
use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use log::info;
use once_cell::sync::Lazy;
use rand::{Rng, distr::Alphanumeric};
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use rust_ffmpeg::{FFmpegProcess, prelude::*};
use std::{
    fs::File,
    ops::{Add, Sub},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, mpsc::Sender},
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;

use crate::{message::Message, model::Model};

pub struct Track {
    pub path: Option<PathBuf>,
    pub pos: Option<Duration>,
    pub duration: Option<Duration>,
    pub tagged_file: Option<TaggedFile>,
    pub has_metadata: bool,
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Playing,
    Paused,
    Idle,
}

const RODIO_SUPPORTED_FORMATS: [&str; 4] = ["flac", "mp3", "ogg", "wav"];
const TESTED_FORMATS: [&str; 6] = ["mp3", "flac", "wav", "ogg", "opus", "oga"];
const UNTESTED_FORMATS: [&str; 5] = ["pcm", "aiff", "aac", "wma", "alac"];
pub const AUDIO_FORMATS: [&str; 11] = [
    "mp3", "flac", "wav", "ogg", "opus", "oga", "pcm", "aiff", "aac", "wma", "alac",
];
const TEMP_FILE: &str = "firefly_temp";

pub static CONVERTED_TRACK: Lazy<PathBuf> = Lazy::new(|| {
    if Path::new(format!("{TEMP_FILE}.flac").as_str()).exists() {
        let rng = rand::rng();
        let rand_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        PathBuf::from(format!("{TEMP_FILE}_{rand_string}.flac"))
    } else {
        PathBuf::from(format!("{TEMP_FILE}.flac"))
    }
});

pub fn is_rodio_supported(path: &Path) -> Result<bool> {
    if path.is_file() {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if RODIO_SUPPORTED_FORMATS.contains(&extension) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(eyre!("file has no extension"))
        }
    } else {
        Err(eyre!("path is not a file"))
    }
}

pub fn get_sink() -> Result<(OutputStream, Sink)> {
    let mut stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    stream_handle.log_on_drop(false);

    Ok((stream_handle, sink))
}

pub fn get_source(track: PathBuf) -> Result<Decoder<File>> {
    let file = File::open(track)?;
    let source = Decoder::new(file)?;

    Ok(source)
}

pub fn choose_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_file()
}

pub fn choose_multiple_files() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_files()
}

pub fn choose_dir() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

pub fn load_track(sink: &mut Sink, track: &Path) -> Result<()> {
    let mut track_temp = track.to_path_buf();
    if !is_rodio_supported(&track_temp)? {
        track_temp = CONVERTED_TRACK.clone();
    }

    let source = get_source(track_temp).expect("Error obtaining source");

    sink.clear();
    sink.append(source);
    sink.play();

    Ok(())
}

pub fn increase_volume(sink: &mut Sink, amount: f32) {
    let current_vol = sink.volume();
    let increased_vol = f32::min(current_vol + amount, 2.0);
    sink.set_volume(increased_vol);
}

pub fn decrease_volume(sink: &mut Sink, amount: f32) {
    let current_vol = sink.volume();
    let decreased_vol = f32::max(current_vol - amount, 0.0);
    sink.set_volume(decreased_vol);
}

pub fn forward(sink: &mut Sink, track_dur: &Duration, forward_dur: Duration) {
    let current_pos = sink.get_pos();
    if current_pos.add(forward_dur) < *track_dur {
        sink.try_seek(current_pos.add(forward_dur))
            .expect("Error forwarding");
    } else if track_dur.sub(current_pos) < forward_dur
        && track_dur.sub(current_pos) > Duration::from_secs(1)
    {
        sink.try_seek(track_dur.sub(Duration::from_secs(1)))
            .expect("Error seeking");
    }
}

pub fn rewind(sink: &mut Sink, track: &Path, rewind_dur: Duration) -> Result<()> {
    let mut path = track.to_path_buf();
    if !is_rodio_supported(&path)? {
        path = CONVERTED_TRACK.clone();
    }

    let current_pos = sink.get_pos();
    let rewinded_pos = match current_pos.checked_sub(rewind_dur) {
        Some(dur) => dur,
        None => {
            sink.clear();
            let source = get_source(path)?;
            sink.append(source);
            sink.play();

            return Ok(());
        }
    };

    sink.clear();
    let source = get_source(path)?;
    sink.append(source);

    sink.try_seek(rewinded_pos).expect("Error rewinding");

    sink.play();

    Ok(())
}

pub fn get_track_duration(track: &Path) -> Result<Duration> {
    let mut temp_path = track.to_path_buf();
    if !is_rodio_supported(&temp_path)? {
        temp_path = CONVERTED_TRACK.clone();
    }

    let tagged_file = Probe::open(temp_path)?.read()?;

    Ok(tagged_file.properties().duration())
}

pub async fn convert_format(track_path: &Path) -> FFmpegProcess {
    FFmpegBuilder::convert(track_path.to_path_buf(), CONVERTED_TRACK.clone())
        .audio_filter(AudioFilter::loudnorm())
        .spawn()
        .await
        .unwrap()
}

pub fn load_now(
    model: &mut Model,
    path: PathBuf,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Result<()> {
    if path.is_file() {
        model.track_queue.prepend_track(path);
        try_next_track(model, msg_tx, info_tx)?;
    }

    Ok(())
}

pub fn bg_conversion(path: &Path, msg_tx: &Sender<Message>, info_tx: &Sender<String>) {
    let path = path.to_path_buf();
    let cloned_msg_tx = msg_tx.clone();
    let cloned_info_tx = info_tx.clone();

    info!("Converting file {}.", path.display());
    cloned_info_tx
        .send("Converting format and normalizing volume...".to_string())
        .unwrap();

    thread::spawn(move || {
        let runtime = Runtime::new().unwrap();
        let ffmpeg_handle = Arc::new(Mutex::new(runtime.block_on(convert_format(&path))));
        cloned_msg_tx
            .send(Message::ConversionStarted(ffmpeg_handle.clone()))
            .unwrap();
        loop {
            if ffmpeg_handle.lock().unwrap().try_wait().unwrap().is_some() {
                if ffmpeg_handle
                    .lock()
                    .unwrap()
                    .try_wait()
                    .unwrap()
                    .unwrap()
                    .success()
                {
                    cloned_info_tx.send("".to_string()).unwrap();
                    cloned_msg_tx.send(Message::ConversionEnded).unwrap();
                }
                cloned_info_tx.send("".to_string()).unwrap();
                info!("Conversion killed.");
                break;
            }
        }
    });
}

pub fn try_next_track(
    model: &mut Model,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Result<()> {
    let next_track = match model.track_queue.dequeue() {
        Some(path) => path,
        None => return Ok(()),
    };

    model.current_track.path = Some(next_track.clone());

    match is_rodio_supported(&next_track) {
        Ok(false) => {
            bg_conversion(&next_track, msg_tx, info_tx);

            return Ok(());
        }
        Ok(true) => {}
        Err(e) => {
            log::error!("{}", e);
            return Err(e);
        }
    }

    play_next_track(model, &next_track)?;

    Ok(())
}

pub fn play_next_track(model: &mut Model, next_track: &Path) -> Result<()> {
    if let Err(e) = load_track(&mut model.sink, next_track) {
        log::error!("{}", e);
        return Err(e);
    };

    model.current_track.tagged_file = Some(get_metadata(&next_track.to_path_buf())?);
    model.current_track.has_metadata = model
        .current_track
        .tagged_file
        .as_ref()
        .and_then(|f| f.primary_tag())
        .and_then(|t| t.title())
        .is_some();
    model.current_track.duration = model
        .current_track
        .path
        .as_ref()
        .and_then(|p| get_track_duration(p).ok());

    Ok(())
}

pub fn get_metadata(track: &PathBuf) -> Result<TaggedFile> {
    match Probe::open(track)?.read() {
        Ok(f) => Ok(f),
        Err(_) => Ok(Probe::open(CONVERTED_TRACK.clone())?.read()?),
    }
}
