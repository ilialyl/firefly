use std::{sync::Arc, time::Duration};

use mpris_server::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Playlist, PlaylistId,
    PlaylistOrdering, PlaylistsInterface, RootInterface, Time, TrackId, TrackListInterface, Uri,
    Volume,
    zbus::{Result, fdo},
};
use tokio::sync::{RwLock, mpsc::Sender};

use crate::{
    global::message::Message, player::message::PlayerMessage, queue::message::QueueMessage,
};

pub struct MprisPlayerState {
    pub position: Time,
    pub playback_status: PlaybackStatus,
    pub metadata: Metadata,
    pub volume: Volume,
    pub loop_status: LoopStatus,
}

impl Default for MprisPlayerState {
    fn default() -> Self {
        Self {
            position: Time::ZERO,
            playback_status: PlaybackStatus::Stopped,
            metadata: Metadata::new(),
            volume: 1.0,
            loop_status: LoopStatus::None,
        }
    }
}

pub struct MprisPlayer {
    pub tx: Sender<Message>,
    pub state: Arc<RwLock<MprisPlayerState>>,
}

impl RootInterface for MprisPlayer {
    async fn raise(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn quit(&self) -> fdo::Result<()> {
        if let Err(e) = self.tx.send(Message::Quit).await {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn can_quit(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn fullscreen(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn set_fullscreen(&self, _fullscreen: bool) -> Result<()> {
        Ok(())
    }

    async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn can_raise(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn has_track_list(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn identity(&self) -> fdo::Result<String> {
        Ok("Firefly".to_string())
    }

    async fn desktop_entry(&self) -> fdo::Result<String> {
        Ok("Firefly".to_string())
    }

    async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
        Ok(vec![])
    }

    async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
        Ok(vec![])
    }
}

impl PlayerInterface for MprisPlayer {
    async fn next(&self) -> fdo::Result<()> {
        if let Err(e) = self.tx.send(Message::Player(PlayerMessage::Next)).await {
            log::error!("Error sending Message through async channel: {e}");
        }

        Ok(())
    }

    async fn previous(&self) -> fdo::Result<()> {
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::PreviousTrack))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn pause(&self) -> fdo::Result<()> {
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::TogglePlay))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn play_pause(&self) -> fdo::Result<()> {
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::TogglePlay))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn stop(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn play(&self) -> fdo::Result<()> {
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::TogglePlay))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn seek(&self, offset: Time) -> fdo::Result<()> {
        log::info!("Seek time: {:?}", offset.as_secs());
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::SeekOffset(offset)))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn set_position(&self, _track_id: TrackId, position: Time) -> fdo::Result<()> {
        log::error!("{:?}", position);
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::SetPosition(
                Duration::from_secs(position.as_secs() as u64),
            )))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }
    async fn position(&self) -> fdo::Result<Time> {
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::UpdateMprisPos))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }

        let pos = self.state.read().await.position;
        log::info!("Mpris retrieved position: {}", pos.as_micros());
        Ok(pos)
    }

    async fn open_uri(&self, _uri: String) -> fdo::Result<()> {
        Ok(())
    }

    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        Ok(self.state.read().await.playback_status)
    }

    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        Ok(self.state.read().await.loop_status)
    }

    async fn set_loop_status(&self, _loop_status: LoopStatus) -> Result<()> {
        if let Err(e) = self
            .tx
            .send(Message::Player(PlayerMessage::ToggleLoop))
            .await
        {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
    }

    async fn set_rate(&self, _rate: PlaybackRate) -> Result<()> {
        Ok(())
    }

    async fn shuffle(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn set_shuffle(&self, _shuffle: bool) -> Result<()> {
        if let Err(e) = self.tx.send(Message::Queue(QueueMessage::Shuffle)).await {
            log::error!("Error sending Message through async channel: {e}");
        }
        Ok(())
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        Ok(self.state.read().await.metadata.clone())
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        let rounded = (self.state.read().await.volume * 100.0).round() / 100.0;

        Ok(rounded)
    }

    async fn set_volume(&self, _volume: Volume) -> Result<()> {
        // let new_rounded = (volume * 100.0).round() / 100.0;
        // let old_rounded = (self.state.read().await.volume * 100.0).round() / 100.0;

        // if new_rounded != old_rounded {
        //     log::info!("Mpris setting volume to {}.", new_rounded);
        //     if let Err(e) = self
        //         .tx
        //         .send(Message::Player(PlayerMessage::SetVolume(
        //             new_rounded as f32,
        //         )))
        //         .await
        //     {
        //         log::error!("Error sending Message through async channel: {e}");
        //     }
        // }

        Ok(())
    }

    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
    }

    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
    }

    async fn can_go_next(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_go_previous(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_play(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        Ok(true)
    }
}

impl TrackListInterface for MprisPlayer {
    async fn get_tracks_metadata(&self, track_ids: Vec<TrackId>) -> fdo::Result<Vec<Metadata>> {
        println!("GetTracksMetadata({track_ids:?})");
        Ok(vec![])
    }

    async fn add_track(
        &self,
        uri: Uri,
        after_track: TrackId,
        set_as_current: bool,
    ) -> fdo::Result<()> {
        println!("AddTrack({uri}, {after_track}, {set_as_current})");
        Ok(())
    }

    async fn remove_track(&self, track_id: TrackId) -> fdo::Result<()> {
        println!("RemoveTrack({track_id})");
        Ok(())
    }

    async fn go_to(&self, track_id: TrackId) -> fdo::Result<()> {
        println!("GoTo({track_id})");
        Ok(())
    }

    async fn tracks(&self) -> fdo::Result<Vec<TrackId>> {
        println!("Tracks");
        Ok(vec![])
    }

    async fn can_edit_tracks(&self) -> fdo::Result<bool> {
        println!("CanEditTracks");
        Ok(false)
    }
}

impl PlaylistsInterface for MprisPlayer {
    async fn activate_playlist(&self, _playlist_id: PlaylistId) -> fdo::Result<()> {
        Ok(())
    }

    async fn get_playlists(
        &self,
        _index: u32,
        _max_count: u32,
        _order: PlaylistOrdering,
        _reverse_order: bool,
    ) -> fdo::Result<Vec<Playlist>> {
        Ok(vec![])
    }

    async fn playlist_count(&self) -> fdo::Result<u32> {
        Ok(0)
    }

    async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>> {
        Ok(vec![])
    }

    async fn active_playlist(&self) -> fdo::Result<Option<Playlist>> {
        Ok(None)
    }
}
