///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x04 - Status register
    pub status: crate::Reg<status::STATUS_SPEC>,
    ///0x08 - Interrupt Enable register
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    ///0x0c - Interrupt Clear register
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    ///0x10 - Memory Control register
    pub memctrl: crate::Reg<memctrl::MEMCTRL_SPEC>,
    ///0x14 - Memory Address register
    pub memaddr: crate::Reg<memaddr::MEMADDR_SPEC>,
    _reserved6: [u8; 0x08],
    ///0x20 - Input Data register
    pub indata: crate::Reg<indata::INDATA_SPEC>,
    ///0x24..0x40 - Alias register
    pub alias: [crate::Reg<alias::ALIAS_SPEC>; 7],
    ///0x40..0x60 - Digest register
    pub digest: [crate::Reg<digest::DIGEST_SPEC>; 8],
}
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///Control register
pub mod ctrl;
///STATUS register accessor: an alias for `Reg<STATUS_SPEC>`
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///Status register
pub mod status;
///INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
///Interrupt Enable register
pub mod intenset;
///INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
///Interrupt Clear register
pub mod intenclr;
///MEMCTRL register accessor: an alias for `Reg<MEMCTRL_SPEC>`
pub type MEMCTRL = crate::Reg<memctrl::MEMCTRL_SPEC>;
///Memory Control register
pub mod memctrl;
///MEMADDR register accessor: an alias for `Reg<MEMADDR_SPEC>`
pub type MEMADDR = crate::Reg<memaddr::MEMADDR_SPEC>;
///Memory Address register
pub mod memaddr;
///INDATA register accessor: an alias for `Reg<INDATA_SPEC>`
pub type INDATA = crate::Reg<indata::INDATA_SPEC>;
///Input Data register
pub mod indata;
///ALIAS register accessor: an alias for `Reg<ALIAS_SPEC>`
pub type ALIAS = crate::Reg<alias::ALIAS_SPEC>;
///Alias register
pub mod alias;
///DIGEST register accessor: an alias for `Reg<DIGEST_SPEC>`
pub type DIGEST = crate::Reg<digest::DIGEST_SPEC>;
///Digest register
pub mod digest;
