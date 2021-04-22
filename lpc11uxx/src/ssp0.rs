#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0. Selects the serial clock rate, bus type, and data size."]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x04 - Control Register 1. Selects master/slave and other modes."]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x08 - Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO."]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - Clock Prescale Register"]
    pub cpsr: crate::Reg<cpsr::CPSR_SPEC>,
    #[doc = "0x14 - Interrupt Mask Set and Clear Register"]
    pub imsc: crate::Reg<imsc::IMSC_SPEC>,
    #[doc = "0x18 - Raw Interrupt Status Register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x1c - Masked Interrupt Status Register"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x20 - SSPICR Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Control Register 0. Selects the serial clock rate, bus type, and data size."]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Register 1. Selects master/slave and other modes."]
pub mod cr1;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO."]
pub mod dr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "CPSR register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod cpsr;
#[doc = "IMSC register accessor: an alias for `Reg<IMSC_SPEC>`"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "Interrupt Mask Set and Clear Register"]
pub mod imsc;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "SSPICR Interrupt Clear Register"]
pub mod icr;
