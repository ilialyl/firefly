use crate::{
    global::{
        logic::files::{choose_dirs, choose_multiple_audio_files, dir_to_audio_paths},
        message::Message,
    },
    queue::logic::{TrackQueue, mini_track::MiniTrack},
};

pub fn queue_dir(queue: &mut TrackQueue) -> Option<Message> {
    if let Some(dirs) = choose_dirs() {
        for dir in dirs {
            if let Err(e) = queue.tx.send(dir_to_audio_paths(&dir)) {
                log::error!("Error sending Path Vec to queue processing worker: {e}");
            };
        }
    }

    None
}

pub fn queue_files(queue: &mut TrackQueue) -> Option<Message> {
    if let Some(path_vec) = choose_multiple_audio_files() {
        if let Err(e) = queue.tx.send(path_vec) {
            log::error!("Error sending Path Vec to queue processing worker: {e}");
        };
    }
    None
}

pub fn queue_mini_track(mini_track: MiniTrack, queue: &mut TrackQueue) -> Option<Message> {
    queue.enqueue_mini_track(mini_track);

    None
}

pub fn move_queue_up(queue: &mut TrackQueue) -> Option<Message> {
    if let Err(e) = queue.move_selected_up() {
        log::error!("{}", e);
    };

    None
}

pub fn move_queue_down(queue: &mut TrackQueue) -> Option<Message> {
    if let Err(e) = queue.move_selected_down() {
        log::error!("{}", e);
    };

    None
}

pub fn toggle_arrange(queue: &mut TrackQueue) -> Option<Message> {
    queue.toggle_arrange();

    None
}

pub fn shuffle(queue: &mut TrackQueue) -> Option<Message> {
    queue.shuffle();

    None
}

pub fn clear(queue: &mut TrackQueue) -> Option<Message> {
    queue.clear();

    None
}

pub fn remove_selected(queue: &mut TrackQueue) -> Option<Message> {
    queue.remove_selected();

    None
}

pub fn scroll_to_start(queue: &mut TrackQueue) -> Option<Message> {
    if !queue.is_empty() {
        queue.selected_index = Some(0)
    }

    None
}

pub fn scroll_to_end(queue: &mut TrackQueue) -> Option<Message> {
    if !queue.is_empty() {
        queue.selected_index = Some(queue.len() - 1)
    }

    None
}
