use std::ops::{Div, Mul};

const BAD_ARG_FORMAT_STR: &str = "Bad argument format: try 'temp:fan_percentage'";

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
        Self {
            inner: std::env::args()
                // Skip executable name
                .skip(1)
                .map(|arg| {
                    let mut split = arg.split(':');

                    (
                        split
                            .next()
                            .expect(BAD_ARG_FORMAT_STR)
                            .parse::<i16>()
                            .expect("Could not parse temperature before ':'"),
                        split
                            .next()
                            .expect(BAD_ARG_FORMAT_STR)
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
    /// array or if the array is empty, `u8::MAX` is returned.
    ///
    pub fn interpolate(&self, x: i16) -> u8 {
        let mut iter = self.inner.iter();

        if let Some((mut temp1, mut fan_pwm1)) = iter.next() {
            // if x is less than minimum temperature, return minimum pwm
            if x < temp1 {
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

        // Empty array
        u8::MAX
    }
}
