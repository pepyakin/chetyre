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
        let mut cursor = Cursor::new(4);
        let mut decay = Decay::new(1, (4, 4, 4));

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
                cursor.step(&mut canvas);
                decay.step(&mut canvas);
                // CANVAS.invert();

                // for _ in 0..100_000_0 {
                //     cortex_m::asm::nop();
                // }

                // Flush the data to the strip.
                canvas.flush(&p.GPIO);
            }
        }
    }

    loop {}
}

struct Cursor {
    /// Every so much ticks cursor moves by 1.
    vel_recip: usize,
    tick: usize,
    pos: usize,
}

impl Cursor {
    fn new(vel_recip: usize) -> Self {
        Cursor {
            tick: 0,
            vel_recip,
            pos: 0,
        }
    }

    fn step(&mut self, canvas: &mut Canvas) {
        self.tick += 1;
        if self.tick == self.vel_recip {
            // Every N ticks increase the position by 1.
            self.tick = 0;

            self.pos += 1;
            if self.pos == 60 {
                self.pos = 0;
            }
        }

        // This should be safe to unwrap since we limit the position.
        *canvas.at_mut(self.pos).unwrap() = Color::white();
    }
}

struct Decay {
    tick: usize,
    /// How quickly the image decays.
    vel_recip: usize,
    decay_amt: (u8, u8, u8),
}

impl Decay {
    fn new(vel_recip: usize, decay_amt: (u8, u8, u8)) -> Self {
        Decay {
            tick: 0,
            vel_recip,
            decay_amt,
        }
    }

    fn step(&mut self, canvas: &mut Canvas) {
        self.tick += 1;
        if self.tick == self.vel_recip {
            // Every N ticks decay the colors.
            self.tick = 0;

            for color in canvas.as_slice_mut().iter_mut() {
                color.decay(self.decay_amt.0, self.decay_amt.1, self.decay_amt.1);
            }
        }
    }
}
