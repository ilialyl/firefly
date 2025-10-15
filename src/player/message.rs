use crate::player::logic::mini_track::MiniTrack;

pub enum PlayerMessage {
    LoadNow,
    TogglePlay,
    Skip,
    PreviousTrack,
    IncreaseVolume,
    DecreaseVolume,
    Seek,
    Rewind,
    ToggleArrange,
    ToggleLoop,
    MoveQueueUp,
    MoveQueueDown,
    QueueFiles,
    QueueDir,
    ShuffleQueue,
    ClearQueue,
    RemoveSelectedQueuedTrack,
    CreatedMiniTrack(MiniTrack),
    ScrollToStart,
    ScrollToEnd,
}
