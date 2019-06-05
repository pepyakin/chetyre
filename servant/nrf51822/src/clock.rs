#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK clock source."]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFCLK clock source."]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK clock source."]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK clock source."]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    #[doc = "0x10 - Start calibration of LFCLK RC oscillator."]
    pub tasks_cal: TASKS_CAL,
    #[doc = "0x14 - Start calibration timer."]
    pub tasks_ctstart: TASKS_CTSTART,
    #[doc = "0x18 - Stop calibration timer."]
    pub tasks_ctstop: TASKS_CTSTOP,
    _reserved0: [u8; 228usize],
    #[doc = "0x100 - HFCLK oscillator started."]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK oscillator started."]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved1: [u8; 4usize],
    #[doc = "0x10c - Callibration of LFCLK RC oscillator completed."]
    pub events_done: EVENTS_DONE,
    #[doc = "0x110 - Callibration timer timeout."]
    pub events_ctto: EVENTS_CTTO,
    _reserved2: [u8; 496usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 256usize],
    #[doc = "0x40c - High frequency clock status."]
    pub hfclkstat: HFCLKSTAT,
    _reserved4: [u8; 8usize],
    #[doc = "0x418 - Low frequency clock status."]
    pub lfclkstat: LFCLKSTAT,
    _reserved5: [u8; 252usize],
    #[doc = "0x518 - Clock source for the LFCLK clock."]
    pub lfclksrc: LFCLKSRC,
    _reserved6: [u8; 28usize],
    #[doc = "0x538 - Calibration timer interval."]
    pub ctiv: CTIV,
    _reserved7: [u8; 20usize],
    #[doc = "0x550 - Crystal frequency."]
    pub xtalfreq: XTALFREQ,
}
#[doc = "Start HFCLK clock source."]
pub struct TASKS_HFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start HFCLK clock source."]
pub mod tasks_hfclkstart;
#[doc = "Stop HFCLK clock source."]
pub struct TASKS_HFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop HFCLK clock source."]
pub mod tasks_hfclkstop;
#[doc = "Start LFCLK clock source."]
pub struct TASKS_LFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start LFCLK clock source."]
pub mod tasks_lfclkstart;
#[doc = "Stop LFCLK clock source."]
pub struct TASKS_LFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop LFCLK clock source."]
pub mod tasks_lfclkstop;
#[doc = "Start calibration of LFCLK RC oscillator."]
pub struct TASKS_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start calibration of LFCLK RC oscillator."]
pub mod tasks_cal;
#[doc = "Start calibration timer."]
pub struct TASKS_CTSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start calibration timer."]
pub mod tasks_ctstart;
#[doc = "Stop calibration timer."]
pub struct TASKS_CTSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop calibration timer."]
pub mod tasks_ctstop;
#[doc = "HFCLK oscillator started."]
pub struct EVENTS_HFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFCLK oscillator started."]
pub mod events_hfclkstarted;
#[doc = "LFCLK oscillator started."]
pub struct EVENTS_LFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFCLK oscillator started."]
pub mod events_lfclkstarted;
#[doc = "Callibration of LFCLK RC oscillator completed."]
pub struct EVENTS_DONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Callibration of LFCLK RC oscillator completed."]
pub mod events_done;
#[doc = "Callibration timer timeout."]
pub struct EVENTS_CTTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Callibration timer timeout."]
pub mod events_ctto;
#[doc = "Interrupt enable set register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "Interrupt enable clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "High frequency clock status."]
pub struct HFCLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High frequency clock status."]
pub mod hfclkstat;
#[doc = "Low frequency clock status."]
pub struct LFCLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low frequency clock status."]
pub mod lfclkstat;
#[doc = "Clock source for the LFCLK clock."]
pub struct LFCLKSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock source for the LFCLK clock."]
pub mod lfclksrc;
#[doc = "Calibration timer interval."]
pub struct CTIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration timer interval."]
pub mod ctiv;
#[doc = "Crystal frequency."]
pub struct XTALFREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crystal frequency."]
pub mod xtalfreq;
