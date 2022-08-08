use clap::{clap_derive::ArgEnum, Parser, Subcommand};

/// A minimal utility for controlling amdgpu fans.
#[derive(Parser)]
#[clap(version)]
pub struct Config {
    #[clap(subcommand)]
    pub command: ArgCommands,
}

#[derive(Clone, Copy, ArgEnum)]
#[repr(u8)]
pub enum SensorType {
    Edge = b'1',
    Junction = b'2',
    Memory = b'3',
}

#[derive(Subcommand)]
pub enum ArgCommands {
    /// Control fans
    Run {
        /// Path to the configuration file containing a valid fan curve
        /// description.
        #[clap(short = 'c', long, value_parser, value_name = "FILE")]
        config_path: String,
        /// Path to the hwmon directory of a valid gpu.
        #[clap(short = 'p', long = "hwmon-path", value_parser, value_name = "DIR")]
        hwmon_path: String,
        #[clap(arg_enum, value_parser, default_value_t = SensorType::Junction)]
        sensor_type: SensorType,
    },
    /// Find folders of valid gpus.
    Find,
}
