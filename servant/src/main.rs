#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt;

use nrf51822::{
    Peripherals,
    UART0,
};

mod canvas;
mod serial;

use canvas::{Color, Canvas};

fn read_color(uart0: &UART0) -> Color {
    let r = serial::read_u8(uart0);
    let g = serial::read_u8(uart0);
    let b = serial::read_u8(uart0);

    Color { g, r, b }
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

        let mut canvas = Canvas::new();

        loop {
            // Read the full command.
            let idx = serial::read_u8(&p.UART0) as usize;
            let color = read_color(&p.UART0);

            // Then clear the canvas and attempt to write the command.
            canvas.clear();
            match canvas.at_mut(idx) {
                Some(led) => *led = color,
                _ => {},
            }

            // Flush in any case.
            canvas.flush(&p.GPIO);
        }
    }

    loop {}
}
