#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers"]
    pub b: [B; 88],
    _reserved1: [u8; 4008usize],
    #[doc = "0x1000 - Word pin registers"]
    pub w: [W; 88],
    _reserved2: [u8; 3744usize],
    #[doc = "0x2000 - Port Direction registers"]
    pub dir: [DIR; 3],
    _reserved3: [u8; 116usize],
    #[doc = "0x2080 - Port Mask register"]
    pub mask: [MASK; 3],
    _reserved4: [u8; 116usize],
    #[doc = "0x2100 - Port pin register"]
    pub pin: [PIN; 3],
    _reserved5: [u8; 116usize],
    #[doc = "0x2180 - Masked port register"]
    pub mpin: [MPIN; 3],
    _reserved6: [u8; 116usize],
    #[doc = "0x2200 - Write: Set port register Read: port output bits"]
    pub set: [SET; 3],
    _reserved7: [u8; 116usize],
    #[doc = "0x2280 - Clear port"]
    pub clr: [CLR; 3],
    _reserved8: [u8; 116usize],
    #[doc = "0x2300 - Toggle port"]
    pub not: [NOT; 3],
}
#[doc = "Byte pin registers"]
pub struct B {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Byte pin registers"]
pub mod b;
#[doc = "Word pin registers"]
pub struct W {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Word pin registers"]
pub mod w;
#[doc = "Port Direction registers"]
pub struct DIR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Port Direction registers"]
pub mod dir;
#[doc = "Port Mask register"]
pub struct MASK {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Port Mask register"]
pub mod mask;
#[doc = "Port pin register"]
pub struct PIN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Port pin register"]
pub mod pin;
#[doc = "Masked port register"]
pub struct MPIN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Masked port register"]
pub mod mpin;
#[doc = "Write: Set port register Read: port output bits"]
pub struct SET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write: Set port register Read: port output bits"]
pub mod set;
#[doc = "Clear port"]
pub struct CLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear port"]
pub mod clr;
#[doc = "Toggle port"]
pub struct NOT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Toggle port"]
pub mod not;
