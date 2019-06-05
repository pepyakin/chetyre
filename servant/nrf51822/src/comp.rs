#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the comparator."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop the comparator."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Sample comparator value."]
    pub tasks_sample: TASKS_SAMPLE,
    _reserved0: [u8; 244usize],
    #[doc = "0x100 - COMP is ready and output is valid."]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Input voltage crossed the threshold going down."]
    pub events_down: EVENTS_DOWN,
    #[doc = "0x108 - Input voltage crossed the threshold going up."]
    pub events_up: EVENTS_UP,
    #[doc = "0x10c - Input voltage crossed the threshold in any direction."]
    pub events_cross: EVENTS_CROSS,
    _reserved1: [u8; 240usize],
    #[doc = "0x200 - Shortcut for the COMP."]
    pub shorts: SHORTS,
    _reserved2: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 244usize],
    #[doc = "0x400 - Compare result."]
    pub result: RESULT,
    _reserved4: [u8; 252usize],
    #[doc = "0x500 - Enable the COMP."]
    pub enable: ENABLE,
    #[doc = "0x504 - Input pin select."]
    pub psel: PSEL,
    #[doc = "0x508 - Reference select."]
    pub refsel: REFSEL,
    #[doc = "0x50c - External reference select."]
    pub extrefsel: EXTREFSEL,
    _reserved5: [u8; 32usize],
    #[doc = "0x530 - Threshold configuration for hysteresis unit."]
    pub th: TH,
    #[doc = "0x534 - Mode configuration."]
    pub mode: MODE,
}
#[doc = "Start the comparator."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the comparator."]
pub mod tasks_start;
#[doc = "Stop the comparator."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the comparator."]
pub mod tasks_stop;
#[doc = "Sample comparator value."]
pub struct TASKS_SAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample comparator value."]
pub mod tasks_sample;
#[doc = "COMP is ready and output is valid."]
pub struct EVENTS_READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "COMP is ready and output is valid."]
pub mod events_ready;
#[doc = "Input voltage crossed the threshold going down."]
pub struct EVENTS_DOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input voltage crossed the threshold going down."]
pub mod events_down;
#[doc = "Input voltage crossed the threshold going up."]
pub struct EVENTS_UP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input voltage crossed the threshold going up."]
pub mod events_up;
#[doc = "Input voltage crossed the threshold in any direction."]
pub struct EVENTS_CROSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input voltage crossed the threshold in any direction."]
pub mod events_cross;
#[doc = "Shortcut for the COMP."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcut for the COMP."]
pub mod shorts;
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
#[doc = "Compare result."]
pub struct RESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare result."]
pub mod result;
#[doc = "Enable the COMP."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable the COMP."]
pub mod enable;
#[doc = "Input pin select."]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input pin select."]
pub mod psel;
#[doc = "Reference select."]
pub struct REFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reference select."]
pub mod refsel;
#[doc = "External reference select."]
pub struct EXTREFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External reference select."]
pub mod extrefsel;
#[doc = "Threshold configuration for hysteresis unit."]
pub struct TH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Threshold configuration for hysteresis unit."]
pub mod th;
#[doc = "Mode configuration."]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode configuration."]
pub mod mode;
