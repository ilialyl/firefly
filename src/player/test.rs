// use std::{
//     path::PathBuf,
//     sync::mpsc::{self, Receiver, Sender},
//     time::Duration,
// };

// use float_cmp::ApproxEq;
// use tokio::runtime::Runtime;

// use crate::{
//     logic::{playback_state::PlaybackState, player, session_state::Session},
//     message::Message,
// };

// #[test]
// fn is_rodio_supported() {
//     let path = [
//         PathBuf::from("test_assets/test.flac"),
//         PathBuf::from("test_assets/test.opus"),
//     ];
//     let result: Vec<bool> = path
//         .iter()
//         .map(|p| player::is_rodio_supported(&p).unwrap())
//         .collect();
//     assert_eq!(result[0], true);
//     assert_eq!(result[1], false);
// }

// #[test]
// fn get_sink() {
//     let result = player::get_sink();
//     assert!(result.is_ok());
// }

// #[test]
// fn get_source() {
//     let result = player::get_source(PathBuf::from("test_assets/test.flac"));
//     assert!(result.is_ok())
// }

// #[test]
// fn load_track() {
//     let path = [PathBuf::from("test_assets/test.flac")];
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());

//     path.iter().for_each(|p| {
//         player::load_track(p, &mut playback).unwrap();
//     });
// }

// #[test]
// fn increase_volume() {
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());

//     player::increase_volume(&mut playback.sink, 0.05);

//     assert!(playback.sink.volume().approx_eq(1.05, (0.0, 2)));
// }

// #[test]
// fn decrease_volume() {
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());

//     player::decrease_volume(&mut playback.sink, 0.05);

//     assert!(playback.sink.volume().approx_eq(0.95, (0.0, 2)));
// }

// #[test]
// fn seek() {
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());
//     let tracks = [PathBuf::from("test_assets/test.flac")];
//     let temp_path = PathBuf::from("test_assets/test_temp.flac");
//     for track in tracks {
//         let duration = player::read_track_duration(&track, &temp_path)
//             .ok()
//             .unwrap();
//         player::load_track(&track, &mut playback).unwrap();
//         playback.sink.pause();

//         player::seek(&mut playback.sink, &duration, Duration::from_secs(5)).ok();

//         assert_eq!(playback.sink.get_pos(), Duration::from_secs(5));
//     }
// }

// #[test]
// fn rewind() {
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());
//     let tracks = [PathBuf::from("test_assets/test.flac")];
//     let temp_path = PathBuf::from("test_assets/test_temp.flac");
//     for track in tracks {
//         let duration = player::read_track_duration(&track, &temp_path).unwrap();
//         player::load_track(&track, &mut playback).unwrap();
//         playback.current.path = Some(track.clone());

//         playback.sink.pause();

//         player::seek(&mut playback.sink, &duration, Duration::from_secs(10)).ok();
//         player::rewind(Duration::from_secs(5), &playback).ok();

//         assert_eq!(playback.sink.get_pos(), Duration::from_secs(5));
//     }
// }

// #[test]
// fn read_track_duration() {
//     let path = [
//         PathBuf::from("test_assets/test.flac"),
//         PathBuf::from("test_assets/test.opus"),
//     ];
//     let temp_path = PathBuf::from("test_assets/test_temp.flac");

//     path.iter()
//         .for_each(|p| assert!(player::read_track_duration(&p, &temp_path).is_ok()));
// }

// #[test]
// fn convert_format() {
//     let session = Session::default();
//     let playback = PlaybackState::new(session.get_code());
//     let path = [PathBuf::from("test_assets/test.opus")];

//     let runtime = Runtime::new().unwrap();

//     for p in path {
//         let temp_path = player::get_temp_file(&p, &playback.get_temp_code());

//         runtime.block_on(async {
//             player::convert_format(&p, &temp_path)
//                 .await
//                 .wait()
//                 .await
//                 .unwrap();
//         });

//         assert!(temp_path.exists());
//         if temp_path.exists() {
//             std::fs::remove_file(&temp_path).unwrap();
//         }
//     }
// }

// #[test]
// fn load_now() {
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());

//     let path = [PathBuf::from("test_assets/test.flac")];
//     let (msg_tx, _msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
//     let (info_tx, _info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

//     path.into_iter().for_each(|p| {
//         player::load_now(p, &mut playback, &msg_tx, &info_tx).unwrap();
//     });
// }

// #[test]
// fn convert_format_in_bg() {
//     let session = Session::default();

//     let path = [PathBuf::from("test_assets/test.opus")];
//     let (msg_tx, msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
//     let (info_tx, _info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

//     for p in path {
//         let temp_path = player::get_temp_file(&p, &session.get_code());

//         player::convert_format_in_bg(&p, &temp_path, &msg_tx, &info_tx);

//         let mut finished = false;
//         while !finished {
//             if let Ok(msg) = msg_rx.try_recv() {
//                 match msg {
//                     Message::ConversionEnded => {
//                         if temp_path.exists() {
//                             finished = true;
//                             std::fs::remove_file(&temp_path).unwrap();
//                         }
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     }
// }

// #[test]
// fn try_next_track() {
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());
//     let (msg_tx, _msg_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
//     let (info_tx, _info_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

//     assert!(player::try_next_track(&mut playback, &msg_tx, &info_tx).is_err());

//     playback
//         .queue
//         .enqueue_tracks(vec![PathBuf::from("test_assets/test.flac")]);

//     player::try_next_track(&mut playback, &msg_tx, &info_tx).unwrap();
// }

// #[test]
// fn play_next_track() {
//     let session = Session::default();
//     let mut playback = PlaybackState::new(session.get_code());
//     let path = [PathBuf::from("test_assets/test.flac")];

//     path.iter()
//         .for_each(|p| player::play_next_track(p, &mut playback).unwrap());
// }

// #[test]
// fn get_metadata() {
//     let session = Session::default();
//     let playback = PlaybackState::new(session.get_code());
//     let path = [
//         PathBuf::from("test_assets/test.flac"),
//         PathBuf::from("test_assets/test.opus"),
//     ];

//     path.iter().for_each(|p| {
//         player::get_metadata(p, player::get_temp_file(p, &playback.get_temp_code())).unwrap();
//     });
// }
