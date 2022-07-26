use super::*;

impl FromStr for TimeMode {
    type Err = Re0Error;
    fn from_str(s: &str) -> Result<Self> {
        let out = match s {
            "地球年" | "earth_year" => TimeMode::EarthYear,
            _ => return Err(Re0Error::simple_error(format!("`{}` 不是一个合法的时间模式", s)).with_level(1)),
        };
        Ok(out)
    }
}

impl NumberLiteral {
    #[doc = include_str!("get_time.md")]
    pub fn get_time(&self) -> Result<i64> {
        let ms = match self.get_unit() {
            "秒" | "s" => self.get_sec(),
            "分钟" | "分" | "m" => self.get_minutes(),
            "小时" | "时" | "h" => self.get_hours(),
            "天" | "days" | "day" | "d" => self.get_days(),
            "天前" | "days_ago" | "day_ago" => self.get_days(),
            "星期" | "周" | "week" | "w" => self.get_weeks(),
            "星期前" | "周前" | "week_ago" => self.get_weeks(),
            "月" | "M" => self.get_months(),
            "年" | "y" => self.get_years(),
            "世纪" | "centuries" | "c" => self.get_centuries(),
            _ => return Err(Re0Error::simple_error(format!("`{}` 不是合法的时间单位", self.get_unit())).with_level(1)),
        };
        Ok(ms as i64)
    }
    #[inline(always)]
    fn get_sec(&self) -> f32 {
        self.get_f32() * 1000.0
    }
    #[inline(always)]
    fn get_minutes(&self) -> f32 {
        self.get_sec() * 60.0
    }
    #[inline(always)]
    fn get_hours(&self) -> f32 {
        self.get_minutes() * 60.0
    }
    #[inline(always)]
    fn get_days(&self) -> f32 {
        self.get_hours() * 24.0
    }
    #[inline(always)]
    fn get_weeks(&self) -> f32 {
        self.get_days() * 7.0
    }
    #[inline(always)]
    fn get_months(&self) -> f32 {
        self.get_days() * 30.0
    }
    #[inline(always)]
    fn get_years(&self) -> f32 {
        self.get_days() * 365.0
    }
    #[inline(always)]
    fn get_centuries(&self) -> f32 {
        self.get_days() * 365.0 * 100.0
    }
}
