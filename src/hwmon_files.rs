use std::{
    fs::{read_dir, read_to_string, File},
    io::{self, Read, Seek, Write},
    path::Path,
};

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
    pub fn new(hwmon_path: &Path) -> Result<Self, io::Error> {
        let mut path = hwmon_path.to_path_buf();
        // open /sys/class/hwmon/hwmon(x)/temp(y)_input
        path.push("temp2_input");

        // Fallback temperature read
        if !path.exists() {
            path.pop();
            path.push("temp1_input");
        }

        let tempx_input = File::open(&path)?;

        path.pop();

        path.push("pwm1");

        // open /sys/class/hwmon/hwmon(x)/pwm1
        let pwm1 = File::create(&path)?;

        Ok(Self { pwm1, tempx_input })
    }

    pub fn print_valid_devices() -> Result<(), io::Error> {
        for entry in read_dir("/sys/class/hwmon/")? {
            let entry = entry?;
            let mut path = entry.path();
            path.push("name");
            if read_to_string(&path)?.as_str() == "amdgpu\n" {
                path.pop();
                if let Some(path) = path.to_str() {
                    println!("Valid gpu path found: {path}");
                }
            }
        }

        Ok(())
    }

    pub fn read_temp(&mut self, buf: &mut String) -> Result<(), io::Error> {
        self.tempx_input.read_to_string(buf)?;
        self.tempx_input.rewind()?;

        buf.truncate(buf.len() - 4);

        Ok(())
    }

    ///
    /// Read the temperatures from files and converts it to pwm as u8 using the
    /// func. buf is just a String buffer to avoid allocations.
    ///
    pub fn set_pwm(&mut self, buf: &mut String) -> Result<(), io::Error> {
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
pub fn change_fan_control_mode(hwmon_path: &Path, mode: Mode) -> Result<(), io::Error> {
    let mut path = hwmon_path.to_path_buf();
    path.push("pwm1_enable");
    File::create(path)?.write_all(&[mode as u8, b'\n'])
}
