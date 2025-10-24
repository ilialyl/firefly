use std::future;

use async_std::{channel::Sender, task};
use mpris_server::{
    LocalPlayerInterface, LocalPlaylistsInterface, LocalRootInterface, LocalServer,
    LocalTrackListInterface, LoopStatus, Metadata, PlaybackRate, PlaybackStatus, Playlist,
    PlaylistId, PlaylistOrdering, Property, Signal, Time, TrackId, Uri, Volume,
    zbus::{Result, fdo},
};

use crate::{global::message::Message, player::message::PlayerMessage};

pub struct MprisPlayer {
    pub tx: Sender<Message>,
}

impl LocalRootInterface for MprisPlayer {
    async fn raise(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn quit(&self) -> fdo::Result<()> {
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
        Ok(true)
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

impl LocalPlayerInterface for MprisPlayer {
    async fn next(&self) -> fdo::Result<()> {
        self.tx
            .send(Message::Player(PlayerMessage::Skip))
            .await
            .unwrap();
        Ok(())
    }

    async fn previous(&self) -> fdo::Result<()> {
        self.tx
            .send(Message::Player(PlayerMessage::PreviousTrack))
            .await
            .unwrap();
        Ok(())
    }

    async fn pause(&self) -> fdo::Result<()> {
        self.tx
            .send(Message::Player(PlayerMessage::TogglePlay))
            .await
            .unwrap();
        Ok(())
    }

    async fn play_pause(&self) -> fdo::Result<()> {
        self.tx
            .send(Message::Player(PlayerMessage::TogglePlay))
            .await
            .unwrap();
        Ok(())
    }

    async fn stop(&self) -> fdo::Result<()> {
        self.tx
            .send(Message::Player(PlayerMessage::TogglePlay))
            .await
            .unwrap();
        Ok(())
    }

    async fn play(&self) -> fdo::Result<()> {
        self.tx
            .send(Message::Player(PlayerMessage::TogglePlay))
            .await
            .unwrap();
        Ok(())
    }

    async fn seek(&self, _offset: Time) -> fdo::Result<()> {
        self.tx
            .send(Message::Player(PlayerMessage::Seek))
            .await
            .unwrap();
        Ok(())
    }

    async fn set_position(&self, _track_id: TrackId, _position: Time) -> fdo::Result<()> {
        Ok(())
    }

    async fn open_uri(&self, _uri: String) -> fdo::Result<()> {
        Ok(())
    }

    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        Ok(PlaybackStatus::Playing)
    }

    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        Ok(LoopStatus::None)
    }

    async fn set_loop_status(&self, _loop_status: LoopStatus) -> Result<()> {
        Ok(())
    }

    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(PlaybackRate::default())
    }

    async fn set_rate(&self, _rate: PlaybackRate) -> Result<()> {
        Ok(())
    }

    async fn shuffle(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn set_shuffle(&self, _shuffle: bool) -> Result<()> {
        Ok(())
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        Ok(Metadata::default())
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        Ok(Volume::default())
    }

    async fn set_volume(&self, _volume: Volume) -> Result<()> {
        Ok(())
    }

    async fn position(&self) -> fdo::Result<Time> {
        Ok(Time::ZERO)
    }

    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(PlaybackRate::default())
    }

    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(PlaybackRate::default())
    }

    async fn can_go_next(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn can_go_previous(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn can_play(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        Ok(true)
    }
}

impl LocalTrackListInterface for MprisPlayer {
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

impl LocalPlaylistsInterface for MprisPlayer {
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

pub async fn run_server(tx: Sender<Message>) -> Result<()> {
    let server = LocalServer::new_with_all("Firefly", MprisPlayer { tx }).await?;
    task::spawn_local(server.run());

    server
        .properties_changed([
            Property::CanSeek(false),
            Property::Metadata(Metadata::new()),
        ])
        .await?;

    server
        .emit(Signal::Seeked {
            position: Time::from_micros(124),
        })
        .await?;

    future::pending::<()>().await;

    Ok(())
}
