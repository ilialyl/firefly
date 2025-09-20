pub mod player_update;
pub mod playlist_update;
pub mod userinput_update;

use std::sync::mpsc::Sender;

use crate::{
    message::{
        Message,
        cmd::*,
        update::{
            player_update::update_player, playlist_update::update_playlist,
            userinput_update::update_userinput,
        },
    },
    model::Model,
};

pub fn update_global(
    model: &mut Model,
    msg: Message,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    match msg {
        Message::Tick => tick(model, msg_tx, info_tx),
        Message::AskConfirmation(msg) => ask_for_confirmation(*msg, model),
        Message::Confirm(answer) => confirmed(answer, model),
        Message::Player(player_msg) => update_player(model, player_msg, msg_tx, info_tx),
        Message::Playlist(playlist_msg) => update_playlist(model, playlist_msg),
        Message::UserInput(userinput_update) => update_userinput(model, userinput_update),
        Message::SetBusy => set_busy(model),
        Message::ConversionStarted(handle) => conversion_started(handle, model),
        Message::ConversionEnded => conversion_ended(model),
        Message::UpdateInfo(info) => update_info(info, model),
        Message::CycleTabs => cycle_tabs(model),
        Message::Quit => quit(model),
    }
}
