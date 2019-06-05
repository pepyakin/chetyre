#!/bin/sh

set -eux

xargo build --release

arm-none-eabi-objcopy -O ihex \
    target/thumbv6m-none-eabi/release/microbit-rust \
    target/thumbv6m-none-eabi/release/microbit-rust.hex


    