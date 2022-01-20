#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - RTC match register"]
    pub match_: crate::Reg<match_::MATCH_SPEC>,
    #[doc = "0x08 - RTC counter register"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    #[doc = "0x0c - High-resolution/wake-up timer control register"]
    pub wake: crate::Reg<wake::WAKE_SPEC>,
    _reserved4: [u8; 0x30],
    #[doc = "0x40..0x60 - General Purpose register"]
    pub gpreg: [crate::Reg<gpreg::GPREG_SPEC>; 8],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC control register"]
pub mod ctrl;
#[doc = "MATCH register accessor: an alias for `Reg<MATCH_SPEC>`"]
pub type MATCH = crate::Reg<match_::MATCH_SPEC>;
#[doc = "RTC match register"]
pub mod match_;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "RTC counter register"]
pub mod count;
#[doc = "WAKE register accessor: an alias for `Reg<WAKE_SPEC>`"]
pub type WAKE = crate::Reg<wake::WAKE_SPEC>;
#[doc = "High-resolution/wake-up timer control register"]
pub mod wake;
#[doc = "GPREG register accessor: an alias for `Reg<GPREG_SPEC>`"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General Purpose register"]
pub mod gpreg;
