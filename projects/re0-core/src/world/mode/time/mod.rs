use std::{
    ops::{AddAssign, Range},
    str::FromStr,
};
use std::ops::SubAssign;

use chrono::{Duration, NaiveDateTime};
use rand::Rng;

use crate::{value::NumberLiteral, world::Dict, Re0Error, Result};
mod parsing;

#[doc = include_str!("time_manager.md")]
pub struct TimeManager {
    mode: TimeMode,
    /// 起始时间范围
    start_range: (NaiveDateTime, NaiveDateTime),
    birth_day: NaiveDateTime,
    start_time: NaiveDateTime,
    current: NaiveDateTime,
    /// 时间流逝倍率
    speed: f32,
}

pub enum TimeMode {
    /// 地球年
    /// 每回合年数 +1
    EarthYear,
}



impl TimeManager {
    pub fn new(data: &Dict<Vec<String>>) -> Self {
        todo!()
    }
    pub fn get_time(&self) -> NaiveDateTime {
        self.current
    }
    pub fn restart_time(&mut self, rng: &mut impl Rng) {
        let time: i64 = rng.gen_range(Range { start: self.start_range.0.timestamp(), end: self.start_range.1.timestamp() });
        self.current = NaiveDateTime::from_timestamp(time, 0)
    }
    pub fn next_round(&mut self) {
        let time = match self.mode {
            TimeMode::EarthYear => Duration::days((365.0 * self.speed) as i64),
        };
        self.current.add_assign(time)
    }

    pub fn next_by(&mut self, time: &NumberLiteral) -> Result<()> {
        self.current.add_assign(Duration::microseconds(time.get_time()?));
        Ok(())
    }

    pub fn back_by(&mut self, time: &NumberLiteral) -> Result<()> {
        self.current.sub_assign(Duration::microseconds(time.get_time()?));
        Ok(())
    }
    pub fn format_time(&self, time: &NaiveDateTime) -> String {
        match self.mode {
            TimeMode::EarthYear => time.format("%Y年").to_string(),
        }
    }
}
