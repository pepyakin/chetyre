use nrf51822::{Peripherals, UART0};

pub fn init(p: &Peripherals) {
    p.GPIO.pin_cnf[24].write(|w| w.pull().pullup().dir().output());
    p.GPIO.pin_cnf[25].write(|w| w.pull().disabled().dir().input());

    p.UART0.pseltxd.write(|w| unsafe { w.bits(24) });
    p.UART0.pselrxd.write(|w| unsafe { w.bits(25) });

    p.UART0.baudrate.write(|w| w.baudrate().baud115200());
    p.UART0.enable.write(|w| w.enable().enabled());
}

pub fn write_str(uart0: &UART0, s: &str) -> core::fmt::Result {
    uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });
    for c in s.as_bytes() {
        /* Write the current character to the output register */
        uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });

        /* Wait until the UART is clear to send */
        while uart0.events_txdrdy.read().bits() == 0 {}

        /* And then set it back to 0 again, just because ?!? */
        uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
    }
    uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    Ok(())
}
