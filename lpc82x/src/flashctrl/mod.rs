#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Flash configuration register"]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - Signature Word"]
    pub fmsw0: FMSW0,
}
#[doc = "Flash configuration register"]
pub struct FLASHCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash configuration register"]
pub mod flashcfg;
#[doc = "Signature start address register"]
pub struct FMSSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "Signature stop-address register"]
pub struct FMSSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "Signature Word"]
pub struct FMSW0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature Word"]
pub mod fmsw0;
