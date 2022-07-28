use super::*;
use re0_pest::value::NumberLiteral;

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
