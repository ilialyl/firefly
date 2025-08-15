use color_eyre::eyre::{Ok, Result, eyre};
use lofty::{file::AudioFile, probe::Probe};
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use rust_ffmpeg::prelude::*;
use std::{
    collections::VecDeque,
    fs::File,
    ops::{Add, Sub},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;

#[derive(PartialEq)]
pub enum Status {
    Playing,
    Paused,
    Idle,
}

const RODIO_SUPPORTED_FORMATS: [&'static str; 4] = ["flac", "mp3", "ogg", "wav"];
const TESTED_FORMATS: [&'static str; 6] = ["mp3", "flac", "wav", "ogg", "opus", "oga"];
const UNTESTED_FORMATS: [&'static str; 5] = ["pcm", "aiff", "aac", "wma", "alac"];
pub const CONVERTED_TRACK: &'static str = "temp.flac";

pub fn is_rodio_supported(track: &PathBuf) -> bool {
    if RODIO_SUPPORTED_FORMATS
        .iter()
        .any(|&i| i == track.extension().unwrap())
    {
        true
    } else {
        false
    }
}

pub fn get_sink() -> Result<(OutputStream, Sink)> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    Ok((stream_handle, sink))
}

pub fn get_source(track: PathBuf) -> Result<Decoder<File>> {
    let file = File::open(track)?;
    let source = Decoder::new(file)?;

    Ok(source)
}

pub fn choose_file() -> Option<PathBuf> {
    let file = FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_file();

    file
}

pub fn choose_multiple_files() -> Option<Vec<PathBuf>> {
    let file = FileDialog::new()
        .add_filter("Tested audio formats", &TESTED_FORMATS)
        .add_filter("Untested audio formats", &UNTESTED_FORMATS)
        .set_directory("~/")
        .pick_files();

    file
}

pub fn load_track(sink: &Arc<Mutex<Sink>>, track: &PathBuf) {
    let mut track_temp = track.clone();
    if !is_rodio_supported(&track_temp) {
        track_temp = PathBuf::from(CONVERTED_TRACK);
    }

    let sink = Arc::clone(sink);
    thread::spawn(move || {
        let source = get_source(track_temp).expect("Error obtaining source");

        let sink = sink.lock().unwrap();
        sink.clear();
        sink.append(source);
        sink.play();
    });
}

pub fn increase_volume(sink: &Arc<Mutex<Sink>>, amount: f32) {
    let sink = sink.lock().unwrap();
    let current_vol = sink.volume().clone();
    let increased_vol = f32::min(current_vol + amount, 2.0);
    sink.set_volume(increased_vol);
}

pub fn decrease_volume(sink: &Arc<Mutex<Sink>>, amount: f32) {
    let sink = sink.lock().unwrap();
    let current_vol = sink.volume().clone();
    let decreased_vol = f32::max(current_vol - amount, 0.0);
    sink.set_volume(decreased_vol);
}

pub fn forward(sink: &Arc<Mutex<Sink>>, track_dur: &Duration, forward_dur: Duration) {
    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    if current_pos.add(forward_dur) < *track_dur {
        sink.try_seek(current_pos.add(forward_dur))
            .expect("Error forwarding");
    } else if track_dur.sub(current_pos) < forward_dur
        && track_dur.sub(current_pos) > Duration::from_secs(1)
    {
        sink.try_seek(track_dur.sub(Duration::from_secs(1)))
            .expect("Error forwarding");
    }
}

pub fn rewind(sink: &Arc<Mutex<Sink>>, track: &PathBuf, rewind_dur: Duration) {
    let mut temp_path = track.clone();
    if !is_rodio_supported(&temp_path) {
        temp_path = PathBuf::from(CONVERTED_TRACK);
    }

    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    let rewinded_pos = current_pos
        .checked_sub(rewind_dur)
        .unwrap_or(Duration::new(1, 0));

    sink.clear();
    let source = get_source(temp_path).expect("Error obtaining source");
    sink.append(source);

    sink.try_seek(rewinded_pos).expect("Error rewinding");

    sink.play();
}

pub fn get_track_duration(track: &PathBuf) -> Duration {
    let mut temp_path = track.clone();
    if !is_rodio_supported(&temp_path) {
        temp_path = PathBuf::from(CONVERTED_TRACK)
    }

    let tagged_file = Probe::open(temp_path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    tagged_file.properties().duration()
}

pub fn convert_format(track_path: &PathBuf) {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        FFmpegBuilder::convert(track_path.clone(), CONVERTED_TRACK)
            .audio_filter(AudioFilter::loudnorm())
            .run()
            .await
            .unwrap();
    });
}

pub fn enqueue_track(track_queue: &mut VecDeque<PathBuf>) -> Result<()> {
    match choose_multiple_files() {
        Some(files) => {
            for f in files {
                track_queue.push_back(f);
            }
            Ok(())
        }
        None => return Err(eyre!("No track was chosen.")),
    }
}
