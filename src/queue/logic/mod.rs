use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

use rand::rng;
use rand::seq::SliceRandom;

use color_eyre::eyre::{Result, eyre};

use crate::{
    global::{logic::files::AUDIO_FORMATS, message::Message},
    player::logic::mini_track::MiniTrack,
    queue::message::QueueMessage,
};

pub struct TrackQueue {
    // A Track is too heavy, hence MiniTrack.
    tracks: VecDeque<MiniTrack>,
    pub selected_index: Option<usize>,
    arrange_mode: bool,
    pub tx: Sender<Vec<PathBuf>>,
}

impl TrackQueue {
    pub fn new(msg_tx: Sender<Message>, unlocked_tick_rate: Arc<AtomicBool>) -> Self {
        let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();
        Self::start_queue_processing_worker(msg_tx, rx, unlocked_tick_rate);
        Self {
            tracks: VecDeque::new() as VecDeque<MiniTrack>,
            selected_index: None,
            arrange_mode: false,
            tx,
        }
    }

    pub fn get(&self) -> &VecDeque<MiniTrack> {
        &self.tracks
    }

    pub fn front_path(&self) -> Option<&PathBuf> {
        self.tracks.front().map(|t| &t.path)
    }

    pub fn get_selected(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn is_arrange(&self) -> bool {
        self.arrange_mode
    }

    pub fn prepend_track(&mut self, path: &Path) {
        self.tracks.push_front(MiniTrack::new(path));
    }

    pub fn enqueue_paths(&mut self, path_vec: Vec<PathBuf>) {
        let new_tracks: Vec<MiniTrack> = path_vec
            .iter()
            .filter(|p| p.is_file())
            .map(|p| MiniTrack::new(p))
            .collect();

        self.tracks.extend(new_tracks);
        if !self.is_empty() && self.selected_index.is_none() {
            self.selected_index = Some(0)
        }
    }

    pub fn enqueue_mini_track(&mut self, mini_track: MiniTrack) {
        self.tracks.push_back(mini_track);
        if !self.is_empty() && self.selected_index.is_none() {
            self.selected_index = Some(0)
        }
    }

    pub fn dequeue(&mut self) -> Option<PathBuf> {
        let path = self.tracks.pop_front().map(|t| t.path);
        if self.is_empty() {
            self.selected_index = None;
        }

        path
    }

    pub fn enqueue_dir(&mut self, dir: &Path) {
        if let Ok(entries) = fs::read_dir(dir) {
            let new_tracks: Vec<MiniTrack> = entries
                .filter_map(|r| r.ok())
                .map(|p| p.path())
                .filter(|p| p.is_file())
                .filter_map(|p| {
                    p.clone()
                        .extension()
                        .and_then(|e| e.to_str())
                        .filter(|e| AUDIO_FORMATS.contains(e))
                        .map(|_| p)
                })
                .map(|p| MiniTrack::new(&p))
                .collect();

            self.tracks.extend(new_tracks);
        }
    }

    pub fn move_selected_up(&mut self) -> Result<()> {
        if let Some(mut selected) = self.selected_index {
            if selected == 0 {
                return Err(eyre!("Cannot move track up, minimum index reached."));
            }

            selected = selected.saturating_sub(1);

            self.selected_index = Some(selected);

            if self.arrange_mode {
                self.tracks.swap(selected, selected.saturating_add(1));
            }
        }

        Ok(())
    }

    pub fn move_selected_down(&mut self) -> Result<()> {
        if let Some(mut selected) = self.selected_index {
            if selected >= self.tracks.len() {
                return Err(eyre!("Cannot move track down, maximum index reached."));
            }

            if self.arrange_mode && selected.saturating_add(1) < self.len() {
                self.tracks.swap(selected, selected.saturating_add(1));
            }

            selected = (selected.saturating_add(1)).min(self.tracks.len().saturating_sub(1));

            self.selected_index = Some(selected);
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn toggle_arrange(&mut self) {
        self.arrange_mode = !self.arrange_mode;
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        let mut vec: Vec<MiniTrack> = self.tracks.clone().into_iter().collect();
        vec.shuffle(&mut rng);
        self.tracks = vec.into_iter().collect();
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.selected_index = None;
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn remove_selected(&mut self) {
        if let Some(selected) = self.selected_index {
            self.tracks.remove(selected);
            if self.tracks.len() <= selected {
                self.selected_index = Some(selected.saturating_sub(1));
            }
            if self.is_empty() {
                self.selected_index = None
            }
        }
    }

    pub fn start_queue_processing_worker(
        msg_tx: Sender<Message>,
        rx: Receiver<Vec<PathBuf>>,
        unlocked_tick_rate: Arc<AtomicBool>,
    ) {
        thread::spawn(move || {
            while let Ok(path_vec) = rx.recv() {
                unlocked_tick_rate.store(true, Ordering::Relaxed);
                path_vec.iter().for_each(|p| {
                    let mini_track = MiniTrack::new(&p);
                    if let Err(e) =
                        msg_tx.send(Message::Queue(QueueMessage::CreatedMiniTrack(mini_track)))
                    {
                        log::error!("Error sending MiniTrack back to main thread: {e}")
                    };
                });
                unlocked_tick_rate.store(false, Ordering::Relaxed);
            }
        });
    }
}
