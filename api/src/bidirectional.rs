use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PropertiesV1 {
    paused: bool,
    time_pos: f64,
    speed: f64,
}

impl PropertiesV1 {
    pub fn new(paused: bool, time_pos: f64, speed: f64) -> Self {
        PropertiesV1 {
            paused,
            time_pos,
            speed,
        }
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn time_pos(&self) -> f64 {
        self.time_pos
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }
}
