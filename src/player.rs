use color_eyre::eyre::{Ok, Result};
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
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
        sink.append(source);
        sink.play();
    });

    file_path
}
