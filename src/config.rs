use clap::{Parser, Subcommand};

/// A minimal utility for controlling amdgpu fans.
#[derive(Parser)]
#[clap(version)]
pub struct Config {
    #[clap(subcommand)]
    pub command: ArgCommands,
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
    },
    /// Find folders of valid gpus.
    Find,
}
