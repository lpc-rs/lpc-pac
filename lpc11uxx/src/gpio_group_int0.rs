#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO grouped interrupt control register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - GPIO grouped interrupt port 0 polarity register"]
    pub port_pol: [PORT_POL; 2],
    _reserved2: [u8; 24usize],
    #[doc = "0x40 - GPIO grouped interrupt port 0/1 enable register"]
    pub port_ena: [PORT_ENA; 2],
}
#[doc = "GPIO grouped interrupt control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO grouped interrupt control register"]
pub mod ctrl;
#[doc = "GPIO grouped interrupt port 0 polarity register"]
pub struct PORT_POL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO grouped interrupt port 0 polarity register"]
pub mod port_pol;
#[doc = "GPIO grouped interrupt port 0/1 enable register"]
pub struct PORT_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO grouped interrupt port 0/1 enable register"]
pub mod port_ena;
