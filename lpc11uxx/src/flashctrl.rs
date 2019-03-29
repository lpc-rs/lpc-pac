#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Flash memory access time configuration register"]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Word 0 \\[31:0\\]"]
    pub fmsw0: FMSW0,
    #[doc = "0x30 - Word 1 \\[63:32\\]"]
    pub fmsw1: FMSW1,
    #[doc = "0x34 - Word 2 \\[95:64\\]"]
    pub fmsw2: FMSW2,
    #[doc = "0x38 - Word 3 \\[127:96\\]"]
    pub fmsw3: FMSW3,
    _reserved7: [u8; 96usize],
    #[doc = "0x9c - EEPROM BIST start address register"]
    pub eemsstart: EEMSSTART,
    #[doc = "0xa0 - EEPROM BIST stop address register"]
    pub eemsstop: EEMSSTOP,
    #[doc = "0xa4 - EEPROM 24-bit BIST signature register"]
    pub eemssig: EEMSSIG,
    _reserved10: [u8; 3896usize],
    #[doc = "0xfe0 - Signature generation status register"]
    pub fmstat: FMSTAT,
    _reserved11: [u8; 4usize],
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub fmstatclr: FMSTATCLR,
}
#[doc = "EEPROM BIST start address register"]
pub struct EEMSSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM BIST start address register"]
pub mod eemsstart;
#[doc = "EEPROM BIST stop address register"]
pub struct EEMSSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM BIST stop address register"]
pub mod eemsstop;
#[doc = "EEPROM 24-bit BIST signature register"]
pub struct EEMSSIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM 24-bit BIST signature register"]
pub mod eemssig;
#[doc = "Flash memory access time configuration register"]
pub struct FLASHCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash memory access time configuration register"]
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
#[doc = "Word 0 \\[31:0\\]"]
pub struct FMSW0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word 0 \\[31:0\\]"]
pub mod fmsw0;
#[doc = "Word 1 \\[63:32\\]"]
pub struct FMSW1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word 1 \\[63:32\\]"]
pub mod fmsw1;
#[doc = "Word 2 \\[95:64\\]"]
pub struct FMSW2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word 2 \\[95:64\\]"]
pub mod fmsw2;
#[doc = "Word 3 \\[127:96\\]"]
pub struct FMSW3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word 3 \\[127:96\\]"]
pub mod fmsw3;
#[doc = "Signature generation status register"]
pub struct FMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature generation status register"]
pub mod fmstat;
#[doc = "Signature generation status clear register"]
pub struct FMSTATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature generation status clear register"]
pub mod fmstatclr;
