#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Length of code region 0."]
    pub clenr0: CLENR0,
    #[doc = "0x04 - Readback protection configuration."]
    pub rbpconf: RBPCONF,
    #[doc = "0x08 - Reset value for CLOCK XTALFREQ register."]
    pub xtalfreq: XTALFREQ,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Firmware ID."]
    pub fwid: FWID,
}
#[doc = "Length of code region 0."]
pub struct CLENR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length of code region 0."]
pub mod clenr0;
#[doc = "Readback protection configuration."]
pub struct RBPCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Readback protection configuration."]
pub mod rbpconf;
#[doc = "Reset value for CLOCK XTALFREQ register."]
pub struct XTALFREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset value for CLOCK XTALFREQ register."]
pub mod xtalfreq;
#[doc = "Firmware ID."]
pub struct FWID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Firmware ID."]
pub mod fwid;
