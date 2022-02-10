///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RTC control register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x04 - RTC match register
    pub match_: crate::Reg<match_::MATCH_SPEC>,
    ///0x08 - RTC counter register
    pub count: crate::Reg<count::COUNT_SPEC>,
    ///0x0c - High-resolution/wake-up timer control register
    pub wake: crate::Reg<wake::WAKE_SPEC>,
    _reserved4: [u8; 0x30],
    ///0x40..0x60 - General Purpose register
    pub gpreg: [crate::Reg<gpreg::GPREG_SPEC>; 8],
}
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///RTC control register
pub mod ctrl;
///MATCH register accessor: an alias for `Reg<MATCH_SPEC>`
pub type MATCH = crate::Reg<match_::MATCH_SPEC>;
///RTC match register
pub mod match_;
///COUNT register accessor: an alias for `Reg<COUNT_SPEC>`
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
///RTC counter register
pub mod count;
///WAKE register accessor: an alias for `Reg<WAKE_SPEC>`
pub type WAKE = crate::Reg<wake::WAKE_SPEC>;
///High-resolution/wake-up timer control register
pub mod wake;
///GPREG register accessor: an alias for `Reg<GPREG_SPEC>`
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
///General Purpose register
pub mod gpreg;
