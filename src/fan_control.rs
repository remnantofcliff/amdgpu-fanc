mod error;
mod pwm_writer;
mod temperature_reader;

pub use self::error::Error;
use self::{pwm_writer::PwmWriter, temperature_reader::TemperatureReader};
use crate::config::SensorType;
use std::{fs::File, io::Write, path::Path, sync::Mutex, time::Duration};

///
/// Used to control the fans. Constructed via `FanControl::enable`. Disables
/// manual fan control on drop.
///
pub struct FanControl {
    pwm: PwmWriter,
    temperature: TemperatureReader,
}

// The reason we need a static is because this needs to be accessed from the
// signal handler. A mutex is required since the signal handler could access it
// at the same time as the main thread if the main thread fails
// to read or write to file.
//
// Only FanControl::disable/enable functions should touch this static.
//
static PWM_ENABLE_FILE: Mutex<Option<File>> = Mutex::new(None);

impl FanControl {
    ///
    /// Write to `FanControl::ENABLE_PWM_FILE_NAME` to enable automatic fan
    /// control.
    ///
    const AUTO_PWM_BYTES: &[u8] = b"2\n";
    ///
    /// File name which contains information about gpu fan mode. Contains "2\n"
    /// if automatic control is on or "1\n" if control is manual. "0\n" would
    /// mean no fan control
    ///
    const ENABLE_PWM_FILE_NAME: &str = "pwm1_enable";
    ///
    /// Write to `FanControl::ENABLE_PWM_FILE_NAME` to enable manual fan
    /// control.
    ///
    const MANUAL_PWM_BYTES: &[u8] = b"1\n";
    ///
    /// Number of retries on failure to enable automatic fan control
    ///
    const RETRIES: u8 = 3;

    ///
    /// Disables manual fan control, does nothing if already disabled.
    ///
    pub fn disable() {
        let option = PWM_ENABLE_FILE.lock().unwrap().take();
        let mut pwm1_enable = match option {
            Some(file) => file,
            _ => return,
        };

        for retry in 1..=Self::RETRIES {
            match pwm1_enable.write_all(Self::AUTO_PWM_BYTES) {
                Ok(_) => return,
                Err(e) => {
                    eprintln!(
                        "Setting automatic fan control failed: {e}\n\
Retrying in 1 second {retry}/{}",
                        Self::RETRIES
                    );

                    std::thread::sleep(Duration::from_secs(1));
                }
            }
        }

        eprintln!(
            "WARNING: Disabling manual fan control failed.\n\
Reboot system or disable it manually"
        );
    }

    ///
    /// Sets the fan control to manual mode. Returns a struct that, when dropped,
    /// will disable manual fan control.
    ///
    pub fn enable(hwmon_path: &Path, sensor_type: SensorType) -> Result<Self, Error> {
        let mut pwm1_enable =
            File::create(hwmon_path.join(Self::ENABLE_PWM_FILE_NAME)).map_err(Error::OpenFile)?;

        pwm1_enable
            .write_all(Self::MANUAL_PWM_BYTES)
            .map_err(Error::ManualControl)?;

        let mut lock = PWM_ENABLE_FILE.lock().unwrap();

        *lock = Some(pwm1_enable);

        Ok(Self {
            pwm: PwmWriter::new(hwmon_path)?,
            temperature: TemperatureReader::new(hwmon_path, sensor_type)?,
        })
    }

    ///
    /// Sets the fan speed pwm value.
    ///
    pub fn set_pwm(&mut self, pwm: u8) -> Result<(), Error> {
        self.pwm.set(pwm)
    }

    ///
    /// Reads the temperature and returns it as an `i16`.
    ///
    pub fn read_temperature(&mut self) -> Result<i16, Error> {
        self.temperature.read()
    }
}

impl Drop for FanControl {
    fn drop(&mut self) {
        Self::disable();
    }
}
