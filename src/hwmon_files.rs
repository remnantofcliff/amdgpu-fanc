use crate::interpolation::TempToPwm;
use std::{
    fmt::Display,
    fmt::Write as FWrite,
    fs::File,
    io::{Read, Seek, Write},
    path::PathBuf,
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
                path.push("pwm1");

                // open /sys/class/hwmon/hwmon(x)/pwm1
                pwm1_vec.push(File::create(&path)?);

                path.pop();
                path.push("temp2_input");

                if !path.exists() {
                    path.pop();
                    path.push("temp1_input");
                }

                // open /sys/class/hwmon/hwmon(x)/temp(y)_input
                tempx_input_vec.push(File::open(&path)?);

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
    /// Returns the number of valid gpus found.
    ///
    pub fn len(&self) -> usize {
        self.pwm1.len()
    }

    ///
    /// Read the temperatures from files and converts it to pwm as u8 using the
    /// func. buf is just a String buffer to avoid allocations.
    ///
    pub fn read_temps_to_pwms(
        &mut self,
        buf: &mut String,
        pwms: &mut [u8],
        temp_to_pwm: &TempToPwm,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (file, pwm) in self.tempx_input.iter_mut().zip(pwms.iter_mut()) {
            buf.clear();
            file.read_to_string(buf)?;
            file.rewind()?;

            *pwm = temp_to_pwm.interpolate((buf[0..buf.len() - 1].parse::<i32>()? / 1000) as i16);
        }

        Ok(())
    }

    ///
    /// Writes the pwms contained in the slice given as argument to pwm1 files.
    ///
    pub fn write_pwms(
        &mut self,
        buf: &mut String,
        pwms: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (file, pwm) in self.pwm1.iter_mut().zip(pwms) {
            buf.clear();

            writeln!(buf, "{pwm}")?;

            file.write_all(buf.as_bytes())?;

            buf.clear();
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

enum Mode {
    Automatic,
    Manual,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mode::Automatic => write!(f, "2"),
            Mode::Manual => write!(f, "1"),
        }
    }
}

fn change_fan_control_mode(mut base_path: PathBuf, mode: Mode) -> Result<(), std::io::Error> {
    base_path.push("pwm1_enable");

    // TODO: Figure out why writefmt doesn't work...
    // Produces Os error 22, "Invalid Argument"
    File::create(base_path)?.write_all(format!("{mode}\n").as_bytes())
}

fn get_hwmon_paths() -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    std::fs::read_dir("/sys/class/hwmon/").map(|dir| {
        dir.filter_map(|result| result.ok())
            .map(|entry| entry.path())
    })
}

fn path_is_amdgpu_path(path: &PathBuf) -> Result<bool, std::io::Error> {
    std::fs::read_dir(path).map(|dir| {
        dir.filter_map(|result| result.ok()).any(|entry| {
            entry.file_name().to_str() == Some("name")
                && std::fs::read_to_string(entry.path()).ok() == Some("amdgpu\n".into())
        })
    })
}

#[cfg(test)]
mod test {
    use super::Mode;

    #[test]
    fn mode_display_test() {
        assert_eq!(format!("{}", Mode::Automatic), "2");
        assert_eq!(format!("{}", Mode::Manual), "1");
    }
}
