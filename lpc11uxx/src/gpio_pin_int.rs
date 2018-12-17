#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode register"]
    pub isel: ISEL,
    #[doc = "0x04 - Pin Interrupt Enable (Rising) register"]
    pub ienr: IENR,
    #[doc = "0x08 - Set Pin Interrupt Enable (Rising) register"]
    pub sienr: SIENR,
    #[doc = "0x0c - Clear Pin Interrupt Enable (Rising) register"]
    pub cienr: CIENR,
    #[doc = "0x10 - Pin Interrupt Enable Falling Edge / Active Level register"]
    pub ienf: IENF,
    #[doc = "0x14 - Set Pin Interrupt Enable Falling Edge / Active Level register"]
    pub sienf: SIENF,
    #[doc = "0x18 - Clear Pin Interrupt Enable Falling Edge / Active Level address"]
    pub cienf: CIENF,
    #[doc = "0x1c - Pin Interrupt Rising Edge register"]
    pub rise: RISE,
    #[doc = "0x20 - Pin Interrupt Falling Edge register"]
    pub fall: FALL,
    #[doc = "0x24 - Pin Interrupt Status register"]
    pub ist: IST,
}
#[doc = "Pin Interrupt Mode register"]
pub struct ISEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Interrupt Mode register"]
pub mod isel;
#[doc = "Pin Interrupt Enable (Rising) register"]
pub struct IENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Interrupt Enable (Rising) register"]
pub mod ienr;
#[doc = "Set Pin Interrupt Enable (Rising) register"]
pub struct SIENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Pin Interrupt Enable (Rising) register"]
pub mod sienr;
#[doc = "Clear Pin Interrupt Enable (Rising) register"]
pub struct CIENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Pin Interrupt Enable (Rising) register"]
pub mod cienr;
#[doc = "Pin Interrupt Enable Falling Edge / Active Level register"]
pub struct IENF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Interrupt Enable Falling Edge / Active Level register"]
pub mod ienf;
#[doc = "Set Pin Interrupt Enable Falling Edge / Active Level register"]
pub struct SIENF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Pin Interrupt Enable Falling Edge / Active Level register"]
pub mod sienf;
#[doc = "Clear Pin Interrupt Enable Falling Edge / Active Level address"]
pub struct CIENF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Pin Interrupt Enable Falling Edge / Active Level address"]
pub mod cienf;
#[doc = "Pin Interrupt Rising Edge register"]
pub struct RISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Interrupt Rising Edge register"]
pub mod rise;
#[doc = "Pin Interrupt Falling Edge register"]
pub struct FALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Interrupt Falling Edge register"]
pub mod fall;
#[doc = "Pin Interrupt Status register"]
pub struct IST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Interrupt Status register"]
pub mod ist;
