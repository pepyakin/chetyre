# Flasing

Execute `./flash.sh`.

# Serial port

The following is tested on macOS:

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

