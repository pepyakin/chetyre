
use nrf51822::GPIO;

/// I have 1m 60LEDs. 1 LED consits of 3 sub-LEDs, one for each color.
const LED_COUNT: usize = 60;

/// A color represented in 24-bit GRB format (the same at the strip).
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Color {
    pub g: u8,
    pub r: u8,
    pub b: u8,
}

impl Color {
    pub const fn black() -> Color {
        Color { g: 0, r: 0, b: 0 }
    }

    pub const fn white() -> Color {
        Color { g: 255, r: 255, b: 255 }
    }

    /// Invert all components.
    pub fn invert(&mut self) {
        self.g = 255 - self.g;
        self.r = 255 - self.r;
        self.b = 255 - self.b;
    }

    /// Substract all components by the specific amount.
    pub fn decay(&mut self, g: u8, r: u8, b: u8) {
        self.g = self.g.saturating_sub(g);
        self.r = self.r.saturating_sub(r);
        self.b = self.b.saturating_sub(b);
    }
}

/// A canvas abstracts painting over the strip.
///
/// This struct accumulates changes and only sends them to the strip when `flush` is called.
pub struct Canvas {
    pin: usize,
    buf: [Color; LED_COUNT],
}

impl Canvas {
    pub const fn new() -> Self {
        Canvas {
            pin: 1,
            buf: [Color::black(); LED_COUNT],
        }
    }

    /// Flush this canvas to LED.
    pub fn flush(&self, gpio: &GPIO) {
        extern "C" {
            fn send_buf_ws2818(
                unused: usize,
                mask: usize,
                clraddr: *const usize,
                setaddr: *const usize,
                ptr: *const u8,
                length: usize,
            );
        }

        const BUF_LEN: usize = LED_COUNT * 3;
        let pin_mask = 1 << self.pin;
        let buf_ptr = self.buf.as_ptr() as *const _;

        // These two are addresses of registers that on write to them either clear or set the
        // specific bit respective bit. They are device specific and could be hardcoded in theory
        // although that wouldn't be too benificial.
        let gpio_clr_ptr = &gpio.outclr as *const _ as *const usize;
        let gpio_set_ptr = &gpio.outset as *const _ as *const usize;

        unsafe {
            send_buf_ws2818(
                0, // unused
                pin_mask,
                gpio_clr_ptr,
                gpio_set_ptr,
                buf_ptr,
                // HardFault is raised without -1 when reading R4 on send_buffer.S:85, LOL.
                BUF_LEN - 1,
            );
        }
    }

    /// Set all LEDs to black.
    pub fn clear(&mut self) {
        self.buf.iter_mut().for_each(|v| *v = Color::black());
    }

    /// Invert all colors.
    pub fn invert(&mut self) {
        self.buf.iter_mut().for_each(|v| v.invert());
    }

    /// Get a mutable reference to the color that corresponds to the given position.
    pub fn at_mut(&mut self, idx: usize) -> Option<&mut Color> {
        self.buf.get_mut(idx)
    }

    /// Returns the backing buffer as a slice.
    pub fn as_slice_mut(&mut self) -> &mut [Color] {
        &mut self.buf
    }

    /// Set the LED by the given `idx` to the specified `color`.
    pub fn set_color(&mut self, idx: usize, color: Color) {
        self.buf[idx] = color;
    }
}
