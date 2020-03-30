# arctis7_controls
Controls for Steelseries Arctis 7 (2019) wireless headset 

Made this so I don't have to use the Steelseries control engine adware virus.
I find that it makes the quality of the headset worse.

Works on Windows, and most likely on Linux.

## Prerequisites
- Python 3
- PyUSB
- libusb or some other backend for PyUSB

#### Windows 
1. Install [Zadig](https://zadig.akeo.ie/)
1. Open it and select `Options->List All Devices`
1. Find `Steelseries Arctis 7 (Interface 5)
1. Install the WinUSB driver to it

Now you can successfully write to the device without Windows throwing a fit.
## Usage

```
python .\arctis7_config.py -h    

usage: arctis7_config.py [-h] [-b] [-l {on,off}] [-s {OFF,LOW,MEDIUM,MAX}] [-i INACTIVE_OFF]

Controls for Steelseries Arctis 7 headset.

optional arguments:
  -h, --help            show this help message and exit
  -b, --show-battery    Display the battery level percentage.
  -l {on,off}, --led-blink {on,off}
                        Set the led blink on the transceiver.
  -s {OFF,LOW,MEDIUM,MAX}, --sidetone {OFF,LOW,MEDIUM,MAX}
                        Set the microphone sidetone.
  -i INACTIVE_OFF, --inactive-off INACTIVE_OFF
                        Set the time to shutdown when inactive in minutes (0 - 90).
```
