# Hardware test for Fri3d Camp 2022 badge (squid)

This repository contains Rust code to access all the hardware on the badge that was provided at Fri3d Camp 2022 edition.

More information on the badge can be found here:

[https://github.com/Fri3dCamp/badge-2020](https://github.com/Fri3dCamp/badge-2020)

To run this, set up the `esp-rs` toolchain as described in the official documentation and execute `cargo run`.

# Building on WSL2 Ubuntu
First update your Ubuntu
```
sudo apt update
sudo apt upgrade
```

## Install Rust 
Follow instructions on https://rustup.rs/
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Select option 1 to proceed.

Exit your console and start it again to load the changed environment.

## Install esp-rs toolchain
Follow instructions on https://esp-rs.github.io/book/installation/riscv-and-xtensa.html
```
cargo install espup
espup install
sh ~/.espressif/esp-idf/v5.0.2/install.sh
```
Install additional tools
```
cargo install ldproxy espflash cargo-espflash
```

## Clone this repository
```
git clone https://github.com/draca-be/squid-hw-test.git
cd squid-hw-test
```

## Building the code
```
source ~/export-esp.sh
cargo build
```

## Flashing the image
```
cargo espflash flash
```

## To build and flash in 1 step
```
cargo run
```

## Install tools to USB forward to WSL2
* In host Windows
  * Install https://github.com/dorssel/usbipd-win/releases
* In WSL linux
  * `sudo apt install linux-tools-virtual hwdata`
  * `sudo update-alternatives --install /usr/local/bin/usbip usbip /usr/lib/linux-tools/*/usbip 20`

### Forward badge USB to WSL
* In host Windows PowerShell with administrative rights
  * `usbipd wsl list`
    * In the list you will find a line with "Silicon Labs CP210x USB to UART Bridge" with in the very front of the line a bus ID, in my case 2-1
  * `usbipd wsl attach --busid 2-1`

### Make an udev rule to give proper permissions and a symbolic link
* Plug in your Fri3d badge and run the following command `udevadm info -a /dev/ttyUSB0`
* Look for the device `ATTRS{product}=="CP2104 USB to UART Bridge Controller"` and look for the `serial` attribute 
* Create the following file in `/etc/udev/rules.d/61-usb_serial.rules` (file owned by root:root 644)
* Change the serial for the one found in the command above (or remove the whole ATTRS{serial} part)
```
# Copy this file to /etc/udev/rules.d/61-usb_serial.rules

ACTION!="add|change", GOTO="usb_serial_rules_end"
SUBSYSTEM!="usb|tty", GOTO="usb_serial_rules_end"

# CP201x
ATTRS{idVendor}=="10c4", ATTRS{idProduct}=="ea60", ATTRS{serial}=="01C81E54", MODE="660", GROUP="plugdev", TAG+="uaccess", SYMLINK+="fri3dBadge2020"

LABEL="usb_serial_rules_end"
```
* Reload the rules `sudo udevadm control --reload`  
  If you get an error "Failed to send reload request: No such file or directory", run `sudo service udev restart` then
  run `sudo udevadm control --reload` again.
* Unplug your badge and plug it in again
* You might need to run this after every restart
  ```
  sudo service udev restart
  sudo udevadm control --reload
  ```
* Enjoy your personalized /dev/fri3dBadge2020 link

### Error `Error while connecting to device`
```
Unable to connect, retrying with default delay...
Unable to connect, retrying with extra delay...
Error: espflash::connection_failed

  × Error while connecting to device
  ╰─▶ Failed to connect to the device
  help: Ensure that the device is connected and the reset and boot pins are not being held down`
```
Try the following when espflash is trying to connect:
```
Serial port: '/dev/ttyUSB0'
Connecting...
```
1. Hold the button labeled boot - drukknop - IO00
2. Press the reset button  
   This will reset the esp32 and hold it in boot mode.  
3. Once espflash is flashing you can let go of the boot button.
