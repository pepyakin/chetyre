#!/bin/sh

openocd -f microbit.cfg -c "program target/thumbv6m-none-eabi/release/microbit-rust.hex verify reset exit"
