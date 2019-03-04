#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Voltage ladder register"]
    pub lad: LAD,
}
#[doc = "Comparator control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator control register"]
pub mod ctrl;
#[doc = "Voltage ladder register"]
pub struct LAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Voltage ladder register"]
pub mod lad;
