///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    ///0x800 - Configuration for shared functions.
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    ///0x804 - Status register for Master, Slave, and Monitor functions.
    pub stat: crate::Reg<stat::STAT_SPEC>,
    ///0x808 - Interrupt Enable Set and read register.
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    ///0x80c - Interrupt Enable Clear register.
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    ///0x810 - Time-out value register.
    pub timeout: crate::Reg<timeout::TIMEOUT_SPEC>,
    ///0x814 - Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function.
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    ///0x818 - Interrupt Status register for Master, Slave, and Monitor functions.
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    _reserved7: [u8; 0x04],
    ///0x820 - Master control register.
    pub mstctl: crate::Reg<mstctl::MSTCTL_SPEC>,
    ///0x824 - Master timing configuration.
    pub msttime: crate::Reg<msttime::MSTTIME_SPEC>,
    ///0x828 - Combined Master receiver and transmitter data register.
    pub mstdat: crate::Reg<mstdat::MSTDAT_SPEC>,
    _reserved10: [u8; 0x14],
    ///0x840 - Slave control register.
    pub slvctl: crate::Reg<slvctl::SLVCTL_SPEC>,
    ///0x844 - Combined Slave receiver and transmitter data register.
    pub slvdat: crate::Reg<slvdat::SLVDAT_SPEC>,
    ///0x848..0x858 - Slave address register.
    pub slvadr: [crate::Reg<slvadr::SLVADR_SPEC>; 4],
    ///0x858 - Slave Qualification for address 0.
    pub slvqual0: crate::Reg<slvqual0::SLVQUAL0_SPEC>,
    _reserved14: [u8; 0x24],
    ///0x880 - Monitor receiver data register.
    pub monrxdat: crate::Reg<monrxdat::MONRXDAT_SPEC>,
    _reserved15: [u8; 0x0778],
    ///0xffc - Peripheral identification register.
    pub id: crate::Reg<id::ID_SPEC>,
}
///CFG register accessor: an alias for `Reg<CFG_SPEC>`
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
///Configuration for shared functions.
pub mod cfg;
///STAT register accessor: an alias for `Reg<STAT_SPEC>`
pub type STAT = crate::Reg<stat::STAT_SPEC>;
///Status register for Master, Slave, and Monitor functions.
pub mod stat;
///INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
///Interrupt Enable Set and read register.
pub mod intenset;
///INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
///Interrupt Enable Clear register.
pub mod intenclr;
///TIMEOUT register accessor: an alias for `Reg<TIMEOUT_SPEC>`
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
///Time-out value register.
pub mod timeout;
///CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
///Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function.
pub mod clkdiv;
///INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
///Interrupt Status register for Master, Slave, and Monitor functions.
pub mod intstat;
///MSTCTL register accessor: an alias for `Reg<MSTCTL_SPEC>`
pub type MSTCTL = crate::Reg<mstctl::MSTCTL_SPEC>;
///Master control register.
pub mod mstctl;
///MSTTIME register accessor: an alias for `Reg<MSTTIME_SPEC>`
pub type MSTTIME = crate::Reg<msttime::MSTTIME_SPEC>;
///Master timing configuration.
pub mod msttime;
///MSTDAT register accessor: an alias for `Reg<MSTDAT_SPEC>`
pub type MSTDAT = crate::Reg<mstdat::MSTDAT_SPEC>;
///Combined Master receiver and transmitter data register.
pub mod mstdat;
///SLVCTL register accessor: an alias for `Reg<SLVCTL_SPEC>`
pub type SLVCTL = crate::Reg<slvctl::SLVCTL_SPEC>;
///Slave control register.
pub mod slvctl;
///SLVDAT register accessor: an alias for `Reg<SLVDAT_SPEC>`
pub type SLVDAT = crate::Reg<slvdat::SLVDAT_SPEC>;
///Combined Slave receiver and transmitter data register.
pub mod slvdat;
///SLVADR register accessor: an alias for `Reg<SLVADR_SPEC>`
pub type SLVADR = crate::Reg<slvadr::SLVADR_SPEC>;
///Slave address register.
pub mod slvadr;
///SLVQUAL0 register accessor: an alias for `Reg<SLVQUAL0_SPEC>`
pub type SLVQUAL0 = crate::Reg<slvqual0::SLVQUAL0_SPEC>;
///Slave Qualification for address 0.
pub mod slvqual0;
///MONRXDAT register accessor: an alias for `Reg<MONRXDAT_SPEC>`
pub type MONRXDAT = crate::Reg<monrxdat::MONRXDAT_SPEC>;
///Monitor receiver data register.
pub mod monrxdat;
///ID register accessor: an alias for `Reg<ID_SPEC>`
pub type ID = crate::Reg<id::ID_SPEC>;
///Peripheral identification register.
pub mod id;
