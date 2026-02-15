use crate::events::PlaybackState;

pub trait MediaPlayer {
    fn display(&self, text: &str);
    fn set_paused(&self, paused: bool);
    fn is_paused(&self) -> Result<bool, &'static str>;
    fn seek(&self, time: f64);
    fn get_time_pos(&self) -> Result<f64, &'static str>;
    fn set_speed(&self, speed: f64);
    fn get_speed(&self) -> Result<f64, &'static str>;
    fn get_playback_state(&self) -> Result<PlaybackState, &'static str>;
}
