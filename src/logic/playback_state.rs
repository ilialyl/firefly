use std::{collections::VecDeque, fs, path::PathBuf, time::Duration};

use color_eyre::eyre::{Result, eyre};
use lofty::file::TaggedFile;
use rodio::{OutputStream, Sink};

use crate::logic::player::{self, AUDIO_FORMATS};

pub struct PlaybackState {
    pub current: Track,
    pub queue: TrackQueue,
    // pub previous: VecDeque<PathBuf>,
    pub looping: bool,
    pub status: PlaybackStatus,
    pub _stream: OutputStream,
    pub sink: Sink,
    session_code: String,
}

impl PlaybackState {
    pub fn new(session_code: String) -> PlaybackState {
        let (stream, sink) = player::get_sink().expect("Error creating sink");
        PlaybackState {
            current: Track::new(),
            queue: TrackQueue::default(),
            looping: false,
            status: PlaybackStatus::default(),
            _stream: stream,
            sink,
            session_code,
        }
    }

    pub fn get_temp_code(&self) -> String {
        self.session_code.clone()
    }
}

pub struct Track {
    pub path: Option<PathBuf>,
    pub pos: Option<Duration>,
    pub duration: Option<Duration>,
    pub tagged_file: Option<TaggedFile>,
    pub has_metadata: bool,
}

impl Track {
    pub fn new() -> Track {
        Track {
            path: None,
            pos: None,
            duration: None,
            tagged_file: None,
            has_metadata: false,
        }
    }

    pub fn reset_dur(&mut self) {
        self.pos = None;
        self.duration = None;
    }

    pub fn clear(&mut self) {
        self.tagged_file = None;
        self.duration = None;
        self.path = None;
        self.has_metadata = false;
        self.pos = None;
    }
}

#[derive(PartialEq, Debug, Default)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    #[default]
    Idle,
}

pub struct TrackQueue {
    queue: VecDeque<PathBuf>,
    selected_index: usize,
    arrange_mode: bool,
}

impl Default for TrackQueue {
    fn default() -> Self {
        Self {
            queue: VecDeque::new() as VecDeque<PathBuf>,
            selected_index: 0,
            arrange_mode: false,
        }
    }
}

impl TrackQueue {
    pub fn get(&self) -> &VecDeque<PathBuf> {
        &self.queue
    }

    pub fn front(&self) -> Option<&PathBuf> {
        self.queue.front()
    }

    pub fn get_selected(&self) -> usize {
        self.selected_index
    }

    pub fn is_arrange(&self) -> bool {
        self.arrange_mode
    }

    pub fn prepend_track(&mut self, path: PathBuf) {
        self.queue.push_front(path);
    }

    pub fn enqueue_tracks(&mut self, path_vec: Vec<PathBuf>) {
        let new_tracks: Vec<PathBuf> = path_vec.into_iter().filter(|p| p.is_file()).collect();
        self.queue.extend(new_tracks);
    }

    pub fn dequeue(&mut self) -> Option<PathBuf> {
        self.queue.pop_front()
    }

    pub fn enqueue_dir(&mut self, dir: PathBuf) {
        if let Ok(entries) = fs::read_dir(dir) {
            let path_vec: Vec<PathBuf> = entries
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
                .take(50)
                .collect();
            self.queue.extend(path_vec);
        }
    }

    pub fn move_selected_up(&mut self) -> Result<()> {
        if self.selected_index == 0 || self.queue.is_empty() {
            return Err(eyre!("Cannot move track up, minimum index reached."));
        }

        self.selected_index = self.selected_index.saturating_sub(1);

        if self.arrange_mode && self.queue.len() > self.selected_index {
            self.queue
                .swap(self.selected_index, self.selected_index + 1);
        }

        Ok(())
    }

    pub fn move_selected_down(&mut self) -> Result<()> {
        if self.queue.is_empty() || self.selected_index == self.queue.len() - 1 {
            return Err(eyre!("Cannot move track down, maximum index reached."));
        }

        self.selected_index = (self.selected_index + 1).min(self.queue.len() - 1);

        if self.arrange_mode {
            self.queue
                .swap(self.selected_index, self.selected_index - 1);
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn toggle_arrange(&mut self) {
        self.arrange_mode = !self.arrange_mode;
    }
}
