///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Async peripheral reset control
    pub asyncpresetctrl: crate::Reg<asyncpresetctrl::ASYNCPRESETCTRL_SPEC>,
    ///0x04 - Set bits in ASYNCPRESETCTRL
    pub asyncpresetctrlset: crate::Reg<asyncpresetctrlset::ASYNCPRESETCTRLSET_SPEC>,
    ///0x08 - Clear bits in ASYNCPRESETCTRL
    pub asyncpresetctrlclr: crate::Reg<asyncpresetctrlclr::ASYNCPRESETCTRLCLR_SPEC>,
    _reserved3: [u8; 0x04],
    ///0x10 - Async peripheral clock control
    pub asyncapbclkctrl: crate::Reg<asyncapbclkctrl::ASYNCAPBCLKCTRL_SPEC>,
    ///0x14 - Set bits in ASYNCAPBCLKCTRL
    pub asyncapbclkctrlset: crate::Reg<asyncapbclkctrlset::ASYNCAPBCLKCTRLSET_SPEC>,
    ///0x18 - Clear bits in ASYNCAPBCLKCTRL
    pub asyncapbclkctrlclr: crate::Reg<asyncapbclkctrlclr::ASYNCAPBCLKCTRLCLR_SPEC>,
    _reserved6: [u8; 0x04],
    ///0x20 - Async APB clock source select A
    pub asyncapbclksela: crate::Reg<asyncapbclksela::ASYNCAPBCLKSELA_SPEC>,
}
///ASYNCPRESETCTRL register accessor: an alias for `Reg<ASYNCPRESETCTRL_SPEC>`
pub type ASYNCPRESETCTRL = crate::Reg<asyncpresetctrl::ASYNCPRESETCTRL_SPEC>;
///Async peripheral reset control
pub mod asyncpresetctrl;
///ASYNCPRESETCTRLSET register accessor: an alias for `Reg<ASYNCPRESETCTRLSET_SPEC>`
pub type ASYNCPRESETCTRLSET = crate::Reg<asyncpresetctrlset::ASYNCPRESETCTRLSET_SPEC>;
///Set bits in ASYNCPRESETCTRL
pub mod asyncpresetctrlset;
///ASYNCPRESETCTRLCLR register accessor: an alias for `Reg<ASYNCPRESETCTRLCLR_SPEC>`
pub type ASYNCPRESETCTRLCLR = crate::Reg<asyncpresetctrlclr::ASYNCPRESETCTRLCLR_SPEC>;
///Clear bits in ASYNCPRESETCTRL
pub mod asyncpresetctrlclr;
///ASYNCAPBCLKCTRL register accessor: an alias for `Reg<ASYNCAPBCLKCTRL_SPEC>`
pub type ASYNCAPBCLKCTRL = crate::Reg<asyncapbclkctrl::ASYNCAPBCLKCTRL_SPEC>;
///Async peripheral clock control
pub mod asyncapbclkctrl;
///ASYNCAPBCLKCTRLSET register accessor: an alias for `Reg<ASYNCAPBCLKCTRLSET_SPEC>`
pub type ASYNCAPBCLKCTRLSET = crate::Reg<asyncapbclkctrlset::ASYNCAPBCLKCTRLSET_SPEC>;
///Set bits in ASYNCAPBCLKCTRL
pub mod asyncapbclkctrlset;
///ASYNCAPBCLKCTRLCLR register accessor: an alias for `Reg<ASYNCAPBCLKCTRLCLR_SPEC>`
pub type ASYNCAPBCLKCTRLCLR = crate::Reg<asyncapbclkctrlclr::ASYNCAPBCLKCTRLCLR_SPEC>;
///Clear bits in ASYNCAPBCLKCTRL
pub mod asyncapbclkctrlclr;
///ASYNCAPBCLKSELA register accessor: an alias for `Reg<ASYNCAPBCLKSELA_SPEC>`
pub type ASYNCAPBCLKSELA = crate::Reg<asyncapbclksela::ASYNCAPBCLKSELA_SPEC>;
///Async APB clock source select A
pub mod asyncapbclksela;
