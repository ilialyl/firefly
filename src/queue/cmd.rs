use crate::{
    global::{
        logic::files::{choose_dirs, choose_multiple_audio_files, dir_to_audio_paths},
        message::Message,
    },
    model::Model,
    player::logic::{Player, mini_track::MiniTrack},
    queue::logic::TrackQueue,
};

pub fn queue_dir(model: &mut Model) -> Option<Message> {
    if let Some(dirs) = choose_dirs() {
        for dir in dirs {
            if let Err(e) = model.player.queue.tx.send(dir_to_audio_paths(&dir)) {
                log::error!("Error sending Path Vec to queue processing worker: {e}");
            };
        }
    }

    None
}

pub fn queue_files(model: &mut Model) -> Option<Message> {
    if let Some(path_vec) = choose_multiple_audio_files() {
        if let Err(e) = model.player.queue.tx.send(path_vec) {
            log::error!("Error sending Path Vec to queue processing worker: {e}");
        };
    }
    None
}

pub fn queue_mini_track(mini_track: MiniTrack, player: &mut Player) -> Option<Message> {
    player.queue.enqueue_mini_track(mini_track);

    None
}

pub fn move_queue_up(player: &mut Player) -> Option<Message> {
    if let Err(e) = player.queue.move_selected_up() {
        log::error!("{}", e);
    };

    None
}

pub fn move_queue_down(player: &mut Player) -> Option<Message> {
    if let Err(e) = player.queue.move_selected_down() {
        log::error!("{}", e);
    };

    None
}

pub fn toggle_arrange(player: &mut Player) -> Option<Message> {
    player.queue.toggle_arrange();

    None
}

pub fn shuffle_queue(player: &mut Player) -> Option<Message> {
    player.queue.shuffle();

    None
}

pub fn clear_queue(player: &mut Player) -> Option<Message> {
    player.queue.clear();

    None
}

pub fn remove_selected_queued_track(player: &mut Player) -> Option<Message> {
    player.queue.remove_selected();

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
