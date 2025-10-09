use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
};

use rand::rng;
use rand::seq::SliceRandom;

use color_eyre::eyre::{Result, eyre};

use crate::global::logic::{files::AUDIO_FORMATS, mini_track::MiniTrack};

pub struct TrackQueue {
    tracks: VecDeque<MiniTrack>,
    selected_index: usize,
    arrange_mode: bool,
}

impl Default for TrackQueue {
    fn default() -> Self {
        Self {
            tracks: VecDeque::new() as VecDeque<MiniTrack>,
            selected_index: 0,
            arrange_mode: false,
        }
    }
}

impl TrackQueue {
    pub fn get(&self) -> &VecDeque<MiniTrack> {
        &self.tracks
    }

    pub fn front_path(&self) -> Option<&PathBuf> {
        self.tracks.front().map(|t| &t.path)
    }

    pub fn get_selected(&self) -> usize {
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
    }

    pub fn enqueue_mini_track(&mut self, mini_track: MiniTrack) {
        self.tracks.push_back(mini_track);
    }

    pub fn dequeue(&mut self) -> Option<PathBuf> {
        self.tracks.pop_front().map(|t| t.path)
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
        if self.selected_index == 0 || self.tracks.is_empty() {
            return Err(eyre!("Cannot move track up, minimum index reached."));
        }

        self.selected_index = self.selected_index.saturating_sub(1);

        if self.arrange_mode && self.tracks.len() > self.selected_index {
            self.tracks
                .swap(self.selected_index, self.selected_index + 1);
        }

        Ok(())
    }

    pub fn move_selected_down(&mut self) -> Result<()> {
        if self.tracks.is_empty() || self.selected_index == self.tracks.len() - 1 {
            return Err(eyre!("Cannot move track down, maximum index reached."));
        }

        self.selected_index = (self.selected_index + 1).min(self.tracks.len() - 1);

        if self.arrange_mode {
            self.tracks
                .swap(self.selected_index, self.selected_index - 1);
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
        self.selected_index = 0;
    }
}
