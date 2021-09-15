#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA control."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Interrupt status."]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x08 - SRAM address of the channel configuration table."]
    pub srambase: crate::Reg<srambase::SRAMBASE_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - Channel Enable read and Set for all DMA channels."]
    pub enableset0: crate::Reg<enableset0::ENABLESET0_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x28 - Channel Enable Clear for all DMA channels."]
    pub enableclr0: crate::Reg<enableclr0::ENABLECLR0_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x30 - Channel Active status for all DMA channels."]
    pub active0: crate::Reg<active0::ACTIVE0_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x38 - Channel Busy status for all DMA channels."]
    pub busy0: crate::Reg<busy0::BUSY0_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x40 - Error Interrupt status for all DMA channels."]
    pub errint0: crate::Reg<errint0::ERRINT0_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x48 - Interrupt Enable read and Set for all DMA channels."]
    pub intenset0: crate::Reg<intenset0::INTENSET0_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x50 - Interrupt Enable Clear for all DMA channels."]
    pub intenclr0: crate::Reg<intenclr0::INTENCLR0_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x58 - Interrupt A status for all DMA channels."]
    pub inta0: crate::Reg<inta0::INTA0_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x60 - Interrupt B status for all DMA channels."]
    pub intb0: crate::Reg<intb0::INTB0_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x68 - Set ValidPending control bits for all DMA channels."]
    pub setvalid0: crate::Reg<setvalid0::SETVALID0_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x70 - Set Trigger control bits for all DMA channels."]
    pub settrig0: crate::Reg<settrig0::SETTRIG0_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x78 - Channel Abort control for all DMA channels."]
    pub abort0: crate::Reg<abort0::ABORT0_SPEC>,
    _reserved15: [u8; 0x0384],
    #[doc = "0x400..0x40c - no description available"]
    pub channel0: CHANNEL,
    _reserved16: [u8; 0x04],
    #[doc = "0x410..0x41c - no description available"]
    pub channel1: CHANNEL,
    _reserved17: [u8; 0x04],
    #[doc = "0x420..0x42c - no description available"]
    pub channel2: CHANNEL,
    _reserved18: [u8; 0x04],
    #[doc = "0x430..0x43c - no description available"]
    pub channel3: CHANNEL,
    _reserved19: [u8; 0x04],
    #[doc = "0x440..0x44c - no description available"]
    pub channel4: CHANNEL,
    _reserved20: [u8; 0x04],
    #[doc = "0x450..0x45c - no description available"]
    pub channel5: CHANNEL,
    _reserved21: [u8; 0x04],
    #[doc = "0x460..0x46c - no description available"]
    pub channel6: CHANNEL,
    _reserved22: [u8; 0x04],
    #[doc = "0x470..0x47c - no description available"]
    pub channel7: CHANNEL,
    _reserved23: [u8; 0x04],
    #[doc = "0x480..0x48c - no description available"]
    pub channel8: CHANNEL,
    _reserved24: [u8; 0x04],
    #[doc = "0x490..0x49c - no description available"]
    pub channel9: CHANNEL,
    _reserved25: [u8; 0x04],
    #[doc = "0x4a0..0x4ac - no description available"]
    pub channel10: CHANNEL,
    _reserved26: [u8; 0x04],
    #[doc = "0x4b0..0x4bc - no description available"]
    pub channel11: CHANNEL,
    _reserved27: [u8; 0x04],
    #[doc = "0x4c0..0x4cc - no description available"]
    pub channel12: CHANNEL,
    _reserved28: [u8; 0x04],
    #[doc = "0x4d0..0x4dc - no description available"]
    pub channel13: CHANNEL,
    _reserved29: [u8; 0x04],
    #[doc = "0x4e0..0x4ec - no description available"]
    pub channel14: CHANNEL,
    _reserved30: [u8; 0x04],
    #[doc = "0x4f0..0x4fc - no description available"]
    pub channel15: CHANNEL,
    _reserved31: [u8; 0x04],
    #[doc = "0x500..0x50c - no description available"]
    pub channel16: CHANNEL,
    _reserved32: [u8; 0x04],
    #[doc = "0x510..0x51c - no description available"]
    pub channel17: CHANNEL,
    _reserved33: [u8; 0x04],
    #[doc = "0x520..0x52c - no description available"]
    pub channel18: CHANNEL,
    _reserved34: [u8; 0x04],
    #[doc = "0x530..0x53c - no description available"]
    pub channel19: CHANNEL,
    _reserved35: [u8; 0x04],
    #[doc = "0x540..0x54c - no description available"]
    pub channel20: CHANNEL,
    _reserved36: [u8; 0x04],
    #[doc = "0x550..0x55c - no description available"]
    pub channel21: CHANNEL,
    _reserved37: [u8; 0x04],
    #[doc = "0x560..0x56c - no description available"]
    pub channel22: CHANNEL,
    _reserved38: [u8; 0x04],
    #[doc = "0x570..0x57c - no description available"]
    pub channel23: CHANNEL,
    _reserved39: [u8; 0x04],
    #[doc = "0x580..0x58c - no description available"]
    pub channel24: CHANNEL,
    _reserved40: [u8; 0x04],
    #[doc = "0x590..0x59c - no description available"]
    pub channel25: CHANNEL,
    _reserved41: [u8; 0x04],
    #[doc = "0x5a0..0x5ac - no description available"]
    pub channel26: CHANNEL,
    _reserved42: [u8; 0x04],
    #[doc = "0x5b0..0x5bc - no description available"]
    pub channel27: CHANNEL,
    _reserved43: [u8; 0x04],
    #[doc = "0x5c0..0x5cc - no description available"]
    pub channel28: CHANNEL,
    _reserved44: [u8; 0x04],
    #[doc = "0x5d0..0x5dc - no description available"]
    pub channel29: CHANNEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Configuration register for DMA channel ."]
    pub cfg: crate::Reg<self::channel::cfg::CFG_SPEC>,
    #[doc = "0x04 - Control and status register for DMA channel ."]
    pub ctlstat: crate::Reg<self::channel::ctlstat::CTLSTAT_SPEC>,
    #[doc = "0x08 - Transfer configuration register for DMA channel ."]
    pub xfercfg: crate::Reg<self::channel::xfercfg::XFERCFG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA control."]
pub mod ctrl;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt status."]
pub mod intstat;
#[doc = "SRAMBASE register accessor: an alias for `Reg<SRAMBASE_SPEC>`"]
pub type SRAMBASE = crate::Reg<srambase::SRAMBASE_SPEC>;
#[doc = "SRAM address of the channel configuration table."]
pub mod srambase;
#[doc = "ENABLESET0 register accessor: an alias for `Reg<ENABLESET0_SPEC>`"]
pub type ENABLESET0 = crate::Reg<enableset0::ENABLESET0_SPEC>;
#[doc = "Channel Enable read and Set for all DMA channels."]
pub mod enableset0;
#[doc = "ENABLECLR0 register accessor: an alias for `Reg<ENABLECLR0_SPEC>`"]
pub type ENABLECLR0 = crate::Reg<enableclr0::ENABLECLR0_SPEC>;
#[doc = "Channel Enable Clear for all DMA channels."]
pub mod enableclr0;
#[doc = "ACTIVE0 register accessor: an alias for `Reg<ACTIVE0_SPEC>`"]
pub type ACTIVE0 = crate::Reg<active0::ACTIVE0_SPEC>;
#[doc = "Channel Active status for all DMA channels."]
pub mod active0;
#[doc = "BUSY0 register accessor: an alias for `Reg<BUSY0_SPEC>`"]
pub type BUSY0 = crate::Reg<busy0::BUSY0_SPEC>;
#[doc = "Channel Busy status for all DMA channels."]
pub mod busy0;
#[doc = "ERRINT0 register accessor: an alias for `Reg<ERRINT0_SPEC>`"]
pub type ERRINT0 = crate::Reg<errint0::ERRINT0_SPEC>;
#[doc = "Error Interrupt status for all DMA channels."]
pub mod errint0;
#[doc = "INTENSET0 register accessor: an alias for `Reg<INTENSET0_SPEC>`"]
pub type INTENSET0 = crate::Reg<intenset0::INTENSET0_SPEC>;
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub mod intenset0;
#[doc = "INTENCLR0 register accessor: an alias for `Reg<INTENCLR0_SPEC>`"]
pub type INTENCLR0 = crate::Reg<intenclr0::INTENCLR0_SPEC>;
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub mod intenclr0;
#[doc = "INTA0 register accessor: an alias for `Reg<INTA0_SPEC>`"]
pub type INTA0 = crate::Reg<inta0::INTA0_SPEC>;
#[doc = "Interrupt A status for all DMA channels."]
pub mod inta0;
#[doc = "INTB0 register accessor: an alias for `Reg<INTB0_SPEC>`"]
pub type INTB0 = crate::Reg<intb0::INTB0_SPEC>;
#[doc = "Interrupt B status for all DMA channels."]
pub mod intb0;
#[doc = "SETVALID0 register accessor: an alias for `Reg<SETVALID0_SPEC>`"]
pub type SETVALID0 = crate::Reg<setvalid0::SETVALID0_SPEC>;
#[doc = "Set ValidPending control bits for all DMA channels."]
pub mod setvalid0;
#[doc = "SETTRIG0 register accessor: an alias for `Reg<SETTRIG0_SPEC>`"]
pub type SETTRIG0 = crate::Reg<settrig0::SETTRIG0_SPEC>;
#[doc = "Set Trigger control bits for all DMA channels."]
pub mod settrig0;
#[doc = "ABORT0 register accessor: an alias for `Reg<ABORT0_SPEC>`"]
pub type ABORT0 = crate::Reg<abort0::ABORT0_SPEC>;
#[doc = "Channel Abort control for all DMA channels."]
pub mod abort0;
