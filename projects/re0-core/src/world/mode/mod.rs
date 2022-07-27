use chrono::NaiveDateTime;
use crate::world::{World, WorldConfig};
pub use self::name::{NameConfig, NameMode};
pub use self::time::{TimeManager, TimeMode};

mod name;
mod time;


impl Default for World {
    fn default() -> Self {
        Self {
            mode: WorldConfig::default()
        }
    }
}

impl Default for NameMode {
    fn default() -> Self {
        Self::Chinese
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self {
            mode: TimeMode::EarthYear,
            start_range: (NaiveDateTime::from_timestamp(0, 0), NaiveDateTime::from_timestamp(0, 0)),
            birth_day: NaiveDateTime::from_timestamp(0, 0),
            start_time: NaiveDateTime::from_timestamp(0, 0),
            current: NaiveDateTime::from_timestamp(0, 0),
            speed: 1.0
        }
    }
}