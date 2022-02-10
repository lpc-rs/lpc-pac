///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register.
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x04 - Status register.
    pub stat: crate::Reg<stat::STAT_SPEC>,
    ///0x08 - Capture configuration register.
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    ///0x0c - Capture clear register.
    pub capclr: crate::Reg<capclr::CAPCLR_SPEC>,
    ///0x10..0x20 - Capture register .
    pub cap: [crate::Reg<cap::CAP_SPEC>; 4],
}
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///Control register.
pub mod ctrl;
///STAT register accessor: an alias for `Reg<STAT_SPEC>`
pub type STAT = crate::Reg<stat::STAT_SPEC>;
///Status register.
pub mod stat;
///CFG register accessor: an alias for `Reg<CFG_SPEC>`
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
///Capture configuration register.
pub mod cfg;
///CAPCLR register accessor: an alias for `Reg<CAPCLR_SPEC>`
pub type CAPCLR = crate::Reg<capclr::CAPCLR_SPEC>;
///Capture clear register.
pub mod capclr;
///CAP register accessor: an alias for `Reg<CAP_SPEC>`
pub type CAP = crate::Reg<cap::CAP_SPEC>;
///Capture register .
pub mod cap;
