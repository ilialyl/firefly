use std::path::PathBuf;

use crate::queue::logic::mini_track::MiniTrack;

/// Messages related to TrackQueue
pub enum QueueMessage {
    ToggleArrange,
    MoveUp,
    MoveDown,
    QueueFilesWithFileDialog,
    QueueDirsWithFileDialog,
    QueuePaths(Vec<PathBuf>),
    Shuffle,
    Clear,
    ScrollToStart,
    ScrollToEnd,
    RemoveSelected,
    CreatedMiniTrack(MiniTrack),
    SkipToSelected,
}
