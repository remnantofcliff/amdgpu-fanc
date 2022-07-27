use crate::interpolation::TempToPwm;
use std::{
    fmt::Write as FWrite,
    fs::File,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

///
/// Contains necessary files for controlling the gpu fans. Manual fan control
/// is disabled when the struct is dropped.
///
pub struct HwmonFiles {
    pwm1: Box<[File]>,
    tempx_input: Box<[File]>,
}

impl HwmonFiles {
    ///
    /// Returns a new HwmonFiles-struct.
    ///
    pub fn new() -> Result<HwmonFiles, std::io::Error> {
        let mut pwm1_vec = Vec::with_capacity(1);
        let mut tempx_input_vec = Vec::with_capacity(1);

        for mut path in get_hwmon_paths()? {
            if path_is_amdgpu_path(&path)? {
                // open /sys/class/hwmon/hwmon(x)/temp(y)_input
                path.push("temp2_input");

                if !path.exists() {
                    path.pop();
                    path.push("temp1_input");
                }

                tempx_input_vec.push(File::open(&path)?);

                path.pop();

                path.push("pwm1");

                // open /sys/class/hwmon/hwmon(x)/pwm1
                pwm1_vec.push(File::create(&path)?);

                path.pop();

                change_fan_control_mode(path, Mode::Manual)?;
            }
        }

        Ok(HwmonFiles {
            pwm1: pwm1_vec.into_boxed_slice(),
            tempx_input: tempx_input_vec.into_boxed_slice(),
        })
    }

    ///
    /// Read the temperatures from files and converts it to pwm as u8 using the
    /// func. buf is just a String buffer to avoid allocations.
    ///
    pub fn update_pwms(
        &mut self,
        buf: &mut String,
        temp_to_pwm: &TempToPwm,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (tempx_input, pwm1) in self.tempx_input.iter_mut().zip(self.pwm1.iter_mut()) {
            buf.clear();

            tempx_input.read_to_string(buf)?;

            tempx_input.rewind()?;

            let pwm =
                temp_to_pwm.interpolate((buf[0..buf.len() - 1].parse::<i32>()? / 1000) as i16);

            buf.clear();

            writeln!(buf, "{pwm}")?;

            pwm1.write_all(buf.as_bytes())?;
        }

        Ok(())
    }
}

impl Drop for HwmonFiles {
    fn drop(&mut self) {
        fn inner() -> Result<(), std::io::Error> {
            for path in get_hwmon_paths()? {
                if path_is_amdgpu_path(&path)? {
                    change_fan_control_mode(path, Mode::Automatic)?;
                }
            }

            Ok(())
        }

        inner().expect("WARNING: Failed to re-enable automatic fan control");
    }
}

#[repr(u8)]
pub enum Mode {
    Automatic = b'2',
    Manual = b'1',
}

fn change_fan_control_mode(mut base_path: PathBuf, mode: Mode) -> Result<(), std::io::Error> {
    base_path.push("pwm1_enable");

    File::create(base_path)?.write_all(&[mode as u8, b'\n'])
}

fn get_hwmon_paths() -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    std::fs::read_dir("/sys/class/hwmon/").map(|dir| {
        dir.filter_map(|result| result.ok())
            .map(|entry| entry.path())
    })
}

fn path_is_amdgpu_path(path: &Path) -> Result<bool, std::io::Error> {
    std::fs::read_dir(path).map(|dir| {
        dir.filter_map(|result| result.ok()).any(|entry| {
            entry.file_name().to_str() == Some("name")
                && std::fs::read_to_string(entry.path()).ok() == Some("amdgpu\n".into())
        })
    })
}
