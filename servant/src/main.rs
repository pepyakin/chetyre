#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf51822::{Peripherals, UART0};
use panic_halt as _;

mod canvas;
mod serial;

use canvas::{Canvas, Color};

/// I have 1m 60LEDs. 1 LED consits of 3 sub-LEDs, one for each color.
const LED_COUNT: usize = 59;

fn read_color(uart0: &UART0) -> Color {
    let r = serial::read_u8(uart0);
    let g = serial::read_u8(uart0);
    let b = serial::read_u8(uart0);

    Color { g, r, b }
}

struct Timer {
    tick: usize,
    period: usize,
}

impl Timer {
    fn new(period: usize) -> Self {
        Timer { tick: 0, period }
    }

    fn tick<F: FnOnce()>(&mut self, f: F) {
        self.tick += 1;
        if self.tick == self.period {
            self.tick = 0;
            f()
        }
    }
}

struct Cursor {
    timer: Timer,
    pos: isize,
    dx: isize,
}

impl Cursor {
    /// offset - starting position.
    /// vel_recip - number of ticks after which cursor moves by 1.
    /// dx - direction of movement.
    fn new(offset: usize, vel_recip: usize, dx: isize) -> Self {
        Cursor {
            timer: Timer::new(vel_recip),
            pos: offset as isize,
            dx,
        }
    }

    fn step(&mut self, canvas: &mut Canvas) {
        let pos = &mut self.pos;
        let dx = self.dx;

        self.timer.tick(|| {
            *pos = *pos + dx;
            if *pos < 0 {
                *pos = LED_COUNT as isize;
            } else if *pos == LED_COUNT as isize {
                *pos = 0;
            }
        });

        // This should be safe to unwrap since we limit the position.
        *canvas.at_mut(self.pos as usize).unwrap() = Color::white();
    }
}

struct Decay {
    timer: Timer,
    decay_amt: (u8, u8, u8),
}

impl Decay {
    fn new(vel_recip: usize, decay_amt: (u8, u8, u8)) -> Self {
        Decay {
            timer: Timer::new(vel_recip),
            decay_amt,
        }
    }

    fn step(&mut self, canvas: &mut Canvas) {
        let decay_amt = &self.decay_amt;
        self.timer.tick(|| {
            for color in canvas.as_slice_mut().iter_mut() {
                color.decay(decay_amt.0, decay_amt.1, decay_amt.2);
            }
        });
    }
}

struct Impulse {
    timer: Timer,
}

impl Impulse {
    fn new(period: usize) -> Self {
        Impulse {
            timer: Timer::new(period),
        }
    }

    fn step(&mut self, canvas: &mut Canvas) {
        self.timer.tick(|| {
            for color in canvas.as_slice_mut().iter_mut() {
                *color = Color::white();
            }
        });
    }
}

/// XorShift32Rng is a very simple PRNG.
struct XorShift32Rng {
    state: u32,
}

impl XorShift32Rng {
    pub fn new(seed: u32) -> Self {
        XorShift32Rng { state: seed }
    }

    /// Get the next pseudo random number.
    pub fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

pub struct RandomDots {
    num: usize,
}

impl RandomDots {
    fn new(num: usize) -> Self {
        RandomDots { num }
    }

    fn step(&self, rng: &mut XorShift32Rng, canvas: &mut Canvas) {
        for _ in 0..self.num {
            let pos = rng.next() as usize % LED_COUNT;

            // Unwrap is safe here since we limit the value to LED count.
            *canvas.at_mut(pos).unwrap() = Color::white();
        }
    }
}

fn trails_pattern(p: &Peripherals) {
    let mut canvas = Canvas::new();
    let mut cur1 = Cursor::new(30, 8, 1);
    // let mut cur2 = Cursor::new(0, 8, 1);
    let mut decay = Decay::new(1, (4, 4, 4));

    loop {
        serial::write_str(&p.UART0, b"flush\r\n");

        decay.step(&mut canvas);
        cur1.step(&mut canvas);
        // cur2.step(&mut canvas);
        // CANVAS.invert();

        // for _ in 0..100_000_0 {
        //     cortex_m::asm::nop();
        // }

        // Flush the data to the strip.
        canvas.flush(&p.GPIO);
    }
}

fn random_dots(p: &Peripherals) {
    let mut canvas = Canvas::new();
    let mut dots = RandomDots::new(2);
    let mut decay = Decay::new(2, (4, 4, 4));
    let mut rng = XorShift32Rng::new(1337);

    let mut tick = 0;

    loop {
        serial::write_str(&p.UART0, b"flush\r\n");

        tick += 1;
        if tick == 75 {
            tick = 0;
            dots.step(&mut rng, &mut canvas);
        }
        decay.step(&mut canvas);

        // Flush the data to the strip.
        canvas.flush(&p.GPIO);
    }
}

fn impulses(p: &Peripherals) {
    let mut canvas = Canvas::new();
    let mut impulse = Impulse::new(350);
    let mut decay = Decay::new(3, (4, 4, 4));

    let mut impulse_timer = Timer::new(350);
    let mut decay_timer = Timer::new(3);

    loop {
        serial::write_str(&p.UART0, b"flush\r\n");

        impulse_timer.tick(|| impulse.step(&mut canvas));
        decay_timer.tick(|| decay.step(&mut canvas));

        // Flush the data to the strip.
        canvas.flush(&p.GPIO);
    }
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

        // trails_pattern(&p);
        random_dots(&p);
        // impulses(&p);
    }

    loop {
        continue;
    }
}
