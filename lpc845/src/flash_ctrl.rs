#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Flash configuration register"]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Flash signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Flash signaure stop address register"]
    pub fmsstop: FMSSTOP,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
    pub fmsw0: FMSW0,
    _reserved3: [u8; 4016usize],
    #[doc = "0xfe0 - Flash signature generation status bit"]
    pub fmstat: FMSTAT,
    _reserved4: [u8; 4usize],
    #[doc = "0xfe8 - Clear FLASH signature generation status bit"]
    pub fmstatclr: FMSTATCLR,
}
#[doc = "Flash configuration register"]
pub struct FLASHCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash configuration register"]
pub mod flashcfg;
#[doc = "Flash signature start address register"]
pub struct FMSSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash signature start address register"]
pub mod fmsstart;
#[doc = "Flash signaure stop address register"]
pub struct FMSSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash signaure stop address register"]
pub mod fmsstop;
#[doc = "Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
pub struct FMSW0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
pub mod fmsw0;
#[doc = "Flash signature generation status bit"]
pub struct FMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash signature generation status bit"]
pub mod fmstat;
#[doc = "Clear FLASH signature generation status bit"]
pub struct FMSTATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear FLASH signature generation status bit"]
pub mod fmstatclr;
