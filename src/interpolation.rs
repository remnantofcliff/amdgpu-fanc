mod error;

pub use error::Error;
use std::path::Path;

///
/// A structure that can interpolate a pwm value based on the temperature given
/// via `interpolate(x: i16)`-method.
///
#[derive(Debug)]
pub struct Interpolator {
    inner: Box<[(i16, u8)]>,
}

impl Interpolator {
    ///
    /// Reads the config file at `config_path` and parses returns a new
    /// `Interpolator` struct.
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
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    match (
                        temp_str.trim().parse::<i16>(),
                        fan_percent_str.trim().parse::<f32>(),
                    ) {
                        (Ok(temp), Ok(fan_percent)) => {
                            Ok((temp, (fan_percent * f32::from(u8::MAX)) as u8))
                        }
                        (Err(e), _) => Err(Error::ParseTemperature(e)),
                        (_, Err(e)) => Err(Error::ParseFanPercent(e)),
                    }
                })
            })
            .collect::<Result<Vec<(i16, u8)>, Error>>()
            .map(|mut inner| {
                inner.dedup_by_key(|(temp, _pwm)| *temp);
                inner.sort_unstable();
                inner.into_boxed_slice()
            })
            .map(|inner| Self { inner })
    }

    ///
    /// Interpolates the pwm based on `temp` and the inner array.
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

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
