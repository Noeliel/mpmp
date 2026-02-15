use crate::{bidirectional::PropertiesV1, message::Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientMessage {
    HelloV1(HelloV1),
    ChangePropertiesV1(PropertiesV1),
    GetPropertiesV1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HelloV1 {
    pub current_properties: PropertiesV1,
}

impl HelloV1 {
    pub fn new(current_properties: PropertiesV1) -> Message {
        Message::ClientMessage(ClientMessage::HelloV1(HelloV1 { current_properties }))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePropertiesV1 {
    pub properties: PropertiesV1,
}

impl ChangePropertiesV1 {
    pub fn new(p: PropertiesV1) -> Message {
        Message::ClientMessage(ClientMessage::ChangePropertiesV1(p))
    }
}
