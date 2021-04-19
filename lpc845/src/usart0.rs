#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - USART Control register. USART control settings that are more likely to change during operation."]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x08 - USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x0c - Interrupt Enable read and Set register. Contains an individual interrupt enable bit for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x10 - Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x14 - Receiver Data register. Contains the last character received."]
    pub rxdat: crate::Reg<rxdat::RXDAT_SPEC>,
    #[doc = "0x18 - Receiver Data with Status register. Combines the last character received with the current USART receive status. Allows DMA or software to recover incoming data and status together."]
    pub rxdatstat: crate::Reg<rxdatstat::RXDATSTAT_SPEC>,
    #[doc = "0x1c - Transmit Data register. Data to be transmitted is written here."]
    pub txdat: crate::Reg<txdat::TXDAT_SPEC>,
    #[doc = "0x20 - Baud Rate Generator register. 16-bit integer baud rate divisor value."]
    pub brg: crate::Reg<brg::BRG_SPEC>,
    #[doc = "0x24 - Interrupt status register. Reflects interrupts that are currently enabled."]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x28 - Oversample selection register for asynchronous communication."]
    pub osr: crate::Reg<osr::OSR_SPEC>,
    #[doc = "0x2c - Address register for automatic address matching."]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
pub mod cfg;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
pub mod ctl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
pub mod stat;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable read and Set register. Contains an individual interrupt enable bit for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
pub mod intenclr;
#[doc = "RXDAT register accessor: an alias for `Reg<RXDAT_SPEC>`"]
pub type RXDAT = crate::Reg<rxdat::RXDAT_SPEC>;
#[doc = "Receiver Data register. Contains the last character received."]
pub mod rxdat;
#[doc = "RXDATSTAT register accessor: an alias for `Reg<RXDATSTAT_SPEC>`"]
pub type RXDATSTAT = crate::Reg<rxdatstat::RXDATSTAT_SPEC>;
#[doc = "Receiver Data with Status register. Combines the last character received with the current USART receive status. Allows DMA or software to recover incoming data and status together."]
pub mod rxdatstat;
#[doc = "TXDAT register accessor: an alias for `Reg<TXDAT_SPEC>`"]
pub type TXDAT = crate::Reg<txdat::TXDAT_SPEC>;
#[doc = "Transmit Data register. Data to be transmitted is written here."]
pub mod txdat;
#[doc = "BRG register accessor: an alias for `Reg<BRG_SPEC>`"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
pub mod brg;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
pub mod intstat;
#[doc = "OSR register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Oversample selection register for asynchronous communication."]
pub mod osr;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register for automatic address matching."]
pub mod addr;
