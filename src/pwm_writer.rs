use std::{fs::File, io::Write, path::Path};

pub struct PwmWriter {
    pwm1: File,
}

impl PwmWriter {
    const PWM_FILE_NAME: &str = "pwm1";

    pub fn new(hwmon_path: &Path) -> std::io::Result<Self> {
        File::create(hwmon_path.join(Self::PWM_FILE_NAME)).map(|pwm1| Self { pwm1 })
    }

    ///
    /// Writes the contents of the buffer to pwm1 file.
    ///
    pub fn set_pwm(&mut self, pwm: u8) -> std::io::Result<()> {
        let arr = [
            pwm / 100 + b'0',
            pwm / 10 % 10 + b'0',
            pwm % 10 + b'0',
            b'\n',
        ];

        let slice = match pwm {
            0..=9 => &arr[2..],
            10..=99 => &arr[1..],
            100..=u8::MAX => &arr,
        };

        self.pwm1.write_all(slice)
    }
}
