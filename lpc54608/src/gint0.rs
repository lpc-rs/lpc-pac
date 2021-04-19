#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO grouped interrupt control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - GPIO grouped interrupt port 0 polarity register"]
    pub port_pol: [crate::Reg<port_pol::PORT_POL_SPEC>; 2],
    _reserved2: [u8; 24usize],
    #[doc = "0x40 - GPIO grouped interrupt port 0 enable register"]
    pub port_ena: [crate::Reg<port_ena::PORT_ENA_SPEC>; 2],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "GPIO grouped interrupt control register"]
pub mod ctrl;
#[doc = "PORT_POL register accessor: an alias for `Reg<PORT_POL_SPEC>`"]
pub type PORT_POL = crate::Reg<port_pol::PORT_POL_SPEC>;
#[doc = "GPIO grouped interrupt port 0 polarity register"]
pub mod port_pol;
#[doc = "PORT_ENA register accessor: an alias for `Reg<PORT_ENA_SPEC>`"]
pub type PORT_ENA = crate::Reg<port_ena::PORT_ENA_SPEC>;
#[doc = "GPIO grouped interrupt port 0 enable register"]
pub mod port_ena;
