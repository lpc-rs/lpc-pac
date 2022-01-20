#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPIFI control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - SPIFI command register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x08 - SPIFI address register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x0c - SPIFI intermediate data register"]
    pub idata: crate::Reg<idata::IDATA_SPEC>,
    #[doc = "0x10 - SPIFI limit register"]
    pub climit: crate::Reg<climit::CLIMIT_SPEC>,
    #[doc = "0x14 - SPIFI data register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x18 - SPIFI memory command register"]
    pub mcmd: crate::Reg<mcmd::MCMD_SPEC>,
    #[doc = "0x1c - SPIFI status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPIFI control register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SPIFI command register"]
pub mod cmd;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPIFI address register"]
pub mod addr;
#[doc = "IDATA register accessor: an alias for `Reg<IDATA_SPEC>`"]
pub type IDATA = crate::Reg<idata::IDATA_SPEC>;
#[doc = "SPIFI intermediate data register"]
pub mod idata;
#[doc = "CLIMIT register accessor: an alias for `Reg<CLIMIT_SPEC>`"]
pub type CLIMIT = crate::Reg<climit::CLIMIT_SPEC>;
#[doc = "SPIFI limit register"]
pub mod climit;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "SPIFI data register"]
pub mod data;
#[doc = "MCMD register accessor: an alias for `Reg<MCMD_SPEC>`"]
pub type MCMD = crate::Reg<mcmd::MCMD_SPEC>;
#[doc = "SPIFI memory command register"]
pub mod mcmd;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPIFI status register"]
pub mod stat;
