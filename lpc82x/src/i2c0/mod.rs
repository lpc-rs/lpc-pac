#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration for shared functions."]
    pub cfg: CFG,
    #[doc = "0x04 - Status register for Master, Slave, and Monitor functions."]
    pub stat: STAT,
    #[doc = "0x08 - Interrupt Enable Set and read register."]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Enable Clear register."]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Time-out value register."]
    pub timeout: TIMEOUT,
    #[doc = "0x14 - Clock pre-divider for the entire I2C block. This determines what time increments are used for the MSTTIME and SLVTIME registers."]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - Interrupt Status register for Master, Slave, and Monitor functions."]
    pub intstat: INTSTAT,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Master control register."]
    pub mstctl: MSTCTL,
    #[doc = "0x24 - Master timing configuration."]
    pub msttime: MSTTIME,
    #[doc = "0x28 - Combined Master receiver and transmitter data register."]
    pub mstdat: MSTDAT,
    _reserved1: [u8; 20usize],
    #[doc = "0x40 - Slave control register."]
    pub slvctl: SLVCTL,
    #[doc = "0x44 - Combined Slave receiver and transmitter data register."]
    pub slvdat: SLVDAT,
    #[doc = "0x48 - Slave address 0."]
    pub slvadr: [SLVADR; 4],
    #[doc = "0x58 - Slave Qualification for address 0."]
    pub slvqual0: SLVQUAL0,
    _reserved2: [u8; 36usize],
    #[doc = "0x80 - Monitor receiver data register."]
    pub monrxdat: MONRXDAT,
}
#[doc = "Configuration for shared functions."]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration for shared functions."]
pub mod cfg;
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub mod stat;
#[doc = "Interrupt Enable Set and read register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set and read register."]
pub mod intenset;
#[doc = "Interrupt Enable Clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear register."]
pub mod intenclr;
#[doc = "Time-out value register."]
pub struct TIMEOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time-out value register."]
pub mod timeout;
#[doc = "Clock pre-divider for the entire I2C block. This determines what time increments are used for the MSTTIME and SLVTIME registers."]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock pre-divider for the entire I2C block. This determines what time increments are used for the MSTTIME and SLVTIME registers."]
pub mod clkdiv;
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub mod intstat;
#[doc = "Master control register."]
pub struct MSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master control register."]
pub mod mstctl;
#[doc = "Master timing configuration."]
pub struct MSTTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master timing configuration."]
pub mod msttime;
#[doc = "Combined Master receiver and transmitter data register."]
pub struct MSTDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combined Master receiver and transmitter data register."]
pub mod mstdat;
#[doc = "Slave control register."]
pub struct SLVCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave control register."]
pub mod slvctl;
#[doc = "Combined Slave receiver and transmitter data register."]
pub struct SLVDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combined Slave receiver and transmitter data register."]
pub mod slvdat;
#[doc = "Slave address 0."]
pub struct SLVADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave address 0."]
pub mod slvadr;
#[doc = "Slave Qualification for address 0."]
pub struct SLVQUAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Qualification for address 0."]
pub mod slvqual0;
#[doc = "Monitor receiver data register."]
pub struct MONRXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Monitor receiver data register."]
pub mod monrxdat;
