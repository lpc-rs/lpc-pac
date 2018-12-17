#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0. Selects the serial clock rate, bus type, and data size."]
    pub cr0: CR0,
    #[doc = "0x04 - Control Register 1. Selects master/slave and other modes."]
    pub cr1: CR1,
    #[doc = "0x08 - Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO."]
    pub dr: DR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Clock Prescale Register"]
    pub cpsr: CPSR,
    #[doc = "0x14 - Interrupt Mask Set and Clear Register"]
    pub imsc: IMSC,
    #[doc = "0x18 - Raw Interrupt Status Register"]
    pub ris: RIS,
    #[doc = "0x1c - Masked Interrupt Status Register"]
    pub mis: MIS,
    #[doc = "0x20 - SSPICR Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x24 - SSP0 DMA control register"]
    pub dmacr: DMACR,
}
#[doc = "Control Register 0. Selects the serial clock rate, bus type, and data size."]
pub struct CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 0. Selects the serial clock rate, bus type, and data size."]
pub mod cr0;
#[doc = "Control Register 1. Selects master/slave and other modes."]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 1. Selects master/slave and other modes."]
pub mod cr1;
#[doc = "Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO."]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO."]
pub mod dr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Clock Prescale Register"]
pub struct CPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Prescale Register"]
pub mod cpsr;
#[doc = "Interrupt Mask Set and Clear Register"]
pub struct IMSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set and Clear Register"]
pub mod imsc;
#[doc = "Raw Interrupt Status Register"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "Masked Interrupt Status Register"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "SSPICR Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSPICR Interrupt Clear Register"]
pub mod icr;
#[doc = "SSP0 DMA control register"]
pub struct DMACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSP0 DMA control register"]
pub mod dmacr;
