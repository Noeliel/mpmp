use crate::{
    mpv_command_string, mpv_event, mpv_format_MPV_FORMAT_STRING, mpv_get_property_string,
    mpv_handle, mpv_observe_property, mpv_set_property_string, mpv_wait_event,
};
use client::{events::PlaybackState, mediaplayer::MediaPlayer};
use std::{
    ffi::{CStr, CString, c_int},
    str::FromStr,
};

const FAILED_TO_PARSE_UTF8_ERR: &str = "Failed to parse UTF-8 String";

pub const MPV_PROPERTY_PAUSE: &CStr = c"pause";
pub const MPV_PROPERTY_TIME_POS: &CStr = c"time-pos";
pub const MPV_PROPERTY_SPEED: &CStr = c"speed";
pub const MPV_STR_TRUE: &CStr = c"yes";
pub const MPV_STR_FALSE: &CStr = c"no";

pub struct Mpv {
    handle: *mut mpv_handle,
}

impl Mpv {
    pub fn new(handle: *mut mpv_handle) -> Self {
        Mpv { handle }
    }

    pub fn command_string(&self, command: &CStr) -> c_int {
        unsafe { mpv_command_string(self.handle, command.as_ptr()) }
    }

    pub fn wait_event(&self, timeout: f64) -> *mut mpv_event {
        unsafe { mpv_wait_event(self.handle, timeout) }
    }

    pub fn get_property_string(&self, name: &CStr) -> Option<&CStr> {
        unsafe {
            let result = mpv_get_property_string(self.handle, name.as_ptr());
            if !result.is_null() {
                Option::Some(CStr::from_ptr(result))
            } else {
                Option::None
            }
        }
    }

    pub fn set_property_string(&self, name: &CStr, value: &CStr) {
        unsafe {
            mpv_set_property_string(self.handle, name.as_ptr(), value.as_ptr());
        }
    }

    pub fn observe_property(&self, name: &CStr) -> i32 {
        unsafe { mpv_observe_property(self.handle, 0, name.as_ptr(), mpv_format_MPV_FORMAT_STRING) }
    }
}

impl MediaPlayer for Mpv {
    fn display(&self, text: &str) {
        self.command_string(
            CString::from_str(format!("show-text '{}' 5000", text).as_str())
                .expect("Failed to convert to CString")
                .as_c_str(),
        );
    }

    fn set_paused(&self, paused: bool) {
        self.set_property_string(
            MPV_PROPERTY_PAUSE,
            if paused { MPV_STR_TRUE } else { MPV_STR_FALSE },
        );
    }

    fn is_paused(&self) -> Result<bool, &'static str> {
        Ok(self
            .get_property_string(MPV_PROPERTY_PAUSE)
            .ok_or("Failed to fetch paused")?
            == MPV_STR_TRUE)
    }

    fn seek(&self, time: f64) {
        let time_str = CString::new(time.to_string())
            .unwrap_or_else(|_| panic!("Failed to construct CString from {}", time));
        self.set_property_string(MPV_PROPERTY_TIME_POS, time_str.as_c_str());
    }

    fn get_time_pos(&self) -> Result<f64, &'static str> {
        self.get_property_string(MPV_PROPERTY_TIME_POS)
            .ok_or("Failed to fetch time-pos")?
            .to_str()
            .map_err(|_| FAILED_TO_PARSE_UTF8_ERR)?
            .parse::<f64>()
            .map_err(|_| "Failed to parse time-pos as f64")
    }

    fn set_speed(&self, speed: f64) {
        let speed_str = CString::new(speed.to_string())
            .unwrap_or_else(|_| panic!("Failed to construct CString from {}", speed));
        self.set_property_string(MPV_PROPERTY_SPEED, speed_str.as_c_str());
    }

    fn get_speed(&self) -> Result<f64, &'static str> {
        self.get_property_string(MPV_PROPERTY_SPEED)
            .ok_or("Failed to fetch speed")?
            .to_str()
            .map_err(|_| FAILED_TO_PARSE_UTF8_ERR)?
            .parse::<f64>()
            .map_err(|_| "Failed to parse speed as f64")
    }

    fn get_playback_state(&self) -> Result<PlaybackState, &'static str> {
        Ok(PlaybackState::new(
            /* paused */ self.is_paused()?,
            /* time_pos */ self.get_time_pos()?,
            /* speed */ self.get_speed()?,
        ))
    }
}
