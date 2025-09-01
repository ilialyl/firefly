use color_eyre::eyre::{Result, eyre};
use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use log::info;
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

use crate::{logic::playback_state::PlaybackState, message::Message};

const RODIO_SUPPORTED_FORMATS: [&str; 4] = ["flac", "mp3", "ogg", "wav"];
const TESTED_FORMATS: [&str; 6] = ["mp3", "flac", "wav", "ogg", "opus", "oga"];
const UNTESTED_FORMATS: [&str; 5] = ["pcm", "aiff", "aac", "wma", "alac"];
pub const AUDIO_FORMATS: [&str; 11] = [
    "mp3", "flac", "wav", "ogg", "opus", "oga", "pcm", "aiff", "aac", "wma", "alac",
];

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

pub fn load_track(track: &Path, playback_st: &mut PlaybackState) -> Result<()> {
    let mut track_temp = track.to_path_buf();
    if !is_rodio_supported(&track_temp)? {
        track_temp = playback_st.current.get_temp();
    }

    let source = get_source(track_temp).expect("Error obtaining source");

    playback_st.sink.clear();
    playback_st.sink.append(source);
    playback_st.sink.play();

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

pub fn rewind(rewind_dur: Duration, playback_st: &PlaybackState) -> Result<()> {
    let mut path = playback_st.current.path.clone().unwrap().to_path_buf();
    if !is_rodio_supported(&path)? {
        path = playback_st.current.get_temp();
    }

    let current_pos = playback_st.sink.get_pos();
    let rewinded_pos = match current_pos.checked_sub(rewind_dur) {
        Some(dur) => dur,
        None => {
            playback_st.sink.clear();
            let source = get_source(path)?;
            playback_st.sink.append(source);
            playback_st.sink.play();

            return Ok(());
        }
    };

    playback_st.sink.clear();
    let source = get_source(path)?;
    playback_st.sink.append(source);

    playback_st
        .sink
        .try_seek(rewinded_pos)
        .expect("Error rewinding");

    playback_st.sink.play();

    Ok(())
}

pub fn get_track_duration(track: &Path, playback_st: &mut PlaybackState) -> Result<Duration> {
    let mut temp_path = track.to_path_buf();
    if !is_rodio_supported(&temp_path)? {
        temp_path = playback_st.current.get_temp();
    }

    let tagged_file = Probe::open(temp_path)?.read()?;

    Ok(tagged_file.properties().duration())
}

pub async fn convert_format(track_path: &Path, temp_path: &Path) -> FFmpegProcess {
    FFmpegBuilder::convert(track_path.to_path_buf(), temp_path.to_path_buf())
        .audio_filter(AudioFilter::loudnorm())
        .spawn()
        .await
        .unwrap()
}

pub fn load_now(
    path: PathBuf,
    playback_st: &mut PlaybackState,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Result<()> {
    if path.is_file() {
        playback_st.queue.prepend_track(path);
        try_next_track(playback_st, msg_tx, info_tx)?;
    }

    Ok(())
}

pub fn bg_conversion(
    path: &Path,
    temp_path: &Path,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) {
    let path = path.to_path_buf();
    let temp = temp_path.to_path_buf();
    let cloned_msg_tx = msg_tx.clone();
    let cloned_info_tx = info_tx.clone();

    info!("Converting file {}.", path.display());
    cloned_info_tx
        .send("Converting format and normalizing volume...".to_string())
        .unwrap();

    thread::spawn(move || {
        let runtime = Runtime::new().unwrap();
        let ffmpeg_handle = Arc::new(Mutex::new(runtime.block_on(convert_format(&path, &temp))));
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
    playback_st: &mut PlaybackState,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Result<()> {
    let next_track = match playback_st.queue.dequeue() {
        Some(path) => path,
        None => return Ok(()),
    };

    playback_st.current.path = Some(next_track.clone());

    match is_rodio_supported(&next_track) {
        Ok(false) => {
            bg_conversion(
                &next_track,
                &playback_st.current.get_temp(),
                msg_tx,
                info_tx,
            );

            return Ok(());
        }
        Ok(true) => {}
        Err(e) => {
            log::error!("{}", e);
            return Err(e);
        }
    }

    play_next_track(&next_track, playback_st)?;

    Ok(())
}

pub fn play_next_track(track: &Path, playback_st: &mut PlaybackState) -> Result<()> {
    if let Err(e) = load_track(track, playback_st) {
        log::error!("{}", e);
        return Err(e);
    };

    playback_st.current.tagged_file = Some(get_metadata(
        &track.to_path_buf(),
        &playback_st.current.get_temp(),
    )?);
    playback_st.current.has_metadata = playback_st
        .current
        .tagged_file
        .as_ref()
        .and_then(|f| f.primary_tag())
        .and_then(|t| t.title())
        .is_some();
    playback_st.current.duration = playback_st
        .current
        .path
        .clone()
        .and_then(|p| get_track_duration(&p, playback_st).ok());

    Ok(())
}

pub fn get_metadata(track: &PathBuf, temp_path: &Path) -> Result<TaggedFile> {
    match Probe::open(track)?.read() {
        Ok(f) => Ok(f),
        Err(_) => Ok(Probe::open(temp_path)?.read()?),
    }
}
