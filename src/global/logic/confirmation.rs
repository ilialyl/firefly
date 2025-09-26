use crate::global::message::Message;

pub enum Response {
    Yes,
    No,
}

#[derive(Default)]
pub struct Confirmation {
    pub msg: Option<Message>,
    pub response: Option<Response>,
    pub prompt: String,
}
