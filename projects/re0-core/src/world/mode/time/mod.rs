use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime};
use rand::Rng;

pub struct TimeConfig {
    mode: TimeMode,
    /// 起始时间范围
    start: (NaiveDateTime, NaiveDateTime),
}

pub enum TimeMode {
    /// 地球年
    /// 每回合年数 +1
    EarthYear,
}

impl FromStr for TimeMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "地球年" | "earth_year" => TimeMode::EarthYear,
            _ => return Err(()),
        };
        Ok(out)
    }
}


impl TimeConfig {
    pub fn start_time(&self, rng: &mut impl Rng) -> NaiveDateTime {
        let time: i64 = rng.gen_range(self.start.0.timestamp(), self.start.1.timestamp());
        NaiveDateTime::from_timestamp(time, 0)
    }

    pub fn next_round(&self, time: &mut NaiveDateTime) {
        match self.mode {
            TimeMode::EarthYear => {}
        }
    }
}
