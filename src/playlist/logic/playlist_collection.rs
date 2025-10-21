use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

use color_eyre::eyre::Result;

use crate::{
    global::{logic::Index, message::Message},
    playlist::{
        logic::{Playlist, get_playlists_path, mini_metadata::MiniMetadata},
        message::PlaylistMessage,
    },
};

pub struct PlaylistCollection {
    playlists: Vec<Playlist>,
    metadata_tx: Sender<(Index, Vec<PathBuf>)>,
}

impl PlaylistCollection {
    pub fn new(msg_tx: Sender<Message>, unlocked_tick_rate: Arc<AtomicBool>) -> Self {
        let (tx, rx) = mpsc::channel::<(Index, Vec<PathBuf>)>();
        Self::start_metadata_loader(msg_tx, rx, unlocked_tick_rate);

        log::debug!("Initialized PlaylistCollection");
        PlaylistCollection {
            playlists: Vec::<Playlist>::new(),
            metadata_tx: tx,
        }
    }

    pub fn add_tracks_to_playlist(&mut self, track_path: &Path, playlist_idx: usize) {
        let tx = self.metadata_tx.clone();
        if let Some(playlist) = self.get_playlist(playlist_idx) {
            let start = playlist.len();
            playlist.add_track_path(track_path);
            if let Some(track) = playlist.mini_tracks.get(start..playlist.len()) {
                if let Err(e) = tx.send((
                    playlist_idx,
                    track.iter().map(|t| t.borrow().path.clone()).collect(),
                )) {
                    log::error!("Error sending path vec to metadata loader: {e}");
                }
            }
        }
    }

    pub fn load_playlists(&mut self) -> Result<()> {
        log::debug!("Loading Playlists");
        let paths = Self::get_playlist_files()?;
        let playlists = paths
            .iter()
            .filter_map(|p| Playlist::from(p).ok())
            .collect::<Vec<Playlist>>();

        self.playlists = playlists;
        self.playlists.iter().enumerate().for_each(|(i, p)| {
            if let Err(e) = self.metadata_tx.send((i, p.get_pathbuf_vec())) {
                log::error!("Error sending track paths to metadata loading thread: {e}");
            }
        });

        Ok(())
    }

    pub fn get_playlist_files() -> Result<Vec<PathBuf>> {
        let playlists_path = get_playlists_path();
        log::info!("Playlist Directory: {:?}", playlists_path.to_str());
        let entries = fs::read_dir(playlists_path)?;

        let json_files: Vec<PathBuf> = entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() && path.extension().map(|ext| ext == "json").unwrap_or(false) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        Ok(json_files)
    }

    pub fn create_playlist(&mut self) -> usize {
        // Create a new playlist and returns index to it
        self.playlists.push(Playlist::default());

        self.playlists.len() - 1
    }

    pub fn len(&self) -> usize {
        self.playlists.len()
    }

    pub fn is_empty(&self) -> bool {
        self.playlists.is_empty()
    }

    pub fn get_playlist(&mut self, idx: usize) -> Option<&mut Playlist> {
        self.playlists.get_mut(idx)
    }

    pub fn get_playlists(&self) -> &Vec<Playlist> {
        &self.playlists
    }

    pub fn delete(&mut self, idx: usize) {
        self.playlists.remove(idx);
    }

    pub fn start_metadata_loader(
        msg_tx: Sender<Message>,
        rx: Receiver<(Index, Vec<PathBuf>)>,
        unlocked_tick_rate: Arc<AtomicBool>,
    ) {
        thread::spawn(move || {
            while let Ok((index, path_vec)) = rx.recv() {
                unlocked_tick_rate.store(true, Ordering::Relaxed);
                path_vec.iter().for_each(|p| {
                    let metadata = MiniMetadata::from(p);
                    if let Err(e) = msg_tx.send(Message::Playlist(PlaylistMessage::LoadedMetadata(
                        index, metadata,
                    ))) {
                        log::error!("Error sending MiniMetdata back to main thread: {e}")
                    };
                });
                unlocked_tick_rate.store(false, Ordering::Relaxed);
            }
        });
    }
}
