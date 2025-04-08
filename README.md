# bitaxe-raw usbserial Firmware

This repository contains USB device-side firmware for the bitaxe series boards.

### Developing

Install Rust:

```Shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

RUSTUP_TOOLCHAIN=stable cargo install espup --locked
espup install

cargo install probe-rs-tools --locked
cargo install cargo-binutils
```

For USB-based development and debugging:

```Shell
. $HOME/export-esp.sh

# Build the latest firmware:
cargo build --release

# Build, program, and attach to the device:
cargo run --release

# Just flash the device, don't attach to RTT:
cargo flash --release --chip esp32s3

# Erase all flash memory:
probe-rs erase --chip esp32s3 --allow-erase-all
```
