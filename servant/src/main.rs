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
        serial::write_str(&p.UART0, b"hello world!\r\n");

        // Configure the pin 1 to be output.
        // Weird thing, that on my microbit I can't make pin 0 to work.
        // Moreover, pin 1 actually provides output on the physical pin 2 (the big one, with
        // a banana plug).
        //
        // *shrug.jpg*
        p.GPIO.pin_cnf[1].write(|w| w.dir().output());

        let mut idx = 0usize;

        loop {
            let data = serial::read_u8(&p.UART0);
            let echo_buf = [data / 3];
            // serial::write_str(&p.UART0, &echo_buf[..]);
            
            unsafe {
                CONTROL_BUF[idx] = data / 5;
                idx = match idx + 1 {
                    SUBLED_COUNT => {
                        CONTROL_BUF.iter_mut().for_each(|v| *v = 0);
                        0
                    }
                    n => n,
                };
                
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
        }
    }

    loop {}
}

