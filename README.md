# bluez-monitor
A simple Rust BlueZ D-Bus monitor that outputs events to stdout in JSON format.

## Installation Instructions
### Depenencies
It requires D-Bus and the BlueZ daemon to be running.
### Arch Linux
Arch users can install from AUR using your favourite package manager.
```
  pikaur -S bluez-monitor
```
### Building from source
```
git clone https://github.com/FieldofClay/bluez-monitor.git
cd bluez-monitor
cargo build --release
```

## Usage
It requires no configuration, just run it.
```
./bluez-monitor
```
It will output any BlueZ events it sees to stdout in JSON format. All events will have an **event** field, and then depending on the type of event, a **device_info** or **adapter_info** field. For example a device connected:
```json
{"device_info":{"address_type":"random","class":"","connected":true,"icon":"input-mouse","mac_address":"12:34:56:78:90:AB","name":"M720 Triathlon","paired":true,"trusted":true},"event":"connected"}
```
Device disconnected:
```json
{"device_info":{"address_type":"public","class":"0x00240404","connected":false,"icon":"audio-headset","mac_address":"12:34:56:78:90:AB","name":"MDR-1000X","paired":true,"trusted":true},"event":"disconnected"}
```
Adapter turned off:
```json
{"adapter_info":{"address_type":"public","alias":"bt-adapter","discovering":false,"mac_address":"12:34:56:78:90:AB","name":"bt-adapter","powered":false},"event":"powered"}
```
Its output can be piped to other programs, this example sends a notification when a device is connected/disconnected:
```bash
#!/bin/bash

bluez-monitor | jq --unbuffered -r '.event, .device_info.name' | (while read -r event && read -r name; do
  if [[ $event == "connected" ]]; then
    notify-send "$name" "Connected"
  elif [[ $event == "disconnected" ]]; then
    notify-send "$name" "Disconnected"
  fi
done)
```