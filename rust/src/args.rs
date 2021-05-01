use std::fmt::Display;
use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Controls for Steelseries Arctis 7 wireless headset.")]
pub(crate) enum Args {
    /// Show battery percentage
    Battery,
    /// Headset configuration options
    Config {
        #[structopt(short, long, help = "On | Off")]
        led_blink: Option<LedBlink>,
        #[structopt(short, long, help = "Off | Low | Medium | High")]
        side_tone: Option<SideTone>,
        #[structopt(short, long, help = "Number of minutes [0 - 90]")]
        inactive_off: Option<InactiveOff>,
    },
}

#[derive(Debug)]
pub(crate) enum LedBlink {
    On,
    Off,
}

impl FromStr for LedBlink {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match input {
            "on" | "On" | "ON" => LedBlink::On,
            "off" | "Off" | "OFF" => LedBlink::Off,
            _ => anyhow::bail!("Could not parse LedBlink argument."),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum SideTone {
    Off = 0x00,
    Low = 0x04,
    Medium = 0x0a,
    High = 0x12,
}

impl Display for SideTone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SideTone::Off => "Off",
            SideTone::Low => "Low",
            SideTone::Medium => "Medium",
            SideTone::High => "High",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for SideTone {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match input {
            "off" | "Off" | "OFF" => SideTone::Off,
            "low" | "Low" | "LOW" => SideTone::Low,
            "medium" | "Medium" | "MEDIUM" => SideTone::Medium,
            "high" | "High" | "HIGH" => SideTone::High,
            _ => anyhow::bail!("Could not parse SideTone argument."),
        })
    }
}

#[derive(Debug)]
pub(crate) struct InactiveOff(pub u8);

impl Display for InactiveOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InactiveOff {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let val = input.trim().parse()?;
        match val {
            val @ 0..=90 => Ok(InactiveOff(val)),
            _ => anyhow::bail!("Range out of bounds (0-90)."),
        }
    }
}
