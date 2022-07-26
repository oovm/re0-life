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
            start_range: ((), ()),
            birth_day: (),
            start_time: (),
            current: (),
            speed: 0.0
        }
    }
}