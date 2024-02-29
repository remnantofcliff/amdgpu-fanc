use clap::{Parser, Subcommand, ValueEnum};

///
/// A minimal utility for controlling amdgpu fans.
///
#[derive(Parser)]
#[clap(version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: ArgCommands,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SensorType {
    Edge,
    Junction,
    Memory,
}

impl SensorType {
    pub const fn file_name(self) -> &'static str {
        match self {
            Self::Edge => "temp1_input",
            Self::Junction => "temp2_input",
            Self::Memory => "temp3_input",
        }
    }
}

#[derive(Subcommand)]
pub enum ArgCommands {
    ///
    /// Control fans
    ///
    Run {
        ///
        /// Path to the configuration file containing a valid fan curve
        /// description.
        ///
        #[arg(short = 'c', long, value_name = "FILE")]
        config_path: String,
        ///
        /// Path to the hwmon directory of a valid gpu.
        ///
        #[arg(short = 'p', long = "hwmon-path", value_parser, value_name = "DIR")]
        hwmon_path: Option<String>,
        #[arg(value_enum, default_value_t = SensorType::Junction)]
        sensor_type: SensorType,
    },
    ///
    /// Find folders of valid gpus.
    ///
    Find,
}
