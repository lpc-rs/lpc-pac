#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status register."]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Capture configuration register."]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x0c - Capture clear register."]
    pub capclr: crate::Reg<capclr::CAPCLR_SPEC>,
    #[doc = "0x10 - Capture register ."]
    pub cap: [crate::Reg<cap::CAP_SPEC>; 4],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register."]
pub mod ctrl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register."]
pub mod stat;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Capture configuration register."]
pub mod cfg;
#[doc = "CAPCLR register accessor: an alias for `Reg<CAPCLR_SPEC>`"]
pub type CAPCLR = crate::Reg<capclr::CAPCLR_SPEC>;
#[doc = "Capture clear register."]
pub mod capclr;
#[doc = "CAP register accessor: an alias for `Reg<CAP_SPEC>`"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Capture register ."]
pub mod cap;
