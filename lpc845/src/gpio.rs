#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub b: [B; 2],
    _reserved0: [u8; 4032usize],
    #[doc = "0x1000 - no description available"]
    pub w: [W; 2],
    _reserved1: [u8; 3840usize],
    #[doc = "0x2000 - Direction registers"]
    pub dir: [DIR; 2],
    _reserved2: [u8; 120usize],
    #[doc = "0x2080 - Mask register"]
    pub mask: [MASK; 2],
    _reserved3: [u8; 120usize],
    #[doc = "0x2100 - Port pin register"]
    pub pin: [PIN; 2],
    _reserved4: [u8; 120usize],
    #[doc = "0x2180 - Masked port register"]
    pub mpin0: MPIN0,
    _reserved5: [u8; 124usize],
    #[doc = "0x2200 - Write: Set register for port Read: output bits for port"]
    pub set: [SET; 2],
    _reserved6: [u8; 120usize],
    #[doc = "0x2280 - Clear port"]
    pub clr: [CLR; 2],
    _reserved7: [u8; 120usize],
    #[doc = "0x2300 - Toggle port"]
    pub not: [NOT; 2],
    _reserved8: [u8; 120usize],
    #[doc = "0x2380 - Set pin direction bits for port"]
    pub dirset: [DIRSET; 2],
    _reserved9: [u8; 120usize],
    #[doc = "0x2400 - Clear pin direction bits for port"]
    pub dirclr: [DIRCLR; 2],
    _reserved10: [u8; 120usize],
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    pub dirnot: [DIRNOT; 2],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct B {
    #[doc = "0x00 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b_: [self::b::B_; 32],
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod b;
#[doc = r" Register block"]
#[repr(C)]
pub struct W {
    #[doc = "0x00 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w_: [self::w::W_; 32],
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod w;
#[doc = "Direction registers"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction registers"]
pub mod dir;
#[doc = "Mask register"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register"]
pub mod mask;
#[doc = "Port pin register"]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port pin register"]
pub mod pin;
#[doc = "Masked port register"]
pub struct MPIN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked port register"]
pub mod mpin0;
#[doc = "Write: Set register for port Read: output bits for port"]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set;
#[doc = "Clear port"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear port"]
pub mod clr;
#[doc = "Toggle port"]
pub struct NOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle port"]
pub mod not;
#[doc = "Set pin direction bits for port"]
pub struct DIRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "Clear pin direction bits for port"]
pub struct DIRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "Toggle pin direction bits for port"]
pub struct DIRNOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
