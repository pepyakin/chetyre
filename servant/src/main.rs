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

        let mut decaying_cursor = DecayingCursor::new(3, 1);

        loop {
            serial::write_str(&p.UART0, b"flush\r\n");

            // // Read the full command.
            // let idx = serial::read_u8(&p.UART0) as usize;
            // let color = read_color(&p.UART0);

            // // Then clear the canvas and attempt to write the command.
            // canvas.clear();
            // match canvas.at_mut(idx) {
            //     Some(led) => *led = color,
            //     _ => {},
            // }

            unsafe {
                decaying_cursor.step();
                // CANVAS.invert();

                // for _ in 0..100_000_0 {
                //     cortex_m::asm::nop();
                // }

                // Flush in any case.
                decaying_cursor.canvas.flush(&p.GPIO);
            }
        }
    }

    loop {}
}

struct DecayingCursor {
    /// Every so much ticks cursor moves by 1.
    vel_recip: usize,
    /// How quickly the trail decays.
    decay: u8,
    /// A tick is a measure of time.
    tick: usize,
    pos: usize,
    canvas: Canvas,
}

impl DecayingCursor {
    fn new(vel_recip: usize, decay: u8) -> Self {
        DecayingCursor {
            vel_recip,
            decay,
            tick: 0,
            pos: 0,
            canvas: Canvas::new(),
        }
    }

    fn step(&mut self) {
        self.tick += 1;
        if self.tick == self.vel_recip {
            // Every N ticks increase the position by 1.
            self.tick = 0;

            self.pos += 1;
            if self.pos == 160 {
                self.pos = 0;
            }
        }

        for (i, color) in self.canvas.as_slice_mut().iter_mut().enumerate() {
            if i == self.pos {
                *color = Color::white();
            } else {
                color.decay(self.decay);
            }
        }
    }
}
