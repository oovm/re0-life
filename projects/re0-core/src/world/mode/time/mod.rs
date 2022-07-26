use std::{
    ops::{AddAssign, Range},
    str::FromStr,
};

use chrono::{Duration, NaiveDateTime};
use rand::Rng;

use crate::{value::NumberLiteral, world::Dict, Re0Error, Result};

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
    type Err = Re0Error;
    fn from_str(s: &str) -> Result<Self> {
        let out = match s {
            "地球年" | "earth_year" => TimeMode::EarthYear,
            _ => return Err(Re0Error::invalid_enumeration(format!("`{}` 不是一个合法的时间模式", s)).with_level(1)),
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
        let time: i64 = rng.gen_range(Range { start: self.start.0.timestamp(), end: self.start.1.timestamp() });
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
    pub fn next_by(&self, time: &NumberLiteral) -> Result<()> {
        match time.get_unit() {
            "秒" | "s" => time.get_value::<f32>() * 1000.0f32,
            "分" | "m" => time.get_value() * 1000.0f32,
            "时" | "h" => time.get_value() * 1000.0f32,
            _ => {}
        }
        Ok(())
    }
    pub fn format_time(&self, time: &NaiveDateTime) -> String {
        match self.mode {
            TimeMode::EarthYear => time.format("%Y年").to_string(),
        }
    }
}
