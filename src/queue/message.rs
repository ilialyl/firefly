use std::path::PathBuf;

use crate::queue::logic::mini_track::MiniTrack;

pub enum QueueMessage {
    ToggleArrange,
    MoveUp,
    MoveDown,
    QueueFilesWithFileDialog,
    QueueFileManually(PathBuf),
    QueueDirsWithFileDialog,
    QueueDirManually(PathBuf),
    Shuffle,
    Clear,
    ScrollToStart,
    ScrollToEnd,
    RemoveSelected,
    CreatedMiniTrack(MiniTrack),
    SkipToSelected,
}
