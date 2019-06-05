#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel group tasks."]
    pub tasks_chg0: TASKS_CHG,
    #[doc = "0x08 - Channel group tasks."]
    pub tasks_chg2: TASKS_CHG,
    _reserved0: [u8; 1264usize],
    #[doc = "0x500 - Channel enable."]
    pub chen: CHEN,
    #[doc = "0x504 - Channel enable set."]
    pub chenset: CHENSET,
    #[doc = "0x508 - Channel enable clear."]
    pub chenclr: CHENCLR,
    _reserved1: [u8; 4usize],
    #[doc = "0x510 - PPI Channel."]
    pub ch0: CH,
    #[doc = "0x518 - PPI Channel."]
    pub ch2: CH,
    #[doc = "0x520 - PPI Channel."]
    pub ch4: CH,
    #[doc = "0x528 - PPI Channel."]
    pub ch6: CH,
    #[doc = "0x530 - PPI Channel."]
    pub ch8: CH,
    #[doc = "0x538 - PPI Channel."]
    pub ch10: CH,
    #[doc = "0x540 - PPI Channel."]
    pub ch12: CH,
    #[doc = "0x548 - PPI Channel."]
    pub ch14: CH,
    _reserved2: [u8; 688usize],
    #[doc = "0x800 - Channel group configuration."]
    pub chg: [CHG; 4],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Enable channel group."]
    pub en: self::tasks_chg::EN,
    #[doc = "0x04 - Disable channel group."]
    pub dis: self::tasks_chg::DIS,
}
#[doc = r" Register block"]
#[doc = "Channel group tasks."]
pub mod tasks_chg;
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Channel event end-point."]
    pub eep: self::ch::EEP,
    #[doc = "0x04 - Channel task end-point."]
    pub tep: self::ch::TEP,
}
#[doc = r" Register block"]
#[doc = "PPI Channel."]
pub mod ch;
#[doc = "Channel enable."]
pub struct CHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable."]
pub mod chen;
#[doc = "Channel enable set."]
pub struct CHENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable set."]
pub mod chenset;
#[doc = "Channel enable clear."]
pub struct CHENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable clear."]
pub mod chenclr;
#[doc = "Channel group configuration."]
pub struct CHG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel group configuration."]
pub mod chg;
