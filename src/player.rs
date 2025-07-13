use color_eyre::eyre::{Ok, Result};
use rodio::{Decoder, Sink};
use std::fs::File;

pub fn get_sink() -> Result<Sink> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;

    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    sink.stop();

    Ok(sink)
}

pub fn get_source(path: &str) -> Result<Decoder<File>> {
    let file = File::open(path)?;

    let source = Decoder::new(file)?;

    Ok(source)
}
