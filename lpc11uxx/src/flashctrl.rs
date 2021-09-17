#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Flash memory access time configuration register"]
    pub flashcfg: crate::Reg<flashcfg::FLASHCFG_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: crate::Reg<fmsstart::FMSSTART_SPEC>,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: crate::Reg<fmsstop::FMSSTOP_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x2c - Word 0 \\[31:0\\]"]
    pub fmsw0: crate::Reg<fmsw0::FMSW0_SPEC>,
    #[doc = "0x30 - Word 1 \\[63:32\\]"]
    pub fmsw1: crate::Reg<fmsw1::FMSW1_SPEC>,
    #[doc = "0x34 - Word 2 \\[95:64\\]"]
    pub fmsw2: crate::Reg<fmsw2::FMSW2_SPEC>,
    #[doc = "0x38 - Word 3 \\[127:96\\]"]
    pub fmsw3: crate::Reg<fmsw3::FMSW3_SPEC>,
    _reserved7: [u8; 0x60],
    #[doc = "0x9c - EEPROM BIST start address register"]
    pub eemsstart: crate::Reg<eemsstart::EEMSSTART_SPEC>,
    #[doc = "0xa0 - EEPROM BIST stop address register"]
    pub eemsstop: crate::Reg<eemsstop::EEMSSTOP_SPEC>,
    #[doc = "0xa4 - EEPROM 24-bit BIST signature register"]
    pub eemssig: crate::Reg<eemssig::EEMSSIG_SPEC>,
    _reserved10: [u8; 0x0f38],
    #[doc = "0xfe0 - Signature generation status register"]
    pub fmstat: crate::Reg<fmstat::FMSTAT_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub fmstatclr: crate::Reg<fmstatclr::FMSTATCLR_SPEC>,
}
#[doc = "EEMSSTART register accessor: an alias for `Reg<EEMSSTART_SPEC>`"]
pub type EEMSSTART = crate::Reg<eemsstart::EEMSSTART_SPEC>;
#[doc = "EEPROM BIST start address register"]
pub mod eemsstart;
#[doc = "EEMSSTOP register accessor: an alias for `Reg<EEMSSTOP_SPEC>`"]
pub type EEMSSTOP = crate::Reg<eemsstop::EEMSSTOP_SPEC>;
#[doc = "EEPROM BIST stop address register"]
pub mod eemsstop;
#[doc = "EEMSSIG register accessor: an alias for `Reg<EEMSSIG_SPEC>`"]
pub type EEMSSIG = crate::Reg<eemssig::EEMSSIG_SPEC>;
#[doc = "EEPROM 24-bit BIST signature register"]
pub mod eemssig;
#[doc = "FLASHCFG register accessor: an alias for `Reg<FLASHCFG_SPEC>`"]
pub type FLASHCFG = crate::Reg<flashcfg::FLASHCFG_SPEC>;
#[doc = "Flash memory access time configuration register"]
pub mod flashcfg;
#[doc = "FMSSTART register accessor: an alias for `Reg<FMSSTART_SPEC>`"]
pub type FMSSTART = crate::Reg<fmsstart::FMSSTART_SPEC>;
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "FMSSTOP register accessor: an alias for `Reg<FMSSTOP_SPEC>`"]
pub type FMSSTOP = crate::Reg<fmsstop::FMSSTOP_SPEC>;
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "FMSW0 register accessor: an alias for `Reg<FMSW0_SPEC>`"]
pub type FMSW0 = crate::Reg<fmsw0::FMSW0_SPEC>;
#[doc = "Word 0 \\[31:0\\]"]
pub mod fmsw0;
#[doc = "FMSW1 register accessor: an alias for `Reg<FMSW1_SPEC>`"]
pub type FMSW1 = crate::Reg<fmsw1::FMSW1_SPEC>;
#[doc = "Word 1 \\[63:32\\]"]
pub mod fmsw1;
#[doc = "FMSW2 register accessor: an alias for `Reg<FMSW2_SPEC>`"]
pub type FMSW2 = crate::Reg<fmsw2::FMSW2_SPEC>;
#[doc = "Word 2 \\[95:64\\]"]
pub mod fmsw2;
#[doc = "FMSW3 register accessor: an alias for `Reg<FMSW3_SPEC>`"]
pub type FMSW3 = crate::Reg<fmsw3::FMSW3_SPEC>;
#[doc = "Word 3 \\[127:96\\]"]
pub mod fmsw3;
#[doc = "FMSTAT register accessor: an alias for `Reg<FMSTAT_SPEC>`"]
pub type FMSTAT = crate::Reg<fmstat::FMSTAT_SPEC>;
#[doc = "Signature generation status register"]
pub mod fmstat;
#[doc = "FMSTATCLR register accessor: an alias for `Reg<FMSTATCLR_SPEC>`"]
pub type FMSTATCLR = crate::Reg<fmstatclr::FMSTATCLR_SPEC>;
#[doc = "Signature generation status clear register"]
pub mod fmstatclr;
