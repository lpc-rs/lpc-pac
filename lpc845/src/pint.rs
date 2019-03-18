#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode register"]
    pub isel: ISEL,
    #[doc = "0x04 - Pin interrupt level or rising edge interrupt enable register"]
    pub ienr: IENR,
    #[doc = "0x08 - Pin interrupt level or rising edge interrupt set register"]
    pub sienr: SIENR,
    #[doc = "0x0c - Pin interrupt level (rising edge interrupt) clear register"]
    pub cienr: CIENR,
    #[doc = "0x10 - Pin interrupt active level or falling edge interrupt enable register"]
    pub ienf: IENF,
    #[doc = "0x14 - Pin interrupt active level or falling edge interrupt set register"]
    pub sienf: SIENF,
    #[doc = "0x18 - Pin interrupt active level or falling edge interrupt clear register"]
    pub cienf: CIENF,
    #[doc = "0x1c - Pin interrupt rising edge register"]
    pub rise: RISE,
    #[doc = "0x20 - Pin interrupt falling edge register"]
    pub fall: FALL,
    #[doc = "0x24 - Pin interrupt status register"]
    pub ist: IST,
    #[doc = "0x28 - Pattern match interrupt control register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x2c - Pattern match interrupt bit-slice source register"]
    pub pmsrc: PMSRC,
    #[doc = "0x30 - Pattern match interrupt bit slice configuration register"]
    pub pmcfg: PMCFG,
}
#[doc = "Pin Interrupt Mode register"]
pub struct ISEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Interrupt Mode register"]
pub mod isel;
#[doc = "Pin interrupt level or rising edge interrupt enable register"]
pub struct IENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt level or rising edge interrupt enable register"]
pub mod ienr;
#[doc = "Pin interrupt level or rising edge interrupt set register"]
pub struct SIENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt level or rising edge interrupt set register"]
pub mod sienr;
#[doc = "Pin interrupt level (rising edge interrupt) clear register"]
pub struct CIENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt level (rising edge interrupt) clear register"]
pub mod cienr;
#[doc = "Pin interrupt active level or falling edge interrupt enable register"]
pub struct IENF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt active level or falling edge interrupt enable register"]
pub mod ienf;
#[doc = "Pin interrupt active level or falling edge interrupt set register"]
pub struct SIENF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt active level or falling edge interrupt set register"]
pub mod sienf;
#[doc = "Pin interrupt active level or falling edge interrupt clear register"]
pub struct CIENF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt active level or falling edge interrupt clear register"]
pub mod cienf;
#[doc = "Pin interrupt rising edge register"]
pub struct RISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt rising edge register"]
pub mod rise;
#[doc = "Pin interrupt falling edge register"]
pub struct FALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt falling edge register"]
pub mod fall;
#[doc = "Pin interrupt status register"]
pub struct IST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt status register"]
pub mod ist;
#[doc = "Pattern match interrupt control register"]
pub struct PMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt control register"]
pub mod pmctrl;
#[doc = "Pattern match interrupt bit-slice source register"]
pub struct PMSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt bit-slice source register"]
pub mod pmsrc;
#[doc = "Pattern match interrupt bit slice configuration register"]
pub struct PMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt bit slice configuration register"]
pub mod pmcfg;
