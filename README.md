
[![Build](https://github.com/trevarj/arctis7_controls/actions/workflows/rust.yml/badge.svg)](https://github.com/trevarj/arctis7_controls/actions/workflows/rust.yml) ![Crates.io](https://img.shields.io/crates/v/arctis7-controls)

# arctis7_controls
Controls for Steelseries Arctis 7 (2019) wireless headset written in Rust or Python (your choice)

Made this so I don't have to use the Steelseries control engine adware virus.
I find that it makes the quality of the headset worse.

Works on Windows, and most likely on Linux.

**The Rust version is preferred and the one that will be maintained.** The Python one works still though.

## Python Prerequisites
- Python 3
- PyUSB
- libusb or some other backend for PyUSB

## Rust Prerequisites
- Rust + cargo

### Installation using Cargo
```
cargo install arctis7-controls
```
or from source
```
git clone https://github.com/trevarj/arctis7_controls
cd arctis7_controls/rust
cargo install --path .
```

#### Windows 
1. Install [Zadig](https://zadig.akeo.ie/)
1. Open it and select `Options->List All Devices`
1. Find `Steelseries Arctis 7 (Interface 5)`
1. Install the WinUSB driver to it

Now you can successfully write to the device without Windows throwing a fit.

## Features
- Check battery percentage 
- Turn on/off LED blink on USB dongle
- Configure microphone side-tone (hearing yourself talk)
- Configure automatic turn off

## Usage

```
➜  arctis7-controls --help
A utility to control the Steelseries Arctis7 Wireless Headset

Usage: arctis7-controls <COMMAND>

Commands:
  battery  Show battery percentage
  config   Headset configuration options
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```
