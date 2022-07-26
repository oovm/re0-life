use std::{ops::AddAssign, str::FromStr};

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Duration, NaiveDateTime,
};
use rand::Rng;
use crate::value::NumberLiteral;
use crate::world::Dict;

pub struct TimeManager {
    mode: TimeMode,
    /// 起始时间范围
    start: (NaiveDateTime, NaiveDateTime),
    current: NaiveDateTime,
    speed: f32,
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

impl TimeManager {
    pub fn new(data: &Dict<Vec<String>>) -> Self {
        todo!()
    }

    pub fn get_time(&self) -> NaiveDateTime {
        self.current
    }

    pub fn restart_time(&mut self, rng: &mut impl Rng) {
        let time: i64 = rng.gen_range(self.start.0.timestamp(), self.start.1.timestamp());
        self.current = NaiveDateTime::from_timestamp(time, 0)
    }
    pub fn next_round(&self, time: &mut NaiveDateTime) {
        match self.mode {
            TimeMode::EarthYear => time.add_assign(Duration::days(365)),
        }
    }
    pub fn next_year(&self, time: &mut NaiveDateTime) {
        let time = Duration::days(365);
    }
    pub fn next_by(&self, time: &NumberLiteral) {
        let time = Duration::days(30);
    }
    pub fn format_time(&self, time: &NaiveDateTime) -> String {
        match self.mode {
            TimeMode::EarthYear => time.format("%Y年").to_string(),
        }
    }
}
