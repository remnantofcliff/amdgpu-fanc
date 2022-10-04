use std::{
    fs::{read_dir, File},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

use crate::config::SensorType;

///
/// Contains necessary files for controlling the gpu fans. Manual fan control
/// is disabled when the struct is dropped.
///
pub struct Files {
    // pwm files to write to
    pwm1: File,

    // files to read temperatures from
    tempx_input: File,
}

impl Files {
    ///
    /// Returns a new HwmonFiles-struct.
    ///
    pub fn new(hwmon_path: &Path, sensor_type: &SensorType) -> Result<Self, std::io::Error> {
        fn tempx_input_path(sensor_type: &SensorType, hwmon_path: &Path) -> PathBuf {
            let file_name = [
                b't',
                b'e',
                b'm',
                b'p',
                *sensor_type as u8,
                b'_',
                b'i',
                b'n',
                b'p',
                b'u',
                b't',
            ];

            hwmon_path.join(unsafe { std::str::from_utf8_unchecked(&file_name) })
        }

        fn tempx_input_path_with_fallback(sensor_type: &SensorType, hwmon_path: &Path) -> PathBuf {
            let path = tempx_input_path(sensor_type, hwmon_path);

            if !path.exists() {
                eprintln!("WARNING: Using fallback sensor type");

                hwmon_path.join("temp1_input")
            } else {
                path
            }
        }

        let pwm1 = File::create(hwmon_path.join("pwm1"))?;
        let tempx_input = File::open(tempx_input_path_with_fallback(sensor_type, hwmon_path))?;

        Ok(Self { pwm1, tempx_input })
    }

    ///
    /// Prints paths of valid devices in system.
    ///
    pub fn print_valid_devices() -> Result<(), std::io::Error> {
        fn dir_is_amdgpu(path: &Path) -> bool {
            std::fs::read_to_string(path.join("name"))
                .ok()
                .filter(|s| s.as_str() == "amdgpu\n")
                .is_some()
        }

        fn print_gpu_path(path: PathBuf) {
            if let Some(path) = path.to_str() {
                println!("Valid gpu path found: {path}")
            }
        }

        read_dir("/sys/class/hwmon/")?
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| dir_is_amdgpu(path))
            .for_each(print_gpu_path);

        Ok(())
    }

    ///
    /// Appends the current gpu temperature in Celsius to the buffer.
    ///
    pub fn read_temp(&mut self, buf: &mut String) -> Result<(), std::io::Error> {
        self.tempx_input.read_to_string(buf)?;
        self.tempx_input.rewind()?;

        buf.truncate(buf.len() - 4);

        Ok(())
    }

    ///
    /// Writes the contents of the buffer to pwm1 file.
    ///
    pub fn set_pwm(&mut self, buf: &mut String) -> Result<(), std::io::Error> {
        self.pwm1.write_all(buf.as_bytes())
    }
}

#[repr(u8)]
pub enum Mode {
    Automatic = b'2',
    Manual = b'1',
}

///
/// Changes the control mode of the fans. `base_path` gets changed to the
/// `pwm1_enable` path.
///
pub fn change_fan_control_mode(hwmon_path: &Path, mode: Mode) -> Result<(), std::io::Error> {
    File::create(hwmon_path.join("pwm1_enable"))?.write_all(&[mode as u8, b'\n'])
}
