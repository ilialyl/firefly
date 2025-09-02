use std::sync::mpsc::{self, Receiver};

use float_cmp::ApproxEq;

use super::*;

#[test]
fn is_rodio_supported() {
    let path = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];
    let result: Vec<bool> = path
        .iter()
        .map(|p| super::is_rodio_supported(&p).unwrap())
        .collect();
    assert_eq!(result[0], true);
    assert_eq!(result[1], false);
}

#[test]
fn get_sink() {
    let result = super::get_sink();
    assert!(result.is_ok());
}

#[test]
fn get_source() {
    let result = super::get_source(PathBuf::from("test_assets/test.flac"));
    assert!(result.is_ok())
}

#[test]
fn load_track() {
    let path = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));

    path.iter()
        .for_each(|p| assert!(super::load_track(p, &mut playback).is_ok()));
}

#[test]
fn increase_volume() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));

    super::increase_volume(&mut playback.sink, 0.05);

    assert!(playback.sink.volume().approx_eq(1.05, (0.0, 2)));
}

#[test]
fn decrease_volume() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));

    super::decrease_volume(&mut playback.sink, 0.05);

    assert!(playback.sink.volume().approx_eq(0.95, (0.0, 2)));
}

#[test]
fn seek() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));
    let tracks = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];
    for track in tracks {
        let duration = super::read_track_duration(&track, &mut playback)
            .ok()
            .unwrap();
        super::load_track(&track, &mut playback).unwrap();
        playback.sink.pause();

        super::seek(&mut playback.sink, &duration, Duration::from_secs(5)).ok();

        assert_eq!(playback.sink.get_pos(), Duration::from_secs(5));
    }
}

#[test]
fn rewind() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));
    let tracks = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];
    for track in tracks {
        let duration = super::read_track_duration(&track, &mut playback).unwrap();
        super::load_track(&track, &mut playback).unwrap();
        playback.current.path = Some(track.clone());

        playback.sink.pause();

        super::seek(&mut playback.sink, &duration, Duration::from_secs(10)).ok();
        super::rewind(Duration::from_secs(5), &playback).ok();

        assert_eq!(playback.sink.get_pos(), Duration::from_secs(5));
    }
}

#[test]
fn read_track_duration() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));
    let path = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];
    path.iter()
        .for_each(|p| assert!(super::read_track_duration(&p, &mut playback).is_ok()));
}

#[test]
fn convert_format() {
    let converted_path = PathBuf::from("test_assets/conversion_test.flac");
    if converted_path.exists() {
        std::fs::remove_file(&converted_path).unwrap();
    }

    let path = [PathBuf::from("test_assets/test.opus")];

    let runtime = Runtime::new().unwrap();

    for p in path {
        runtime.block_on(async {
            super::convert_format(&p, &converted_path)
                .await
                .wait()
                .await
                .unwrap();
        })
    }

    assert!(converted_path.exists());
    if converted_path.exists() {
        std::fs::remove_file(&converted_path).unwrap();
    }
}

#[test]
fn load_now() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));
    let path = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];
    let (msg_tx, _msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (info_tx, _info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    path.into_iter()
        .for_each(|p| assert!(super::load_now(p, &mut playback, &msg_tx, &info_tx).is_ok()));
}

#[test]
fn convert_format_in_bg() {
    let converted_path = PathBuf::from("test_assets/bg_conversion_test.flac");
    if converted_path.exists() {
        std::fs::remove_file(&converted_path).unwrap();
    }

    let path = [PathBuf::from("test_assets/test.opus")];
    let (msg_tx, msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (info_tx, _info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    for p in path {
        super::convert_format_in_bg(&p, &converted_path, &msg_tx, &info_tx);

        let mut finished = false;
        while !finished {
            if let Ok(msg) = msg_rx.try_recv() {
                match msg {
                    Message::ConversionEnded => {
                        if converted_path.exists() {
                            finished = true;
                            std::fs::remove_file(&converted_path).unwrap();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[test]
fn try_next_track() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));
    let (msg_tx, _msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (info_tx, _info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    assert!(super::try_next_track(&mut playback, &msg_tx, &info_tx).is_err());

    playback.queue.enqueue_tracks(vec![
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ]);

    assert!(super::try_next_track(&mut playback, &msg_tx, &info_tx).is_ok());
    assert!(super::try_next_track(&mut playback, &msg_tx, &info_tx).is_ok());
}

#[test]
fn play_next_track() {
    let mut playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));
    let path = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];

    path.iter()
        .for_each(|p| assert!(super::play_next_track(p, &mut playback).is_ok()));
}

#[test]
fn get_metadata() {
    let playback = PlaybackState::new(PathBuf::from("test_assets/test_temp.flac"));
    let path = [
        PathBuf::from("test_assets/test.flac"),
        PathBuf::from("test_assets/test.opus"),
    ];

    path.iter()
        .for_each(|p| assert!(super::get_metadata(p, &playback.current.get_temp()).is_ok()));
}
