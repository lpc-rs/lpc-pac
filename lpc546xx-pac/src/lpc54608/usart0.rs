///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - USART Configuration register. Basic USART configuration settings that typically are not changed during operation.
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    ///0x04 - USART Control register. USART control settings that are more likely to change during operation.
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    ///0x08 - USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them.
    pub stat: crate::Reg<stat::STAT_SPEC>,
    ///0x0c - Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    ///0x10 - Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared.
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x0c],
    ///0x20 - Baud Rate Generator register. 16-bit integer baud rate divisor value.
    pub brg: crate::Reg<brg::BRG_SPEC>,
    ///0x24 - Interrupt status register. Reflects interrupts that are currently enabled.
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    ///0x28 - Oversample selection register for asynchronous communication.
    pub osr: crate::Reg<osr::OSR_SPEC>,
    ///0x2c - Address register for automatic address matching.
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    _reserved9: [u8; 0x0dd0],
    ///0xe00 - FIFO configuration and enable register.
    pub fifocfg: crate::Reg<fifocfg::FIFOCFG_SPEC>,
    ///0xe04 - FIFO status register.
    pub fifostat: crate::Reg<fifostat::FIFOSTAT_SPEC>,
    ///0xe08 - FIFO trigger settings for interrupt and DMA request.
    pub fifotrig: crate::Reg<fifotrig::FIFOTRIG_SPEC>,
    _reserved12: [u8; 0x04],
    ///0xe10 - FIFO interrupt enable set (enable) and read register.
    pub fifointenset: crate::Reg<fifointenset::FIFOINTENSET_SPEC>,
    ///0xe14 - FIFO interrupt enable clear (disable) and read register.
    pub fifointenclr: crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>,
    ///0xe18 - FIFO interrupt status register.
    pub fifointstat: crate::Reg<fifointstat::FIFOINTSTAT_SPEC>,
    _reserved15: [u8; 0x04],
    ///0xe20 - FIFO write data.
    pub fifowr: crate::Reg<fifowr::FIFOWR_SPEC>,
    _reserved16: [u8; 0x0c],
    ///0xe30 - FIFO read data.
    pub fiford: crate::Reg<fiford::FIFORD_SPEC>,
    _reserved17: [u8; 0x0c],
    ///0xe40 - FIFO data read with no FIFO pop.
    pub fifordnopop: crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>,
    _reserved18: [u8; 0x01b8],
    ///0xffc - Peripheral identification register.
    pub id: crate::Reg<id::ID_SPEC>,
}
///CFG register accessor: an alias for `Reg<CFG_SPEC>`
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
///USART Configuration register. Basic USART configuration settings that typically are not changed during operation.
pub mod cfg;
///CTL register accessor: an alias for `Reg<CTL_SPEC>`
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
///USART Control register. USART control settings that are more likely to change during operation.
pub mod ctl;
///STAT register accessor: an alias for `Reg<STAT_SPEC>`
pub type STAT = crate::Reg<stat::STAT_SPEC>;
///USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them.
pub mod stat;
///INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
///Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.
pub mod intenset;
///INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
///Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared.
pub mod intenclr;
///BRG register accessor: an alias for `Reg<BRG_SPEC>`
pub type BRG = crate::Reg<brg::BRG_SPEC>;
///Baud Rate Generator register. 16-bit integer baud rate divisor value.
pub mod brg;
///INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
///Interrupt status register. Reflects interrupts that are currently enabled.
pub mod intstat;
///OSR register accessor: an alias for `Reg<OSR_SPEC>`
pub type OSR = crate::Reg<osr::OSR_SPEC>;
///Oversample selection register for asynchronous communication.
pub mod osr;
///ADDR register accessor: an alias for `Reg<ADDR_SPEC>`
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
///Address register for automatic address matching.
pub mod addr;
///FIFOCFG register accessor: an alias for `Reg<FIFOCFG_SPEC>`
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
///FIFO configuration and enable register.
pub mod fifocfg;
///FIFOSTAT register accessor: an alias for `Reg<FIFOSTAT_SPEC>`
pub type FIFOSTAT = crate::Reg<fifostat::FIFOSTAT_SPEC>;
///FIFO status register.
pub mod fifostat;
///FIFOTRIG register accessor: an alias for `Reg<FIFOTRIG_SPEC>`
pub type FIFOTRIG = crate::Reg<fifotrig::FIFOTRIG_SPEC>;
///FIFO trigger settings for interrupt and DMA request.
pub mod fifotrig;
///FIFOINTENSET register accessor: an alias for `Reg<FIFOINTENSET_SPEC>`
pub type FIFOINTENSET = crate::Reg<fifointenset::FIFOINTENSET_SPEC>;
///FIFO interrupt enable set (enable) and read register.
pub mod fifointenset;
///FIFOINTENCLR register accessor: an alias for `Reg<FIFOINTENCLR_SPEC>`
pub type FIFOINTENCLR = crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>;
///FIFO interrupt enable clear (disable) and read register.
pub mod fifointenclr;
///FIFOINTSTAT register accessor: an alias for `Reg<FIFOINTSTAT_SPEC>`
pub type FIFOINTSTAT = crate::Reg<fifointstat::FIFOINTSTAT_SPEC>;
///FIFO interrupt status register.
pub mod fifointstat;
///FIFOWR register accessor: an alias for `Reg<FIFOWR_SPEC>`
pub type FIFOWR = crate::Reg<fifowr::FIFOWR_SPEC>;
///FIFO write data.
pub mod fifowr;
///FIFORD register accessor: an alias for `Reg<FIFORD_SPEC>`
pub type FIFORD = crate::Reg<fiford::FIFORD_SPEC>;
///FIFO read data.
pub mod fiford;
///FIFORDNOPOP register accessor: an alias for `Reg<FIFORDNOPOP_SPEC>`
pub type FIFORDNOPOP = crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>;
///FIFO data read with no FIFO pop.
pub mod fifordnopop;
///ID register accessor: an alias for `Reg<ID_SPEC>`
pub type ID = crate::Reg<id::ID_SPEC>;
///Peripheral identification register.
pub mod id;
