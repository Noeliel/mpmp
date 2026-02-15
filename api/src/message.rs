use crate::{client::ClientMessage, server::ServerMessage};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    ServerMessage(ServerMessage),
    ClientMessage(ClientMessage),
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ClientMessage> for Message {
    fn from(value: ClientMessage) -> Self {
        Message::ClientMessage(value)
    }
}

impl From<ServerMessage> for Message {
    fn from(value: ServerMessage) -> Self {
        Message::ServerMessage(value)
    }
}
