use std::fmt;
use std::{
    num::ParseIntError,
    ops::{Div, Mul},
};

const BAD_FORMAT_STR: &str = "Bad config format: try 'temp => fan_percentage'";
const ERROR_STR: &str = "Error in parsing config:";

///
/// A structure that can interpolate a pwm value based on the temperature given
/// via `interpolate(x: i16)`-method.
///
#[derive(Debug)]
pub struct TempToPwm {
    inner: Box<[(i16, u8)]>,
}

impl TempToPwm {
    ///
    /// Parses a lines from a config file and creates a `TempToPwm` struct that
    /// can interpolate between values contained in the internal array.
    ///
    pub fn from_config_lines(lines: &[&str]) -> Result<Self, TempToPwmError> {
        let mut inner = Vec::with_capacity(lines.len());

        for line in lines {
            let mut split = line.split("=>");
            let temp = split
                .next()
                .ok_or(TempToPwmError::BadConfig(BAD_FORMAT_STR))?
                .trim()
                .parse::<i16>()?;

            let pwm = split
                .next()
                .ok_or(TempToPwmError::BadConfig(BAD_FORMAT_STR))?
                .trim()
                .parse::<u16>()?
                .mul(255)
                .div(100) as u8;

            inner.push((temp, pwm));
        }

        inner.dedup_by(|(temp1, _), (temp2, _)| temp1 == temp2);
        inner.sort_unstable();

        let inner = inner.into_boxed_slice();

        Ok(Self { inner })
    }

    ///
    /// Interpolates the pwm based on temperature x.
    ///
    /// If the temperature is lower than the minimum temperature in the inner
    /// array, the pwm corresponding to the minimum temperature is used.
    ///
    /// If the temperature is higher than the maximum temperature in the inner
    /// array or if the array is empty, `u8::MAX` is returned.
    ///
    pub fn interpolate(&self, x: i16) -> u8 {
        let mut iter = self.inner.iter();

        if let Some((mut temp1, mut fan_pwm1)) = iter.next() {
            // if x is less or equal to minimum temperature, return minimum pwm
            if x <= temp1 {
                return fan_pwm1;
            }

            for (temp2, fan_pwm2) in iter {
                // Interpolation
                if x < *temp2 {
                    return (((i16::from(fan_pwm1) * (*temp2 - x))
                        + i16::from(*fan_pwm2) * (x - temp1))
                        / (*temp2 - temp1)) as u8;
                }

                temp1 = *temp2;
                fan_pwm1 = *fan_pwm2;
            }
        }

        u8::MAX
    }
}

#[derive(Debug)]
pub enum TempToPwmError {
    Parse(ParseIntError),
    BadConfig(&'static str),
}

impl fmt::Display for TempToPwmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TempToPwmError::Parse(e) => write!(f, "{ERROR_STR} {e}"),
            TempToPwmError::BadConfig(e) => write!(f, "{ERROR_STR} {e}"),
        }
    }
}

impl std::error::Error for TempToPwmError {}

impl From<ParseIntError> for TempToPwmError {
    fn from(e: ParseIntError) -> Self {
        Self::Parse(e)
    }
}

#[cfg(test)]
mod test {
    use super::TempToPwm;

    #[test]
    fn new_test() {
        let arr = [" 22   => 45", "19=>35", "59 => 100"];
        let t = TempToPwm::from_config_lines(&arr).unwrap();
        assert_eq!(t.inner[0].0, 19);
        assert_eq!(t.inner[0].1, 89);
        assert_eq!(t.inner[1].0, 22);
        assert_eq!(t.inner[1].1, 114);
        assert_eq!(t.inner[2].0, 59);
        assert_eq!(t.inner[2].1, 255);
    }

    #[test]
    fn interpolate_test() {
        let t = TempToPwm {
            inner: Box::new([(0, 10), (50, 40), (60, 70), (100, 100)]),
        };
        assert_eq!(t.interpolate(-1), 10);
        assert_eq!(t.interpolate(20), 22);
        assert_eq!(t.interpolate(51), 43);
        assert_eq!(t.interpolate(63), 72);
        assert_eq!(t.interpolate(101), u8::MAX);
    }
}
