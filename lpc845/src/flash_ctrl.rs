#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Flash configuration register"]
    pub flashcfg: crate::Reg<flashcfg::FLASHCFG_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Flash signature start address register"]
    pub fmsstart: crate::Reg<fmsstart::FMSSTART_SPEC>,
    #[doc = "0x24 - Flash signaure stop address register"]
    pub fmsstop: crate::Reg<fmsstop::FMSSTOP_SPEC>,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
    pub fmsw0: crate::Reg<fmsw0::FMSW0_SPEC>,
    _reserved4: [u8; 4016usize],
    #[doc = "0xfe0 - Flash signature generation status bit"]
    pub fmstat: crate::Reg<fmstat::FMSTAT_SPEC>,
    _reserved5: [u8; 4usize],
    #[doc = "0xfe8 - Clear FLASH signature generation status bit"]
    pub fmstatclr: crate::Reg<fmstatclr::FMSTATCLR_SPEC>,
}
#[doc = "FLASHCFG register accessor: an alias for `Reg<FLASHCFG_SPEC>`"]
pub type FLASHCFG = crate::Reg<flashcfg::FLASHCFG_SPEC>;
#[doc = "Flash configuration register"]
pub mod flashcfg;
#[doc = "FMSSTART register accessor: an alias for `Reg<FMSSTART_SPEC>`"]
pub type FMSSTART = crate::Reg<fmsstart::FMSSTART_SPEC>;
#[doc = "Flash signature start address register"]
pub mod fmsstart;
#[doc = "FMSSTOP register accessor: an alias for `Reg<FMSSTOP_SPEC>`"]
pub type FMSSTOP = crate::Reg<fmsstop::FMSSTOP_SPEC>;
#[doc = "Flash signaure stop address register"]
pub mod fmsstop;
#[doc = "FMSW0 register accessor: an alias for `Reg<FMSW0_SPEC>`"]
pub type FMSW0 = crate::Reg<fmsw0::FMSW0_SPEC>;
#[doc = "Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
pub mod fmsw0;
#[doc = "FMSTAT register accessor: an alias for `Reg<FMSTAT_SPEC>`"]
pub type FMSTAT = crate::Reg<fmstat::FMSTAT_SPEC>;
#[doc = "Flash signature generation status bit"]
pub mod fmstat;
#[doc = "FMSTATCLR register accessor: an alias for `Reg<FMSTATCLR_SPEC>`"]
pub type FMSTATCLR = crate::Reg<fmstatclr::FMSTATCLR_SPEC>;
#[doc = "Clear FLASH signature generation status bit"]
pub mod fmstatclr;
