#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Command/Status register"]
    pub devcmdstat: crate::Reg<devcmdstat::DEVCMDSTAT_SPEC>,
    #[doc = "0x04 - USB Info register"]
    pub info: crate::Reg<info::INFO_SPEC>,
    #[doc = "0x08 - USB EP Command/Status List start address"]
    pub epliststart: crate::Reg<epliststart::EPLISTSTART_SPEC>,
    #[doc = "0x0c - USB Data buffer start address"]
    pub databufstart: crate::Reg<databufstart::DATABUFSTART_SPEC>,
    #[doc = "0x10 - USB Link Power Management register"]
    pub lpm: crate::Reg<lpm::LPM_SPEC>,
    #[doc = "0x14 - USB Endpoint skip"]
    pub epskip: crate::Reg<epskip::EPSKIP_SPEC>,
    #[doc = "0x18 - USB Endpoint Buffer in use"]
    pub epinuse: crate::Reg<epinuse::EPINUSE_SPEC>,
    #[doc = "0x1c - USB Endpoint Buffer Configuration register"]
    pub epbufcfg: crate::Reg<epbufcfg::EPBUFCFG_SPEC>,
    #[doc = "0x20 - USB interrupt status register"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x24 - USB interrupt enable register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x28 - USB set interrupt status register"]
    pub intsetstat: crate::Reg<intsetstat::INTSETSTAT_SPEC>,
    _reserved11: [u8; 8usize],
    #[doc = "0x34 - USB Endpoint toggle register"]
    pub eptoggle: crate::Reg<eptoggle::EPTOGGLE_SPEC>,
}
#[doc = "DEVCMDSTAT register accessor: an alias for `Reg<DEVCMDSTAT_SPEC>`"]
pub type DEVCMDSTAT = crate::Reg<devcmdstat::DEVCMDSTAT_SPEC>;
#[doc = "USB Device Command/Status register"]
pub mod devcmdstat;
#[doc = "INFO register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "USB Info register"]
pub mod info;
#[doc = "EPLISTSTART register accessor: an alias for `Reg<EPLISTSTART_SPEC>`"]
pub type EPLISTSTART = crate::Reg<epliststart::EPLISTSTART_SPEC>;
#[doc = "USB EP Command/Status List start address"]
pub mod epliststart;
#[doc = "DATABUFSTART register accessor: an alias for `Reg<DATABUFSTART_SPEC>`"]
pub type DATABUFSTART = crate::Reg<databufstart::DATABUFSTART_SPEC>;
#[doc = "USB Data buffer start address"]
pub mod databufstart;
#[doc = "LPM register accessor: an alias for `Reg<LPM_SPEC>`"]
pub type LPM = crate::Reg<lpm::LPM_SPEC>;
#[doc = "USB Link Power Management register"]
pub mod lpm;
#[doc = "EPSKIP register accessor: an alias for `Reg<EPSKIP_SPEC>`"]
pub type EPSKIP = crate::Reg<epskip::EPSKIP_SPEC>;
#[doc = "USB Endpoint skip"]
pub mod epskip;
#[doc = "EPINUSE register accessor: an alias for `Reg<EPINUSE_SPEC>`"]
pub type EPINUSE = crate::Reg<epinuse::EPINUSE_SPEC>;
#[doc = "USB Endpoint Buffer in use"]
pub mod epinuse;
#[doc = "EPBUFCFG register accessor: an alias for `Reg<EPBUFCFG_SPEC>`"]
pub type EPBUFCFG = crate::Reg<epbufcfg::EPBUFCFG_SPEC>;
#[doc = "USB Endpoint Buffer Configuration register"]
pub mod epbufcfg;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "USB interrupt status register"]
pub mod intstat;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "USB interrupt enable register"]
pub mod inten;
#[doc = "INTSETSTAT register accessor: an alias for `Reg<INTSETSTAT_SPEC>`"]
pub type INTSETSTAT = crate::Reg<intsetstat::INTSETSTAT_SPEC>;
#[doc = "USB set interrupt status register"]
pub mod intsetstat;
#[doc = "EPTOGGLE register accessor: an alias for `Reg<EPTOGGLE_SPEC>`"]
pub type EPTOGGLE = crate::Reg<eptoggle::EPTOGGLE_SPEC>;
#[doc = "USB Endpoint toggle register"]
pub mod eptoggle;
