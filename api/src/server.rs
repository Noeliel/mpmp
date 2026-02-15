use crate::{bidirectional::PropertiesV1, message::Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerMessage {
    HelloV1(HelloV1),
    ChangePropertiesV1(PropertiesV1),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HelloV1 {}

impl HelloV1 {
    pub fn new() -> Message {
        Message::ServerMessage(ServerMessage::HelloV1(HelloV1 {}))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePropertiesV1 {
    pub property: PropertiesV1,
}

impl ChangePropertiesV1 {
    pub fn new(p: PropertiesV1) -> Message {
        Message::ServerMessage(ServerMessage::ChangePropertiesV1(p))
    }
}
