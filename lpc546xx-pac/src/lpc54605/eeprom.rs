///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - EEPROM command register
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    _reserved1: [u8; 0x04],
    ///0x08 - EEPROM read wait state register
    pub rwstate: crate::Reg<rwstate::RWSTATE_SPEC>,
    ///0x0c - EEPROM auto programming register
    pub autoprog: crate::Reg<autoprog::AUTOPROG_SPEC>,
    ///0x10 - EEPROM wait state register
    pub wstate: crate::Reg<wstate::WSTATE_SPEC>,
    ///0x14 - EEPROM clock divider register
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    ///0x18 - EEPROM power-down register
    pub pwrdwn: crate::Reg<pwrdwn::PWRDWN_SPEC>,
    _reserved6: [u8; 0x0fbc],
    ///0xfd8 - EEPROM interrupt enable clear
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    ///0xfdc - EEPROM interrupt enable set
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    ///0xfe0 - EEPROM interrupt status
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    ///0xfe4 - EEPROM interrupt enable
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    ///0xfe8 - EEPROM interrupt status clear
    pub intstatclr: crate::Reg<intstatclr::INTSTATCLR_SPEC>,
    ///0xfec - EEPROM interrupt status set
    pub intstatset: crate::Reg<intstatset::INTSTATSET_SPEC>,
}
///CMD register accessor: an alias for `Reg<CMD_SPEC>`
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///EEPROM command register
pub mod cmd;
///RWSTATE register accessor: an alias for `Reg<RWSTATE_SPEC>`
pub type RWSTATE = crate::Reg<rwstate::RWSTATE_SPEC>;
///EEPROM read wait state register
pub mod rwstate;
///AUTOPROG register accessor: an alias for `Reg<AUTOPROG_SPEC>`
pub type AUTOPROG = crate::Reg<autoprog::AUTOPROG_SPEC>;
///EEPROM auto programming register
pub mod autoprog;
///WSTATE register accessor: an alias for `Reg<WSTATE_SPEC>`
pub type WSTATE = crate::Reg<wstate::WSTATE_SPEC>;
///EEPROM wait state register
pub mod wstate;
///CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
///EEPROM clock divider register
pub mod clkdiv;
///PWRDWN register accessor: an alias for `Reg<PWRDWN_SPEC>`
pub type PWRDWN = crate::Reg<pwrdwn::PWRDWN_SPEC>;
///EEPROM power-down register
pub mod pwrdwn;
///INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
///EEPROM interrupt enable clear
pub mod intenclr;
///INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
///EEPROM interrupt enable set
pub mod intenset;
///INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
///EEPROM interrupt status
pub mod intstat;
///INTEN register accessor: an alias for `Reg<INTEN_SPEC>`
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
///EEPROM interrupt enable
pub mod inten;
///INTSTATCLR register accessor: an alias for `Reg<INTSTATCLR_SPEC>`
pub type INTSTATCLR = crate::Reg<intstatclr::INTSTATCLR_SPEC>;
///EEPROM interrupt status clear
pub mod intstatclr;
///INTSTATSET register accessor: an alias for `Reg<INTSTATSET_SPEC>`
pub type INTSTATSET = crate::Reg<intstatset::INTSTATSET_SPEC>;
///EEPROM interrupt status set
pub mod intstatset;
