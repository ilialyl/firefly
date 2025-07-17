use color_eyre::eyre::{Ok, Result};
use lofty::{file::AudioFile, probe::Probe};
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    ops::Add,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

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
    let loaded_sink = Arc::clone(sink); // Clone Arc to move into thread
    let file = FileDialog::new()
        .add_filter("audio", &["mp3", "flac"])
        .set_directory("~/")
        .pick_file();

    let file = match file {
        Some(f) => f,
        None => return None,
    };

    let file_path = Some(file.clone());

    thread::spawn(move || {
        let source = get_source(file).expect("Error obtaining source");

        let sink = loaded_sink.lock().unwrap();
        sink.clear();
        sink.append(source);
        sink.play();
    });

    file_path
}

pub fn increase_volume(sink: &Arc<Mutex<Sink>>, increase_by: f32) {
    let sink = sink.lock().unwrap();
    let current_vol = sink.volume().clone();
    let increased_vol = f32::min(current_vol + increase_by, 2.0);
    sink.set_volume(increased_vol);
}

pub fn decrease_volume(sink: &Arc<Mutex<Sink>>, decrease_by: f32) {
    let sink = sink.lock().unwrap();
    let current_vol = sink.volume().clone();
    let decreased_vol = f32::max(current_vol - decrease_by, 0.0);
    sink.set_volume(decreased_vol);
}

pub fn forward(sink: &Arc<Mutex<Sink>>, track_duration: &Duration, duration: Duration) {
    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    if current_pos.add(duration) < *track_duration {
        sink.try_seek(current_pos.add(duration))
            .expect("Error forwarding");
    }
}

pub fn rewind(sink: &Arc<Mutex<Sink>>, track: PathBuf, duration: Duration) {
    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    let rewinded_pos = current_pos
        .checked_sub(duration)
        .unwrap_or(Duration::new(1, 0));

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
