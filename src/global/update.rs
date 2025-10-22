use std::sync::mpsc::Sender;

use crate::{
    global::{cmd::*, message::Message},
    model::Model,
    player::update::update_player,
    playlist::update::update_playlist,
    queue::update::update_queue,
    user_input::update::update_userinput,
};

pub fn update_global(
    model: &mut Model,
    msg: Message,
    msg_tx: &Sender<Message>,
    info_tx: &Sender<String>,
) -> Option<Message> {
    match msg {
        Message::Tick => tick(model, msg_tx, info_tx),
        Message::AskConfirmation(prompt, msg) => ask_for_confirmation(prompt, *msg, model),
        Message::Confirm(answer) => confirmed(answer, model),
        Message::Player(player_msg) => update_player(model, player_msg, msg_tx),
        Message::Queue(queue_msg) => update_queue(model, queue_msg, msg_tx),
        Message::Playlist(playlist_msg) => update_playlist(model, playlist_msg, msg_tx),
        Message::UserInput(userinput_update) => update_userinput(model, userinput_update),
        Message::ConversionStarted(handle) => conversion_started(handle, model),
        Message::ConversionEnded => conversion_ended(model),
        Message::UpdateInfoMsg(info) => update_info_msg(info, model),
        Message::CycleTabs => cycle_tabs(model),
        Message::AcknowledgeInfo => acknowledge_info(model),
        Message::DisplayInfoMsg(info) => display_info_msg(info, info_tx),
        Message::ProtocolCreated(img, id) => set_track_protocol(img, id, model, info_tx),
        Message::ShowHelp => toggle_show_help(model),
        Message::Quit => quit(model),
    }
}
