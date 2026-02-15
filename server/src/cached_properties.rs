use api::bidirectional::PropertiesV1;
use std::time::Instant;

pub enum Property {
    Paused = 1 << 0,
    TimePos = 1 << 1,
    Speed = 1 << 2,
}

#[derive(Clone)]
pub struct CachedProperties {
    pub paused: bool,
    time_pos: f64,
    time_set_at: Instant,
    speed: f64,
}

impl CachedProperties {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_paused(&mut self, p: bool) {
        self.paused = p;
    }

    pub fn get_paused(&self) -> bool {
        self.paused
    }

    pub fn set_time_pos(&mut self, time_pos: f64) {
        self.time_set_at = Instant::now();
        self.time_pos = time_pos;
    }

    pub fn get_time_pos(&self) -> f64 {
        let elapsed: f64 = if self.paused {
            0.0
        } else {
            self.time_set_at.elapsed().as_secs_f64()
        };

        self.time_pos + (elapsed * self.speed)
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    pub fn diff(&self, other: &CachedProperties) -> u32 {
        let mut diff = 0;

        if self.paused != other.paused {
            diff |= Property::Paused as u32;
        }

        if self.speed != other.speed {
            diff |= Property::Speed as u32;
        }

        if self.time_pos != other.time_pos {
            diff |= Property::TimePos as u32;
        }

        diff
    }
}

impl From<&CachedProperties> for PropertiesV1 {
    fn from(p: &CachedProperties) -> Self {
        PropertiesV1::new(p.get_paused(), p.get_time_pos(), p.get_speed())
    }
}

impl From<&PropertiesV1> for CachedProperties {
    fn from(p: &PropertiesV1) -> Self {
        CachedProperties {
            paused: p.paused(),
            time_pos: p.time_pos(),
            time_set_at: Instant::now(),
            speed: p.speed(),
        }
    }
}

impl Default for CachedProperties {
    fn default() -> Self {
        CachedProperties {
            paused: true,
            time_pos: 0.0,
            time_set_at: Instant::now(),
            speed: 1.0,
        }
    }
}
