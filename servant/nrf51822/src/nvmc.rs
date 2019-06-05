#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Ready flag."]
    pub ready: READY,
    _reserved1: [u8; 256usize],
    #[doc = "0x504 - Configuration register."]
    pub config: CONFIG,
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    pub erasepage: ERASEPAGE,
    #[doc = "0x50c - Register for erasing all non-volatile user memory."]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Register for erasing a protected non-volatile memory page."]
    pub eraseprotectedpage: ERASEPROTECTEDPAGE,
    #[doc = "0x514 - Register for start erasing User Information Congfiguration Registers."]
    pub eraseuicr: ERASEUICR,
}
#[doc = "Ready flag."]
pub struct READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready flag."]
pub mod ready;
#[doc = "Configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register."]
pub mod config;
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub struct ERASEPAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub mod erasepage;
#[doc = "Register for erasing all non-volatile user memory."]
pub struct ERASEALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing all non-volatile user memory."]
pub mod eraseall;
#[doc = "Register for erasing a protected non-volatile memory page."]
pub struct ERASEPROTECTEDPAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing a protected non-volatile memory page."]
pub mod eraseprotectedpage;
#[doc = "Register for start erasing User Information Congfiguration Registers."]
pub struct ERASEUICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for start erasing User Information Congfiguration Registers."]
pub mod eraseuicr;
