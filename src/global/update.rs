use crate::{
    app::App,
    global::{cmd::*, message::Message},
    player::update::update_player,
    playlist::update::update_playlist,
    queue::update::update_queue,
    user_input::update::update_userinput,
};

pub async fn update_global(app: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::Tick => tick(app).await,
        Message::AskConfirmation(prompt, msg) => ask_for_confirmation(prompt, *msg, app),
        Message::Confirm(answer) => confirmed(answer, app),
        Message::Player(player_msg) => update_player(app, player_msg).await,
        Message::Queue(queue_msg) => update_queue(app, queue_msg).await,
        Message::Playlist(playlist_msg) => update_playlist(app, playlist_msg),
        Message::UserInput(userinput_msg) => update_userinput(app, userinput_msg),
        Message::UpdateInfoMsg(info) => update_info_msg(info, app),
        Message::CycleTabs => cycle_tabs(app),
        Message::AcknowledgeInfo => acknowledge_info(app),
        Message::DisplayInfoMsg(info) => display_info_msg(info, app),
        Message::ProtocolCreated(img, id) => set_track_protocol(img, id, app),
        Message::ShowHelp => toggle_show_help(app),
        Message::Quit => quit(app),
    }
}
