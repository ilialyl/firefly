use crate::{
    global::{cmd::*, message::Message},
    model::Model,
    player::update::update_player,
    playlist::update::update_playlist,
    queue::update::update_queue,
    user_input::update::update_userinput,
};

pub async fn update_global(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Tick => tick(model).await,
        Message::AskConfirmation(prompt, msg) => ask_for_confirmation(prompt, *msg, model),
        Message::Confirm(answer) => confirmed(answer, model),
        Message::Player(player_msg) => update_player(model, player_msg).await,
        Message::Queue(queue_msg) => update_queue(model, queue_msg).await,
        Message::Playlist(playlist_msg) => update_playlist(model, playlist_msg),
        Message::UserInput(userinput_msg) => update_userinput(model, userinput_msg),
        Message::ConversionStarted(handle) => conversion_started(handle, model),
        Message::ConversionEnded => conversion_ended(model),
        Message::UpdateInfoMsg(info) => update_info_msg(info, model),
        Message::CycleTabs => cycle_tabs(model),
        Message::AcknowledgeInfo => acknowledge_info(model),
        Message::DisplayInfoMsg(info) => display_info_msg(info, model),
        Message::ProtocolCreated(img, id) => set_track_protocol(img, id, model),
        Message::ShowHelp => toggle_show_help(model),
        Message::Quit => quit(model),
    }
}
