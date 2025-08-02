use color_eyre::eyre::{Ok, Result};
use ez_ffmpeg::FfmpegContext;
use ez_ffmpeg::FfmpegScheduler;
use lofty::{file::AudioFile, probe::Probe};
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    ops::{Add, Sub},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(PartialEq)]
pub enum Status {
    Playing,
    Paused,
    Idle,
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

pub fn load_track(sink: &Arc<Mutex<Sink>>, track: PathBuf) {
    let sink = Arc::clone(sink);
    thread::spawn(move || {
        let source = get_source(track).expect("Error obtaining source");

        let sink = sink.lock().unwrap();
        sink.clear();
        sink.append(source);
        sink.play();
    });
}

pub fn load_track_manually(sink: &Arc<Mutex<Sink>>) -> Option<PathBuf> {
    let sink = Arc::clone(sink); // Clone Arc to move into thread
    let file = FileDialog::new()
        .add_filter("audio", &["mp3", "flac", "opus"])
        .set_directory("~/")
        .pick_file();

    let mut file = match file {
        Some(f) => f,
        None => return None,
    };

    let supported_extension = vec!["flac", "mp3"];

    if !supported_extension
        .iter()
        .any(|&i| i == file.extension().unwrap())
    {
        convert_format(&file).unwrap();
    }

    let file_path = Some(file.clone());

    thread::spawn(move || {
        if !supported_extension
            .iter()
            .any(|&i| i == file.extension().unwrap())
        {
            file = PathBuf::from("temp.flac");
        }

        let source = get_source(file).expect("Error obtaining source");

        let sink = sink.lock().unwrap();
        sink.clear();
        sink.append(source);
        sink.play();
    });

    file_path
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

pub fn forward(sink: &Arc<Mutex<Sink>>, track_dur: &Duration, dur: Duration) {
    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    if current_pos.add(dur) < *track_dur {
        sink.try_seek(current_pos.add(dur))
            .expect("Error forwarding");
    } else if track_dur.sub(current_pos) < dur
        && track_dur.sub(current_pos) > Duration::from_secs(1)
    {
        sink.try_seek(track_dur.sub(Duration::from_secs(1)))
            .expect("Error forwarding");
    }
}

pub fn rewind(sink: &Arc<Mutex<Sink>>, track: PathBuf, dur: Duration) {
    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    let rewinded_pos = current_pos.checked_sub(dur).unwrap_or(Duration::new(1, 0));

    sink.clear();
    let source = get_source(track).expect("Error obtaining source");
    sink.append(source);

    sink.try_seek(rewinded_pos).expect("Error rewinding");

    sink.play();
}

pub fn get_track_duration(track: PathBuf) -> Duration {
    let tagged_file = Probe::open(track)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    tagged_file.properties().duration()
}

pub fn convert_format(track_path: &PathBuf) -> Result<()> {
    let track_path_str = track_path.display().to_string();
    let context = FfmpegContext::builder()
        .input(track_path_str)
        .filter_desc("hue=s=0") // Example filter: desaturate (optional)
        .output("temp.flac")
        .build()?;

    // 2. Run it via FfmpegScheduler (synchronous mode)
    let result = FfmpegScheduler::new(context).start()?.wait();
    result?; // Propagate any errors that occur
    Ok(())
}
