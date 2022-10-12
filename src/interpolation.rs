mod error;

pub use error::Error;
use std::path::Path;

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
    pub fn from_config(config_path: &Path) -> Result<Self, Error> {
        std::fs::read_to_string(config_path)?
            .lines()
            .map(|line| {
                line.split_once("=>")
                    .ok_or_else(|| Error::ConfigMissingDelimiter(line.to_string()))
            })
            .flat_map(|result| {
                result.map(|(temp_str, fan_percent_str)| {
                    match (
                        temp_str.trim().parse::<i16>(),
                        fan_percent_str.trim().parse::<u16>(),
                    ) {
                        (Ok(temp), Ok(fan_percent)) => Ok((temp, (fan_percent * 255 / 100) as u8)),
                        (Err(e), _) => Err(Error::Parse(e)),
                        (_, Err(e)) => Err(Error::Parse(e)),
                    }
                })
            })
            .collect::<Result<Box<[(i16, u8)]>, Error>>()
            .map(|inner| TempToPwm { inner })
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
    pub fn interpolate(&self, temp: i16) -> u8 {
        let mut iter = self.inner.iter();

        if let Some((mut temp1, mut fan_pwm1)) = iter.next() {
            // if x is less or equal to minimum temperature, return minimum pwm
            if temp <= temp1 {
                return fan_pwm1;
            }

            for (temp2, fan_pwm2) in iter {
                // Interpolation
                if temp < *temp2 {
                    return (((i16::from(fan_pwm1) * (*temp2 - temp))
                        + i16::from(*fan_pwm2) * (temp - temp1))
                        / (*temp2 - temp1)) as u8;
                }

                temp1 = *temp2;
                fan_pwm1 = *fan_pwm2;
            }
        }

        u8::MAX
    }
}
