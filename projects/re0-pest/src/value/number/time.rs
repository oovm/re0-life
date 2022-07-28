use super::*;

impl Value {
    #[doc = include_str!("get_time.md")]
    pub fn get_time(&self) -> Result<i64, String> {
        let ms = match self.as_unit() {
            "秒" | "s" => self.get_sec(),
            "分钟" | "分" | "m" => self.get_minutes(),
            "分钟前" | "minutes_ago" => self.get_minutes(),
            "小时" | "时" | "hour" | "hours" | "h" => self.get_hours(),
            "小时前" | "hours_ago" => self.get_hours(),
            "天" | "days" | "day" | "d" => self.get_days(),
            "天前" | "days_ago" | "day_ago" => self.get_days(),
            "星期" | "周" | "week" | "w" => self.get_weeks(),
            "星期前" | "周前" | "week_ago" => self.get_weeks(),
            "月" | "M" => self.get_months(),
            "年" | "years" | "y" => self.get_years(),
            "年前" | "years_ago" => self.get_years(),
            "世纪" | "centuries" | "c" => self.get_centuries(),
            _ => return Err(format!("`{}` 不是合法的时间单位", self.as_unit())),
        };
        Ok(ms as i64)
    }
    #[inline(always)]
    fn get_sec(&self) -> f32 {
        self.as_f64() as f32 * 1000.0
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
