use crate::config::SensorType;
use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
};

pub struct TemperatureReader {
    tempx_input: File,
}

impl TemperatureReader {
    pub fn new(hwmon_path: &Path, sensor_type: &SensorType) -> std::io::Result<Self> {
        File::open({
            let mut path = hwmon_path.join(sensor_type.file_name());

            if !path.exists() {
                eprintln!("WARNING: Using fallback sensor type");

                path.set_file_name(SensorType::Edge.file_name());
            }

            path
        })
        .map(|tempx_input| Self { tempx_input })
    }

    ///
    /// Returns an integer with the current temperature
    ///
    pub fn read(&mut self) -> Result<i16, std::io::Error> {
        let mut buf = [0; 10];
        let length = self.tempx_input.read(&mut buf)? - 4;

        self.tempx_input.rewind()?;

        let mut result = 0;

        for (i, c) in buf[..length]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, num_index)| (i as u32, num_index))
        {
            match c {
                b'-' => result *= -1,
                b'0'..=b'9' => result += i16::from(c - b'0') * i16::pow(10, i),
                _ => (),
            }
        }

        Ok(result)
    }
}
