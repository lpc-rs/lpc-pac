#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - no description available"]
    pub b: [B; 2],
    _reserved1: [u8; 0x0fc0],
    #[doc = "0x1000..0x1100 - no description available"]
    pub w: [W; 2],
    _reserved2: [u8; 0x0f00],
    #[doc = "0x2000..0x2008 - Direction registers"]
    pub dir: [crate::Reg<dir::DIR_SPEC>; 2],
    _reserved3: [u8; 0x78],
    #[doc = "0x2080..0x2088 - Mask register"]
    pub mask: [crate::Reg<mask::MASK_SPEC>; 2],
    _reserved4: [u8; 0x78],
    #[doc = "0x2100..0x2108 - Port pin register"]
    pub pin: [crate::Reg<pin::PIN_SPEC>; 2],
    _reserved5: [u8; 0x78],
    #[doc = "0x2180..0x2188 - Masked port register"]
    pub mpin: [crate::Reg<mpin::MPIN_SPEC>; 2],
    _reserved6: [u8; 0x78],
    #[doc = "0x2200..0x2208 - Write: Set register for port Read: output bits for port"]
    pub set: [crate::Reg<set::SET_SPEC>; 2],
    _reserved7: [u8; 0x78],
    #[doc = "0x2280..0x2288 - Clear port"]
    pub clr: [crate::Reg<clr::CLR_SPEC>; 2],
    _reserved8: [u8; 0x78],
    #[doc = "0x2300..0x2308 - Toggle port"]
    pub not: [crate::Reg<not::NOT_SPEC>; 2],
    _reserved9: [u8; 0x78],
    #[doc = "0x2380..0x2388 - Set pin direction bits for port"]
    pub dirset: [crate::Reg<dirset::DIRSET_SPEC>; 2],
    _reserved10: [u8; 0x78],
    #[doc = "0x2400..0x2408 - Clear pin direction bits for port"]
    pub dirclr: [crate::Reg<dirclr::DIRCLR_SPEC>; 2],
    _reserved11: [u8; 0x78],
    #[doc = "0x2480..0x2488 - Toggle pin direction bits for port"]
    pub dirnot: [crate::Reg<dirnot::DIRNOT_SPEC>; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct B {
    #[doc = "0x00..0x20 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b_: [crate::Reg<self::b::b_::B__SPEC>; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod b;
#[doc = r"Register block"]
#[repr(C)]
pub struct W {
    #[doc = "0x00..0x80 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w_: [crate::Reg<self::w::w_::W__SPEC>; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod w;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction registers"]
pub mod dir;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register"]
pub mod mask;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Port pin register"]
pub mod pin;
#[doc = "MPIN register accessor: an alias for `Reg<MPIN_SPEC>`"]
pub type MPIN = crate::Reg<mpin::MPIN_SPEC>;
#[doc = "Masked port register"]
pub mod mpin;
#[doc = "SET register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set;
#[doc = "CLR register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Clear port"]
pub mod clr;
#[doc = "NOT register accessor: an alias for `Reg<NOT_SPEC>`"]
pub type NOT = crate::Reg<not::NOT_SPEC>;
#[doc = "Toggle port"]
pub mod not;
#[doc = "DIRSET register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "DIRCLR register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "DIRNOT register accessor: an alias for `Reg<DIRNOT_SPEC>`"]
pub type DIRNOT = crate::Reg<dirnot::DIRNOT_SPEC>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
