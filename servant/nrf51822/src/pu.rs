#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1536usize],
    #[doc = "0x600 - Address of first instruction to replace."]
    pub replaceaddr: [REPLACEADDR; 8],
    _reserved1: [u8; 96usize],
    #[doc = "0x680 - Relative address of patch instructions."]
    pub patchaddr: [PATCHADDR; 8],
    _reserved2: [u8; 96usize],
    #[doc = "0x700 - Patch enable register."]
    pub patchen: PATCHEN,
    #[doc = "0x704 - Patch enable register."]
    pub patchenset: PATCHENSET,
    #[doc = "0x708 - Patch disable register."]
    pub patchenclr: PATCHENCLR,
}
#[doc = "Address of first instruction to replace."]
pub struct REPLACEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address of first instruction to replace."]
pub mod replaceaddr;
#[doc = "Relative address of patch instructions."]
pub struct PATCHADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Relative address of patch instructions."]
pub mod patchaddr;
#[doc = "Patch enable register."]
pub struct PATCHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Patch enable register."]
pub mod patchen;
#[doc = "Patch enable register."]
pub struct PATCHENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Patch enable register."]
pub mod patchenset;
#[doc = "Patch disable register."]
pub struct PATCHENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Patch disable register."]
pub mod patchenclr;
