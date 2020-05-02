#!/usr/bin/env python3

import argparse
import os
from enum import Enum

import usb.control
import usb.core
import usb.util
import time

def bounded_inactive_time(arg):
    time = int(arg)
    if time >= 0 and time <= 90:
        return time
    raise argparse.ArgumentTypeError("Choose a value between 0 and 90.")

def set_turnoff_inactive(time):
    # 0x00 to 0x5a (zero to 90 minutes)
    if time >= 0 and time <= 0x5a:
        data_turnoff = [0x06, 0x51, time]
        send_request(data_turnoff)
    
class SideTone(Enum):
    OFF = 0x00
    LOW = 0x04
    MEDIUM = 0x0a
    MAX = 0x12
    
def set_mic_sidetone(level):
    data = SideTone[level].value
    data_sidetone = [0x06, 0x35, 0x01, 00, data]
    send_request(data_sidetone)

def get_battery_level():
    data_battery = [0x06, 0x18]
    send_request(data_battery)
    ret = dev.read(0x83, size_or_buffer=32)
    # 3rd byte is the percentage
    return ret[2]

def ctrl_led_blink(turn_on):
    # blink is 0x02, no blink is 0x00
    data_led = [0x06, 0x55, 0x01]
    data_led.append(0x02 if turn_on else 0x00)
    return send_request(data_led)

def save_state():
    data_save = [0x06, 0x09]
    send_request(data_save)

def send_request(data):
    return dev.ctrl_transfer(0x21, 0x09, 0x0206, 5, data_or_wLength=data)

def create_cli_parser():
    parser = argparse.ArgumentParser(description="Controls for Steelseries Arctis 7 wireless headset.")
    parser.add_argument("-b", "--show-battery", help="Display the battery level percentage.", action="store_true")
    parser.add_argument("-l", "--led-blink", choices=["on", "off"], help="Set the led blink on the transceiver.", action="store")
    parser.add_argument("-s", "--sidetone", choices=[e.name for e in SideTone], help="Set the microphone sidetone.", action="store")
    parser.add_argument("-i", "--inactive-off", type=bounded_inactive_time, help="Set the time to shutdown when inactive in minutes (0 - 90).", action="store")
    return parser

def handle_args(args):

    if args.show_battery:
        print("Battery level: {0}%".format(str(get_battery_level())))

    led = args.led_blink
    if led:
        ctrl_led_blink(True if led == "on" else False)
        print("Transmitter LED {0}.".format(args.led_blink))

    sidetone = args.sidetone
    if sidetone:
        set_mic_sidetone(sidetone)
        print("Sidetone set to {0}.".format(sidetone))

    inactive_time = args.inactive_off
    if inactive_time:
        set_turnoff_inactive(inactive_time)
        print("Inactive shutoff timer set to {0}.".format(inactive_time))
        
# get device
dev = usb.core.find(idVendor=0x1038, idProduct=0x12ad)
reattach = False
if os.name != 'nt' and dev.is_kernel_driver_active(5):
    reattach = True
    dev.detach_kernel_driver(5)
# parse cli args
args = create_cli_parser().parse_args()
try:
    handle_args(args)
    # save state
    save_state()
except usb.core.USBError as identifier:
    print("Error occured: {}".format(identifier))
finally:
    # free device
    usb.util.dispose_resources(dev)
    if reattach:
        dev.attach_kernel_driver(5)