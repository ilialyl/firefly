use std::ops::{Add, Sub};
use std::sync::Arc;
use std::time::Duration;

use mpris_server::Time;
use rust_ffmpeg::FFmpegProcess;
use tokio::sync::Mutex;

use crate::player::logic::format_conversion::FormatConversion;
use crate::player::message::PlayerMessage;
use crate::{
    global::{
        logic::{files::choose_audio_file, session_state::RunningState},
        message::Message,
    },
    model::Model,
    player::logic::Player,
};

pub fn load_now(model: &mut Model) -> Option<Message> {
    if let Some(path) = choose_audio_file() {
        model.queue.prepend_track(&path);
        return Some(Message::Player(PlayerMessage::Next));
    }

    None
}

pub async fn toggle_play(model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    if let Err(e) = model.player.toggle_play().await {
        log::error!("Error toggling play: {e}");
    }

    None
}

pub async fn seek_offset(offset: Time, model: &mut Model) -> Option<Message> {
    if let Some(current_track) = model.player.current.as_ref()
        && let Some(dur) = current_track.duration
    {
        log::info!("Offset seeked: {:?}", offset);
        let pos_time = Time::from_secs(model.player.sink.get_pos().as_secs() as i64);
        if offset.is_positive() {
            let new_pos = model
                .player
                .sink
                .get_pos()
                .saturating_add(Duration::from_secs(offset.as_secs() as u64));
            if dur > new_pos {
                set_position(new_pos, model).await;
            }
        } else if offset.is_negative() {
            let new_pos =
                Duration::from_secs(pos_time.as_secs().saturating_add(offset.as_secs()) as u64);
            if dur > new_pos {
                set_position(new_pos, model).await;
            }
        }
    }

    None
}

pub async fn set_position(position: Duration, model: &mut Model) -> Option<Message> {
    if let Err(e) = model.player.set_position(position).await {
        log::error!("Error setting position: {e}");
    }

    None
}

pub async fn rewind(duration: Option<Duration>, model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    let track_dur = if let Some(current_track) = model.player.current.as_mut() {
        current_track.duration.unwrap_or(Duration::from_secs(0))
    } else {
        return None;
    };

    // Rewind duration depends on the duration of the track.
    let rewind_dur = if let Some(dur) = duration {
        dur
    } else if track_dur > Duration::from_secs(36000) {
        Duration::from_secs(1800)
    } else if track_dur > Duration::from_secs(18000) {
        Duration::from_secs(600)
    } else if track_dur > Duration::from_secs(3600) {
        Duration::from_secs(300)
    } else if track_dur > Duration::from_secs(1800) {
        Duration::from_secs(60)
    } else if track_dur > Duration::from_secs(600) {
        Duration::from_secs(10)
    } else {
        Duration::from_secs(5)
    };

    if let Err(e) = model.player.rewind(rewind_dur).await {
        log::error!("Error rewinding: {e}");
    };

    None
}

pub async fn seek(duration: Option<Duration>, model: &mut Model) -> Option<Message> {
    if model.session.state == RunningState::RunningFFmpeg {
        return None;
    }

    let track_dur = if let Some(current_track) = model.player.current.as_mut() {
        current_track.duration.unwrap_or(Duration::from_secs(0))
    } else {
        return None;
    };

    // Seek duration depends on the duration of the track.
    let seek_dur = if let Some(duration) = duration {
        duration
    } else if track_dur > Duration::from_secs(3600) {
        Duration::from_secs(20)
    } else if track_dur > Duration::from_secs(1800) {
        Duration::from_secs(15)
    } else if track_dur > Duration::from_secs(600) {
        Duration::from_secs(10)
    } else {
        Duration::from_secs(5)
    };

    if let Some(current_track) = &model.player.current {
        let duration = current_track.duration;
        if let Err(e) = model
            .player
            .seek(&duration.unwrap_or(Duration::from_secs(0)), seek_dur)
            .await
        {
            log::error!("Error seeking: {e}");
        }
    }

    None
}

pub async fn play_next_track(model: &mut Model) -> Option<Message> {
    if model.queue.is_empty() {
        return None;
    }

    if let Some(handle) = model.player.ffmpeg_handle.take()
        && let Err(e) = handle.lock().await.kill().await
    {
        log::error!("Error killing FFmpeg process: {e}");
    };

    model.session.state = RunningState::Running;

    log::info!("Loading track {:?}.", model.queue.front_path());
    model.player.sink.clear();
    if let Err(e) = model.player.load_next_track(&mut model.queue).await {
        log::error!("Error loading next track: {e}");
    };

    if let Some(current_track) = model.player.current.as_mut()
        && (current_track.conversion_status == FormatConversion::Unnecessary
            || current_track.conversion_status == FormatConversion::Done)
        && let Err(e) = model.player.reload().await
    {
        log::error!("Error reloading track in skip(): {e}");
    }

    if let Err(e) = model.player.sync_and_notify_metadata().await {
        log::error!("Error updating metadata for mpris server: {e}");
    };

    None
}

pub async fn toggle_loop(player: &mut Player) -> Option<Message> {
    if let Err(e) = player.toggle_loop().await {
        log::error!("Error toggling loop: {e}");
    };

    None
}

pub async fn decrease_volume(amount: f32, model: &mut Model) -> Option<Message> {
    if let Err(e) = model
        .player
        .set_volume(model.player.volume().sub(amount))
        .await
    {
        log::error!("Error setting volume: {e}");
    };

    None
}

pub async fn increase_volume(amount: f32, model: &mut Model) -> Option<Message> {
    if let Err(e) = model
        .player
        .set_volume(model.player.volume().add(amount))
        .await
    {
        log::error!("Error setting volume: {e}");
    };

    None
}

pub async fn set_volume(amount: f32, model: &mut Model) -> Option<Message> {
    if let Err(e) = model.player.set_volume(amount).await {
        log::error!("Error setting volume: {e}");
    };

    None
}

pub async fn previous_track(model: &mut Model) -> Option<Message> {
    if model.player.previous.is_empty() {
        return None;
    }

    if let Some(handle) = model.player.ffmpeg_handle.take()
        && let Err(e) = handle.lock().await.kill().await
    {
        log::error!("Error killing FFmpeg process: {e}");
    };

    model.session.state = RunningState::Running;

    log::info!("Loading previous track...");
    model.player.sink.clear();
    if let Err(e) = model.player.load_prev_track(&mut model.queue).await {
        log::error!("Error loading previous track: {e}");
    };

    if let Some(current_track) = model.player.current.as_mut()
        && current_track.conversion_status != FormatConversion::Running
        && let Err(e) = model.player.reload().await
    {
        log::error!("Error reloading track after prepending previous track in queue: {e}");
    };

    None
}

pub fn conversion_started(handle: Arc<Mutex<FFmpegProcess>>, model: &mut Model) -> Option<Message> {
    model.player.ffmpeg_handle = Some(handle);
    model.session.state = RunningState::RunningFFmpeg;

    None
}

pub async fn conversion_ended(model: &mut Model) -> Option<Message> {
    model.session.state = RunningState::Running;
    if let Some(current_track) = model.player.current.as_mut() {
        current_track.conversion_status = FormatConversion::Done;
        if let Err(e) = current_track.reload_after_conversion() {
            log::error!("Error reloading metadata after conversion: {e}")
        } else if let Err(e) = model.player.update_and_notify_mpris_all().await {
            log::error!("Error updating mpris server state: {e}");
        };
        if let Err(e) = model.player.reload().await {
            log::error!("Error reloading track after conversion: {e}");
        };
    }

    None
}

pub async fn sync_mpris_pos(model: &mut Model) -> Option<Message> {
    if let Err(e) = model.player.sync_mpris_pos().await {
        log::error!("Error syncing mpris position: {e}");
    }

    None
}

pub async fn sync_mpris_volume(model: &mut Model) -> Option<Message> {
    if let Err(e) = model.player.sync_mpris_volume().await {
        log::error!("Error syncing mpris volume: {e}");
    }

    None
}
