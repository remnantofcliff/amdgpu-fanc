use std::{fs::File, io::Write, path::Path, sync::Mutex, time::Duration};

///
/// A zero-sized struct that disables manual fan control when dropped.
/// Does nothing if automatic fan control is already on.
///
pub struct ManualFanControl;

///
/// The reason we need a static is because this needs to be accessed from the
/// signal handler.
///
static PWM_ENABLE_FILE: Mutex<Option<File>> = Mutex::new(None);

impl ManualFanControl {
    const AUTO_PWM_BYTES: &[u8] = b"2\n";
    const ENABLE_PWM_FILE_NAME: &str = "pwm1_enable";
    const MANUAL_PWM_BYTES: &[u8] = b"1\n";
    const RETRIES: u8 = 3;

    ///
    /// Sets the fan control to manual mode. Returns a struct that, when dropped,
    /// will disable manual fan control.
    ///
    pub fn enable(hwmon_path: &Path) -> std::io::Result<ManualFanControl> {
        let mut pwm1_enable = File::create(hwmon_path.join(Self::ENABLE_PWM_FILE_NAME))?;

        pwm1_enable.write_all(Self::MANUAL_PWM_BYTES)?;

        let mut lock = PWM_ENABLE_FILE.lock().unwrap();

        *lock = Some(pwm1_enable);

        Ok(ManualFanControl)
    }
    ///
    /// Disables manual fan control, does nothing if already disabled.
    ///
    pub fn disable(&mut self) {
        let mut pwm1_enable = match PWM_ENABLE_FILE.lock().unwrap().take() {
            Some(it) => it,
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
}

impl Drop for ManualFanControl {
    fn drop(&mut self) {
        self.disable();
    }
}
