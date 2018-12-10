#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers port 0; pins PIO0_0 to PIO0_28"]
    pub b: [B; 29],
    _reserved0: [u8; 4067usize],
    #[doc = "0x1000 - Word pin registers port 0"]
    pub w: [W; 29],
    _reserved1: [u8; 3980usize],
    #[doc = "0x2000 - Direction registers port 0"]
    pub dir0: DIR0,
    _reserved2: [u8; 124usize],
    #[doc = "0x2080 - Mask register port 0"]
    pub mask0: MASK0,
    _reserved3: [u8; 124usize],
    #[doc = "0x2100 - Port pin register port 0"]
    pub pin0: PIN0,
    _reserved4: [u8; 124usize],
    #[doc = "0x2180 - Masked port register port 0"]
    pub mpin0: MPIN0,
    _reserved5: [u8; 124usize],
    #[doc = "0x2200 - Write: Set register for port 0 Read: output bits for port 0"]
    pub set0: SET0,
    _reserved6: [u8; 124usize],
    #[doc = "0x2280 - Clear port 0"]
    pub clr0: CLR0,
    _reserved7: [u8; 124usize],
    #[doc = "0x2300 - Toggle port 0"]
    pub not0: NOT0,
    _reserved8: [u8; 124usize],
    #[doc = "0x2380 - Set pin direction bits for port 0."]
    pub dirset0: DIRSET0,
    _reserved9: [u8; 124usize],
    #[doc = "0x2400 - Clear pin direction bits for port 0."]
    pub dirclr0: DIRCLR0,
    _reserved10: [u8; 124usize],
    #[doc = "0x2480 - Toggle pin direction bits for port 0."]
    pub dirnot0: DIRNOT0,
}
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_28"]
pub struct B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_28"]
pub mod b;
#[doc = "Word pin registers port 0"]
pub struct W {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word pin registers port 0"]
pub mod w;
#[doc = "Direction registers port 0"]
pub struct DIR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction registers port 0"]
pub mod dir0;
#[doc = "Mask register port 0"]
pub struct MASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register port 0"]
pub mod mask0;
#[doc = "Port pin register port 0"]
pub struct PIN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port pin register port 0"]
pub mod pin0;
#[doc = "Masked port register port 0"]
pub struct MPIN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked port register port 0"]
pub mod mpin0;
#[doc = "Write: Set register for port 0 Read: output bits for port 0"]
pub struct SET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write: Set register for port 0 Read: output bits for port 0"]
pub mod set0;
#[doc = "Clear port 0"]
pub struct CLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear port 0"]
pub mod clr0;
#[doc = "Toggle port 0"]
pub struct NOT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle port 0"]
pub mod not0;
#[doc = "Set pin direction bits for port 0."]
pub struct DIRSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set pin direction bits for port 0."]
pub mod dirset0;
#[doc = "Clear pin direction bits for port 0."]
pub struct DIRCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear pin direction bits for port 0."]
pub mod dirclr0;
#[doc = "Toggle pin direction bits for port 0."]
pub struct DIRNOT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle pin direction bits for port 0."]
pub mod dirnot0;
