///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0xc0 - no description available
    pub b: [B; 6],
    _reserved1: [u8; 0x0f40],
    ///0x1000..0x1300 - no description available
    pub w: [W; 6],
    _reserved2: [u8; 0x0d00],
    ///0x2000..0x2018 - Direction registers
    pub dir: [crate::Reg<dir::DIR_SPEC>; 6],
    _reserved3: [u8; 0x68],
    ///0x2080..0x2098 - Mask register
    pub mask: [crate::Reg<mask::MASK_SPEC>; 6],
    _reserved4: [u8; 0x68],
    ///0x2100..0x2118 - Port pin register
    pub pin: [crate::Reg<pin::PIN_SPEC>; 6],
    _reserved5: [u8; 0x68],
    ///0x2180..0x2198 - Masked port register
    pub mpin: [crate::Reg<mpin::MPIN_SPEC>; 6],
    _reserved6: [u8; 0x68],
    ///0x2200..0x2218 - Write: Set register for port Read: output bits for port
    pub set: [crate::Reg<set::SET_SPEC>; 6],
    _reserved7: [u8; 0x68],
    ///0x2280..0x2298 - Clear port
    pub clr: [crate::Reg<clr::CLR_SPEC>; 6],
    _reserved8: [u8; 0x68],
    ///0x2300..0x2318 - Toggle port
    pub not: [crate::Reg<not::NOT_SPEC>; 6],
    _reserved9: [u8; 0x68],
    ///0x2380..0x2398 - Set pin direction bits for port
    pub dirset: [crate::Reg<dirset::DIRSET_SPEC>; 6],
    _reserved10: [u8; 0x68],
    ///0x2400..0x2418 - Clear pin direction bits for port
    pub dirclr: [crate::Reg<dirclr::DIRCLR_SPEC>; 6],
    _reserved11: [u8; 0x68],
    ///0x2480..0x2498 - Toggle pin direction bits for port
    pub dirnot: [crate::Reg<dirnot::DIRNOT_SPEC>; 6],
}
///Register block
#[repr(C)]
pub struct B {
    ///0x00..0x20 - Byte pin registers for all port 0 and 1 GPIO pins
    pub b_: [crate::Reg<self::b::b_::B__SPEC>; 32],
}
///Register block
///no description available
pub mod b;
///Register block
#[repr(C)]
pub struct W {
    ///0x00..0x80 - Word pin registers for all port 0 and 1 GPIO pins
    pub w_: [crate::Reg<self::w::w_::W__SPEC>; 32],
}
///Register block
///no description available
pub mod w;
///DIR register accessor: an alias for `Reg<DIR_SPEC>`
pub type DIR = crate::Reg<dir::DIR_SPEC>;
///Direction registers
pub mod dir;
///MASK register accessor: an alias for `Reg<MASK_SPEC>`
pub type MASK = crate::Reg<mask::MASK_SPEC>;
///Mask register
pub mod mask;
///PIN register accessor: an alias for `Reg<PIN_SPEC>`
pub type PIN = crate::Reg<pin::PIN_SPEC>;
///Port pin register
pub mod pin;
///MPIN register accessor: an alias for `Reg<MPIN_SPEC>`
pub type MPIN = crate::Reg<mpin::MPIN_SPEC>;
///Masked port register
pub mod mpin;
///SET register accessor: an alias for `Reg<SET_SPEC>`
pub type SET = crate::Reg<set::SET_SPEC>;
///Write: Set register for port Read: output bits for port
pub mod set;
///CLR register accessor: an alias for `Reg<CLR_SPEC>`
pub type CLR = crate::Reg<clr::CLR_SPEC>;
///Clear port
pub mod clr;
///NOT register accessor: an alias for `Reg<NOT_SPEC>`
pub type NOT = crate::Reg<not::NOT_SPEC>;
///Toggle port
pub mod not;
///DIRSET register accessor: an alias for `Reg<DIRSET_SPEC>`
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
///Set pin direction bits for port
pub mod dirset;
///DIRCLR register accessor: an alias for `Reg<DIRCLR_SPEC>`
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
///Clear pin direction bits for port
pub mod dirclr;
///DIRNOT register accessor: an alias for `Reg<DIRNOT_SPEC>`
pub type DIRNOT = crate::Reg<dirnot::DIRNOT_SPEC>;
///Toggle pin direction bits for port
pub mod dirnot;
