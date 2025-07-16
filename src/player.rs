use color_eyre::eyre::{Ok, Result};
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
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

pub fn get_source(path: PathBuf) -> Result<Decoder<File>> {
    let file = File::open(path)?;

    let source = Decoder::new(file)?;

    Ok(source)
}

pub fn load_track(sink: &Arc<Mutex<Sink>>) -> Option<PathBuf> {
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

pub fn forward(sink: &Arc<Mutex<Sink>>) {
    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    sink.try_seek(current_pos + Duration::new(5, 0))
        .expect("Error forwarding");
}

pub fn rewind(sink: &Arc<Mutex<Sink>>, file: PathBuf) {
    let sink = sink.lock().unwrap();
    let current_pos = sink.get_pos();
    let rewinded_pos = current_pos
        .checked_sub(Duration::new(5, 0))
        .unwrap_or(Duration::new(1, 0));

    sink.clear();
    let source = get_source(file).expect("Error obtaining source");
    sink.append(source);

    sink.try_seek(rewinded_pos).expect("Error rewinding");

    sink.play();
}

pub fn get_track_pos(sink: &Arc<Mutex<Sink>>) -> String {
    let sink = sink.lock().unwrap();
    let raw_pos = sink.get_pos();
    let sec = raw_pos.as_secs() % 60;
    let min = raw_pos.as_secs() / 60;

    format!("{:02}:{:02}", min, sec)
}
