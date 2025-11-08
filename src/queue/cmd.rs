use std::path::PathBuf;

use crate::{
    global::{
        logic::files::{
            audio_paths_from_dir, choose_dirs, choose_multiple_audio_files,
            filter_paths_for_audio_files,
        },
        message::Message,
    },
    model::Model,
    player::{self, logic::Player},
    queue::logic::{TrackQueue, mini_track::MiniTrack},
};

pub fn queue_dirs_with_file_dialog(queue: &mut TrackQueue) -> Option<Message> {
    if let Some(dirs) = choose_dirs() {
        dirs.iter().for_each(|dir| {
            if let Err(e) = queue.tx.send(audio_paths_from_dir(dir)) {
                log::error!("Error sending Path Vec to queue processing worker: {e}");
            };
        });
    }

    None
}

pub fn queue_files_with_file_dialog(
    queue: &mut TrackQueue,
    player: &mut Player,
) -> Option<Message> {
    if let Some(mut path_vec) = choose_multiple_audio_files()
        && !path_vec.is_empty()
    {
        if player.current.is_none()
            && queue.is_empty()
            && let Some(first) = path_vec.first()
        {
            queue.enqueue_paths(vec![first.to_path_buf()]);
            path_vec.remove(0);
        }
        if let Err(e) = queue.tx.send(path_vec) {
            log::error!("Error sending Path Vec to queue processing worker: {e}");
        }
    }
    None
}

pub fn queue_paths(paths: Vec<PathBuf>, queue: &mut TrackQueue) -> Option<Message> {
    let mut valid_paths = filter_paths_for_audio_files(paths);

    if !valid_paths.is_empty() {
        if let Some(first) = valid_paths.first() {
            queue.enqueue_paths(vec![first.to_path_buf()]);
            valid_paths.remove(0);
        }
        if let Err(e) = queue.tx.send(valid_paths) {
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

pub async fn skip_to_selected(model: &mut Model) -> Option<Message> {
    model.queue.skip_to_selected();
    player::cmd::play_next_track(model).await;

    None
}
