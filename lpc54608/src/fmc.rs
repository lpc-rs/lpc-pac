#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub fctr: crate::Reg<fctr::FCTR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Wait state register"]
    pub fbwst: crate::Reg<fbwst::FBWST_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: crate::Reg<fmsstart::FMSSTART_SPEC>,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: crate::Reg<fmsstop::FMSSTOP_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x2c..0x3c - Words of 128-bit signature word"]
    pub fmsw: [crate::Reg<fmsw::FMSW_SPEC>; 4],
    _reserved5: [u8; 0x0fa4],
    #[doc = "0xfe0 - Signature generation status register"]
    pub fmstat: crate::Reg<fmstat::FMSTAT_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub fmstatclr: crate::Reg<fmstatclr::FMSTATCLR_SPEC>,
}
#[doc = "FCTR register accessor: an alias for `Reg<FCTR_SPEC>`"]
pub type FCTR = crate::Reg<fctr::FCTR_SPEC>;
#[doc = "Control register"]
pub mod fctr;
#[doc = "FBWST register accessor: an alias for `Reg<FBWST_SPEC>`"]
pub type FBWST = crate::Reg<fbwst::FBWST_SPEC>;
#[doc = "Wait state register"]
pub mod fbwst;
#[doc = "FMSSTART register accessor: an alias for `Reg<FMSSTART_SPEC>`"]
pub type FMSSTART = crate::Reg<fmsstart::FMSSTART_SPEC>;
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "FMSSTOP register accessor: an alias for `Reg<FMSSTOP_SPEC>`"]
pub type FMSSTOP = crate::Reg<fmsstop::FMSSTOP_SPEC>;
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "FMSW register accessor: an alias for `Reg<FMSW_SPEC>`"]
pub type FMSW = crate::Reg<fmsw::FMSW_SPEC>;
#[doc = "Words of 128-bit signature word"]
pub mod fmsw;
#[doc = "FMSTAT register accessor: an alias for `Reg<FMSTAT_SPEC>`"]
pub type FMSTAT = crate::Reg<fmstat::FMSTAT_SPEC>;
#[doc = "Signature generation status register"]
pub mod fmstat;
#[doc = "FMSTATCLR register accessor: an alias for `Reg<FMSTATCLR_SPEC>`"]
pub type FMSTATCLR = crate::Reg<fmstatclr::FMSTATCLR_SPEC>;
#[doc = "Signature generation status clear register"]
pub mod fmstatclr;
