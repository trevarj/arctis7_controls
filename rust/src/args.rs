use clap::{Parser, Subcommand, ValueEnum};

pub(crate) type InactiveOff = u8;

/// Controller for Steelseries Arctis 7 wireless headset.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Show battery percentage
    Battery,
    /// Headset configuration options
    Config {
        /// LED blink on USB dongle
        #[clap(short, long, value_parser)]
        led_blink: Option<LedBlink>,
        /// Ability to hear your voice through headphones
        #[clap(short, long, value_parser)]
        side_tone: Option<SideTone>,
        /// Number of minutes to automatically turn off headset
        #[clap(short, long, value_parser = clap::value_parser!(u8).range(0..=90))]
        inactive_off: Option<u8>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum LedBlink {
    On,
    Off,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum SideTone {
    Off = 0x00,
    Low = 0x04,
    Medium = 0x0a,
    High = 0x12,
}
