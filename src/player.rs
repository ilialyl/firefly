use color_eyre::eyre::{Ok, Result};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, path::PathBuf};

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
