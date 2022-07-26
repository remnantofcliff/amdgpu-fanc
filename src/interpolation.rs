use std::ops::{Div, Mul};

///
/// A structure that can interpolate a pwm value based on the temperature given
/// via `interpolate(x: i16)`-method.
///
pub struct TempToPwm {
    inner: Box<[(i16, u8)]>,
}
impl TempToPwm {
    ///
    /// Generates the struct based on command-line arguments. The arguments
    /// should be formatted as 'temp:fan_percentage' as in "30:50", which
    /// translates to "At thirty degrees celcius, fan speed should be at 50
    /// percent."
    ///
    /// # Panics
    /// Panics if the command-line arguments are faulty.
    ///
    pub fn from_args() -> Self {
        TempToPwm {
            inner: std::env::args()
                .skip(1)
                .map(|arg| {
                    let mut split = arg.split(':');
                    (
                        split
                            .next()
                            .unwrap()
                            .parse::<i16>()
                            .expect("Could not parse temperature before ':'"),
                        split
                            .next()
                            .expect("Bad argument format: try 'temp:fan_percentage")
                            .parse::<u16>()
                            .expect("Could not parse fan speed percent after ':'")
                            .mul(255)
                            .div(100) as u8,
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    ///
    /// Interpolates the pwm based on temperature x.
    ///
    /// If the temperature is lower than the minimum temperature in the inner
    /// array, the pwm corresponding to the minimum temperature is used.
    ///
    /// If the temperature is higher than the maximum temperature in the inner
    /// array, the pwm corresponding to the maximum temperature is used.
    ///
    /// If the inner array is empty u8::MAX is returned.
    ///
    pub fn interpolate(&self, x: i16) -> u8 {
        let mut iter = self.inner.iter().peekable();

        if let Some((first_temp, first_fan_pwm)) = iter.peek() {
            if x < *first_temp {
                return *first_fan_pwm;
            }
        }
        while let Some((temp1, fan_pwm1)) = iter.next() {
            if let Some((temp2, fan_pwm2)) = iter.peek() {
                if x < *temp2 {
                    return (((*fan_pwm1 as i16 * (*temp2 - x)) + *fan_pwm2 as i16 * (x - *temp1))
                        / (*temp2 - *temp1)) as u8;
                }
            } else {
                return *fan_pwm1;
            }
        }

        u8::MAX
    }
}
