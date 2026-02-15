use std::time::Instant;

use api::bidirectional::PropertiesV1;

pub enum ClientEvent {
    PropertyChange(PlaybackState),
    Info(String),
}

#[derive(Debug)]
pub struct PlaybackState {
    pub paused: bool,
    time_pos: f64,
    pub speed: f64,
    timestamp: Instant,
}

impl PlaybackState {
    pub fn new(paused: bool, time_pos: f64, speed: f64) -> Self {
        PlaybackState {
            paused,
            time_pos,
            speed,
            timestamp: Instant::now(),
        }
    }

    pub fn get_time_pos(&self) -> f64 {
        let elapsed: f64 = if self.paused {
            0.0
        } else {
            self.timestamp.elapsed().as_secs_f64()
        };

        self.time_pos + (elapsed * self.speed)
    }
}

impl From<PlaybackState> for PropertiesV1 {
    fn from(s: PlaybackState) -> Self {
        PropertiesV1::new(s.paused, s.time_pos, s.speed)
    }
}

impl From<PropertiesV1> for PlaybackState {
    fn from(p: PropertiesV1) -> Self {
        PlaybackState {
            paused: p.paused(),
            time_pos: p.time_pos(),
            speed: p.speed(),
            timestamp: Instant::now(),
        }
    }
}
