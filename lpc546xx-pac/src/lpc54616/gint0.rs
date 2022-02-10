///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO grouped interrupt control register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x1c],
    ///0x20..0x28 - GPIO grouped interrupt port 0 polarity register
    pub port_pol: [crate::Reg<port_pol::PORT_POL_SPEC>; 2],
    _reserved2: [u8; 0x18],
    ///0x40..0x48 - GPIO grouped interrupt port 0 enable register
    pub port_ena: [crate::Reg<port_ena::PORT_ENA_SPEC>; 2],
}
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///GPIO grouped interrupt control register
pub mod ctrl;
///PORT_POL register accessor: an alias for `Reg<PORT_POL_SPEC>`
pub type PORT_POL = crate::Reg<port_pol::PORT_POL_SPEC>;
///GPIO grouped interrupt port 0 polarity register
pub mod port_pol;
///PORT_ENA register accessor: an alias for `Reg<PORT_ENA_SPEC>`
pub type PORT_ENA = crate::Reg<port_ena::PORT_ENA_SPEC>;
///GPIO grouped interrupt port 0 enable register
pub mod port_ena;
