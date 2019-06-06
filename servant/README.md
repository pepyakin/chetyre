# Overview

This is the firmware of servant. This firmware drives a WS2812 LED strip with received commands by
serial.

It is built for microbit, which uses nRF51822. There is already
the [nrf51822] crate, but for [historical reasons][nrf51#6] I ended up with stiching different
chunks and doing some parts myself.

The reference documents I am using:

- [nRF51 RM]
- [WS2812]

This project is only tested on macOS.

# Flasing

Execute `./flash.sh`.

# Serial port

1. Find an appropriate device with a path similar to /dev/cu.usbmodem<number>
2. Then `screen /dev/cu.usbmodem<number> 115200`
3. To exit, press ^A ^D

# Debugging

Start the ocd gdb server:

```
pyocd-gdbserver --persist -t nrf51 -bh -r --elf target/thumbv6m-none-eabi/release/microbit-rust
```

and then run the following in gdb:

```
target remote :3333
```

[nRF51 RM]: https://infocenter.nordicsemi.com/pdf/nRF51_RM_v3.0.pdf
[nrf51822]: https://crates.io/crates/nrf51822
[nrf51#6]: https://github.com/nrf-rs/nrf51/issues/6
[WS2812]: https://cdn-shop.adafruit.com/datasheets/WS2812.pdf