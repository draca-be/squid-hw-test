# Hardware test for Fri3d Camp 2022 badge (squid)

This repository contains Rust code to access all the hardware on the badge that was provided at Fri3d Camp 2022 edition.

More information on the badge can be found here:

[https://github.com/Fri3dCamp/badge-2020](https://github.com/Fri3dCamp/badge-2020)

To run this, set up the `esp-rs` toolchain as described in the official documentation and execute `cargo run`.



# installing rust on WSL2 Ubuntu
first update your Ubuntu
`sudo apt update`
`sudo apt upgrade`

## install rust according to instructions on https://rustup.rs/
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
select option 1 to proceed
exit your console and start it again to load the changed environment

## installing rust toolchain according to instructions on https://esp-rs.github.io/book/installation/riscv-and-xtensa.html

`cargo install espup`
`espup install`
`sh ~/.espressif/esp-idf/v5.0.2/install.sh`

## clone this repository
`git clone https://github.com/draca-be/squid-hw-test.git`
`cd squid-hw-test`

`cargo install ldproxy espflash cargo-espflash`

### to build the code
`source export-esp.sh`
`cargo build`

### to flash the code
`cargo espflash flash`

### to build and flash the code in 1 step
`cargo run`


## install tools to USB forward to WSL2
* in windows
  * install https://github.com/dorssel/usbipd-win/releases in windows
* in WSL linux
  * `sudo apt install linux-tools-virtual hwdata`
  * `sudo update-alternatives --install /usr/local/bin/usbip usbip /usr/lib/linux-tools/*/usbip 20`

## forward badge USB to WSL
* in windows powerschell with administrative rights
  * `usbipd wsl list`
    * in the list you see you will find a line with "Silicon Labs CP210x USB to UART Bridge" with in the very front of the line a bus ID, in my case 2-1
  * `usbipd wsl attach --busid 2-1`

## Make a udev rule to give proper permissions and a symbolic link
* plug in your fri3d badge and run the following command `udevadm info -a /dev/ttyUSB0`
  look for the device `ATTRS{product}=="CP2104 USB to UART Bridge Controller"` and look for the `serial` attribute 
* create the following file in `/etc/udev/rules.d/61-usb_serial.rules` (file owned by root:root 644)
  change the serial for the one found in the command above (or remove the whole ATTRS{serial} part)
```
# Copy this file to /etc/udev/rules.d/61-usb_serial.rules

ACTION!="add|change", GOTO="usb_serial_rules_end"
SUBSYSTEM!="usb|tty", GOTO="usb_serial_rules_end"

# CP201x
ATTRS{idVendor}=="10c4", ATTRS{idProduct}=="ea60", ATTRS{serial}=="01C81E54", MODE="660", GROUP="plugdev", TAG+="uaccess", SYMLINK+="fri3dBadge2020"

LABEL="usb_serial_rules_end"
```
* reload the rules `sudo udevadm control --reload`  
  If you get an error "Failed to send reload request: No such file or directory"  
  run `sudo service udev restart` then run `sudo udevadm control --reload` again.
* unplug your badge and plug it in again
* you might need to run this after every restart
  `sudo service udev restart`
  `sudo udevadm control --reload`
  * enjoy your personalized /dev/fri3dBadge2020 link