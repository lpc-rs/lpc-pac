///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SPIFI control register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x04 - SPIFI command register
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    ///0x08 - SPIFI address register
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    ///0x0c - SPIFI intermediate data register
    pub idata: crate::Reg<idata::IDATA_SPEC>,
    ///0x10 - SPIFI limit register
    pub climit: crate::Reg<climit::CLIMIT_SPEC>,
    ///0x14 - SPIFI data register
    pub data: crate::Reg<data::DATA_SPEC>,
    ///0x18 - SPIFI memory command register
    pub mcmd: crate::Reg<mcmd::MCMD_SPEC>,
    ///0x1c - SPIFI status register
    pub stat: crate::Reg<stat::STAT_SPEC>,
}
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SPIFI control register
pub mod ctrl;
///CMD register accessor: an alias for `Reg<CMD_SPEC>`
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///SPIFI command register
pub mod cmd;
///ADDR register accessor: an alias for `Reg<ADDR_SPEC>`
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
///SPIFI address register
pub mod addr;
///IDATA register accessor: an alias for `Reg<IDATA_SPEC>`
pub type IDATA = crate::Reg<idata::IDATA_SPEC>;
///SPIFI intermediate data register
pub mod idata;
///CLIMIT register accessor: an alias for `Reg<CLIMIT_SPEC>`
pub type CLIMIT = crate::Reg<climit::CLIMIT_SPEC>;
///SPIFI limit register
pub mod climit;
///DATA register accessor: an alias for `Reg<DATA_SPEC>`
pub type DATA = crate::Reg<data::DATA_SPEC>;
///SPIFI data register
pub mod data;
///MCMD register accessor: an alias for `Reg<MCMD_SPEC>`
pub type MCMD = crate::Reg<mcmd::MCMD_SPEC>;
///SPIFI memory command register
pub mod mcmd;
///STAT register accessor: an alias for `Reg<STAT_SPEC>`
pub type STAT = crate::Reg<stat::STAT_SPEC>;
///SPIFI status register
pub mod stat;
