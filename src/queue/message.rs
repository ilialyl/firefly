use crate::queue::logic::mini_track::MiniTrack;

pub enum QueueMessage {
    ToggleArrange,
    MoveUp,
    MoveDown,
    QueueFiles,
    QueueDir,
    Shuffle,
    Clear,
    ScrollToStart,
    ScrollToEnd,
    RemoveSelected,
    CreatedMiniTrack(MiniTrack),
    SkipToSelected,
}
