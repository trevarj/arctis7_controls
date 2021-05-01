use std::time::Duration;

use anyhow::Result;
use rusb::{DeviceHandle, GlobalContext};
use structopt::StructOpt;

use crate::args::{Args, InactiveOff, LedBlink, SideTone};

mod args;

const BATTERY: [u8; 2] = [0x06, 0x18];

const LED_ON: [u8; 4] = [0x06, 0x55, 0x01, 0x02];
const LED_OFF: [u8; 4] = [0x06, 0x55, 0x01, 0x00];

// Last index is the parameter for INACTIVE_OFF and SIDE_TONE;
const INACTIVE_OFF: [u8; 3] = [0x06, 0x51, 0x0];
const SIDE_TONE: [u8; 5] = [0x06, 0x35, 0x01, 00, 0x0];

const SAVE_STATE: [u8; 2] = [0x06, 0x09];

fn set_led_blink(dev: &DeviceHandle<GlobalContext>, state: Option<LedBlink>) -> Result<()> {
    if let Some(state) = state {
        let data = match state {
            LedBlink::On => LED_ON,
            LedBlink::Off => LED_OFF,
        };
        send_request(dev, &data)?;
        println!("Led blink set to {:?}", state);
    }
    Ok(())
}

fn set_side_tone(dev: &DeviceHandle<GlobalContext>, state: Option<SideTone>) -> Result<()> {
    if let Some(state) = state {
        let mut data = SIDE_TONE;
        data[4] = state as u8;
        send_request(dev, &data)?;
        println!("Side tone set to {}", state);
    }
    Ok(())
}

fn set_inactive_off(dev: &DeviceHandle<GlobalContext>, state: Option<InactiveOff>) -> Result<()> {
    if let Some(state) = state {
        let mut data = INACTIVE_OFF;
        data[2] = state.0;
        send_request(dev, &data)?;
        println!("Inactive off set to {} minutes", state);
    }
    Ok(())
}

fn save_state(dev: &DeviceHandle<GlobalContext>) -> Result<()> {
    send_request(dev, &SAVE_STATE)?;
    Ok(())
}

fn send_request(dev: &DeviceHandle<GlobalContext>, data: &[u8]) -> Result<usize> {
    Ok(dev.write_control(0x21, 0x09, 0x0206, 5, data, Duration::from_secs(1))?)
}

fn main() -> Result<()> {
    let args: Args = StructOpt::from_args();
    let mut dev = rusb::devices()?
        .iter()
        .find(|dev| {
            if let Ok(desc) = dev.device_descriptor() {
                desc.vendor_id() == 0x1038 && desc.product_id() == 0x12ad
            } else {
                false
            }
        })
        .ok_or_else(|| anyhow::anyhow!("Cannot find Arctis headset."))?
        .open()?;

    dev.set_auto_detach_kernel_driver(true).unwrap_or_default();

    match args {
        Args::Battery => {
            send_request(&dev, &BATTERY)?;
            let mut buf = [0; 32];
            match dev.read_interrupt(0x83, &mut buf, Duration::from_secs(5)) {
                Ok(_) | Err(rusb::Error::Overflow) => println!("Battery level: {}%", buf[2]),
                Err(err) => anyhow::bail!("Error reading from device: {}", err),
            }
        }
        Args::Config {
            led_blink,
            side_tone,
            inactive_off,
        } => {
            set_led_blink(&dev, led_blink)?;
            set_side_tone(&dev, side_tone)?;
            set_inactive_off(&dev, inactive_off)?;
            save_state(&dev)?;
        }
    }

    Ok(())
}
