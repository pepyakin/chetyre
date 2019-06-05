#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt;

use nrf51822::{spis1::semstat::SEMSTATR, Peripherals};

mod serial;

/// I have 1m 60LEDs. 1 LED consits of 3 sub-LEDs, one for each color.
const SUBLED_COUNT: usize = 180;
static mut CONTROL_BUF: &mut [u8] = &mut [0; SUBLED_COUNT];

extern "C" {
    fn sendBufferAsm(
        unused: usize,
        mask: usize,
        clraddr: *const usize,
        setaddr: *const usize,
        ptr: *const u8,
        length: usize,
    );
}

#[entry]
fn main() -> ! {
    if let Some(p) = Peripherals::take() {
        serial::init(&p);
        serial::write_str(&p.UART0, "hello world!\r\n");

        // Configure the pin 1 to be output.
        // Weird thing, that on my microbit I can't make pin 0 to work.
        // Moreover, pin 1 actually provides output on the physical pin 2 (the big one, with
        // a banana plug).
        //
        // *shrug.jpg*
        p.GPIO.pin_cnf[1].write(|w| w.dir().output());

        // Initial seed for PRNG. There is a hardware random number generator in 
        // nRF51, so I should consider using it for getting the initial seed.
        let mut seed = 1337u32;

        loop {
            for i in 0..SUBLED_COUNT {
                unsafe {
                    seed = xorshift32(seed);
                    CONTROL_BUF[i] = (seed as u8) / 8; // `/ 8` is an attempt to decrease the brightness.
                }
            }

            unsafe {
                let clraddr = &p.GPIO.outclr as *const _ as *const usize;
                let setaddr = &p.GPIO.outset as *const _ as *const usize;

                sendBufferAsm(
                    0,
                    1 << 1, // bitmask for pin 1
                    clraddr,
                    setaddr,
                    CONTROL_BUF.as_ptr() as *const _,
                    SUBLED_COUNT,
                );
            }

            for _ in 0..1_000_000 {
                cortex_m::asm::nop();
            }
        }
    }

    loop {}
}

/// xorshift32 is the simplest routine that gives you random numbers.
fn xorshift32(state: u32) -> u32 {
    let mut x = state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}
