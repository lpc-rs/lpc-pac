#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Async peripheral reset control"]
    pub asyncpresetctrl: crate::Reg<asyncpresetctrl::ASYNCPRESETCTRL_SPEC>,
    #[doc = "0x04 - Set bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlset: crate::Reg<asyncpresetctrlset::ASYNCPRESETCTRLSET_SPEC>,
    #[doc = "0x08 - Clear bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlclr: crate::Reg<asyncpresetctrlclr::ASYNCPRESETCTRLCLR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Async peripheral clock control"]
    pub asyncapbclkctrl: crate::Reg<asyncapbclkctrl::ASYNCAPBCLKCTRL_SPEC>,
    #[doc = "0x14 - Set bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlset: crate::Reg<asyncapbclkctrlset::ASYNCAPBCLKCTRLSET_SPEC>,
    #[doc = "0x18 - Clear bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlclr: crate::Reg<asyncapbclkctrlclr::ASYNCAPBCLKCTRLCLR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Async APB clock source select A"]
    pub asyncapbclksela: crate::Reg<asyncapbclksela::ASYNCAPBCLKSELA_SPEC>,
}
#[doc = "ASYNCPRESETCTRL register accessor: an alias for `Reg<ASYNCPRESETCTRL_SPEC>`"]
pub type ASYNCPRESETCTRL = crate::Reg<asyncpresetctrl::ASYNCPRESETCTRL_SPEC>;
#[doc = "Async peripheral reset control"]
pub mod asyncpresetctrl;
#[doc = "ASYNCPRESETCTRLSET register accessor: an alias for `Reg<ASYNCPRESETCTRLSET_SPEC>`"]
pub type ASYNCPRESETCTRLSET = crate::Reg<asyncpresetctrlset::ASYNCPRESETCTRLSET_SPEC>;
#[doc = "Set bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlset;
#[doc = "ASYNCPRESETCTRLCLR register accessor: an alias for `Reg<ASYNCPRESETCTRLCLR_SPEC>`"]
pub type ASYNCPRESETCTRLCLR = crate::Reg<asyncpresetctrlclr::ASYNCPRESETCTRLCLR_SPEC>;
#[doc = "Clear bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlclr;
#[doc = "ASYNCAPBCLKCTRL register accessor: an alias for `Reg<ASYNCAPBCLKCTRL_SPEC>`"]
pub type ASYNCAPBCLKCTRL = crate::Reg<asyncapbclkctrl::ASYNCAPBCLKCTRL_SPEC>;
#[doc = "Async peripheral clock control"]
pub mod asyncapbclkctrl;
#[doc = "ASYNCAPBCLKCTRLSET register accessor: an alias for `Reg<ASYNCAPBCLKCTRLSET_SPEC>`"]
pub type ASYNCAPBCLKCTRLSET = crate::Reg<asyncapbclkctrlset::ASYNCAPBCLKCTRLSET_SPEC>;
#[doc = "Set bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlset;
#[doc = "ASYNCAPBCLKCTRLCLR register accessor: an alias for `Reg<ASYNCAPBCLKCTRLCLR_SPEC>`"]
pub type ASYNCAPBCLKCTRLCLR = crate::Reg<asyncapbclkctrlclr::ASYNCAPBCLKCTRLCLR_SPEC>;
#[doc = "Clear bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlclr;
#[doc = "ASYNCAPBCLKSELA register accessor: an alias for `Reg<ASYNCAPBCLKSELA_SPEC>`"]
pub type ASYNCAPBCLKSELA = crate::Reg<asyncapbclksela::ASYNCAPBCLKSELA_SPEC>;
#[doc = "Async APB clock source select A"]
pub mod asyncapbclksela;
