use crate::config::SensorType;
use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
};

pub struct TemperatureReader {
    ///
    /// The file to read temperatures from
    ///
    tempx_input: File,
}

impl TemperatureReader {
    pub fn new(hwmon_path: &Path, sensor_type: SensorType) -> Result<Self, super::Error> {
        File::open({
            let mut path = hwmon_path.join(sensor_type.file_name());

            if !path.exists() {
                eprintln!("WARNING: Using fallback sensor type");

                path.set_file_name(SensorType::Edge.file_name());
            }

            path
        })
        .map(|tempx_input| Self { tempx_input })
        .map_err(super::Error::OpenFile)
    }

    ///
    /// Returns an integer with the current temperature
    ///
    pub fn read(&mut self) -> Result<i16, super::Error> {
        let mut buf = [0; 10];
        let length = self
            .tempx_input
            .read(&mut buf)
            .map_err(super::Error::ReadTemperature)?
            - 4;

        self.tempx_input
            .rewind()
            .map_err(super::Error::ReadTemperature)?;

        let mut result = 0;

        #[allow(clippy::cast_possible_truncation)]
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
