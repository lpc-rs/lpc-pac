#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA control."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Interrupt status."]
    pub intstat: INTSTAT,
    #[doc = "0x08 - SRAM address of the channel configuration table."]
    pub srambase: SRAMBASE,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - Channel Enable read and Set for all DMA channels."]
    pub enableset0: ENABLESET0,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - Channel Enable Clear for all DMA channels."]
    pub enableclr0: ENABLECLR0,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - Channel Active status for all DMA channels."]
    pub active0: ACTIVE0,
    _reserved3: [u8; 4usize],
    #[doc = "0x38 - Channel Busy status for all DMA channels."]
    pub busy0: BUSY0,
    _reserved4: [u8; 4usize],
    #[doc = "0x40 - Error Interrupt status for all DMA channels."]
    pub errint0: ERRINT0,
    _reserved5: [u8; 4usize],
    #[doc = "0x48 - Interrupt Enable read and Set for all DMA channels."]
    pub intenset0: INTENSET0,
    _reserved6: [u8; 4usize],
    #[doc = "0x50 - Interrupt Enable Clear for all DMA channels."]
    pub intenclr0: INTENCLR0,
    _reserved7: [u8; 4usize],
    #[doc = "0x58 - Interrupt A status for all DMA channels."]
    pub inta0: INTA0,
    _reserved8: [u8; 4usize],
    #[doc = "0x60 - Interrupt B status for all DMA channels."]
    pub intb0: INTB0,
    _reserved9: [u8; 4usize],
    #[doc = "0x68 - Set ValidPending control bits for all DMA channels."]
    pub setvalid0: SETVALID0,
    _reserved10: [u8; 4usize],
    #[doc = "0x70 - Set Trigger control bits for all DMA channels."]
    pub settrig0: SETTRIG0,
    _reserved11: [u8; 4usize],
    #[doc = "0x78 - Channel Abort control for all DMA channels."]
    pub abort0: ABORT0,
    _reserved12: [u8; 900usize],
    #[doc = "0x400 - no description available"]
    pub channel0: CHANNEL,
    _reserved13: [u8; 4usize],
    #[doc = "0x410 - no description available"]
    pub channel1: CHANNEL,
    _reserved14: [u8; 4usize],
    #[doc = "0x420 - no description available"]
    pub channel2: CHANNEL,
    _reserved15: [u8; 4usize],
    #[doc = "0x430 - no description available"]
    pub channel3: CHANNEL,
    _reserved16: [u8; 4usize],
    #[doc = "0x440 - no description available"]
    pub channel4: CHANNEL,
    _reserved17: [u8; 4usize],
    #[doc = "0x450 - no description available"]
    pub channel5: CHANNEL,
    _reserved18: [u8; 4usize],
    #[doc = "0x460 - no description available"]
    pub channel6: CHANNEL,
    _reserved19: [u8; 4usize],
    #[doc = "0x470 - no description available"]
    pub channel7: CHANNEL,
    _reserved20: [u8; 4usize],
    #[doc = "0x480 - no description available"]
    pub channel8: CHANNEL,
    _reserved21: [u8; 4usize],
    #[doc = "0x490 - no description available"]
    pub channel9: CHANNEL,
    _reserved22: [u8; 4usize],
    #[doc = "0x4a0 - no description available"]
    pub channel10: CHANNEL,
    _reserved23: [u8; 4usize],
    #[doc = "0x4b0 - no description available"]
    pub channel11: CHANNEL,
    _reserved24: [u8; 4usize],
    #[doc = "0x4c0 - no description available"]
    pub channel12: CHANNEL,
    _reserved25: [u8; 4usize],
    #[doc = "0x4d0 - no description available"]
    pub channel13: CHANNEL,
    _reserved26: [u8; 4usize],
    #[doc = "0x4e0 - no description available"]
    pub channel14: CHANNEL,
    _reserved27: [u8; 4usize],
    #[doc = "0x4f0 - no description available"]
    pub channel15: CHANNEL,
    _reserved28: [u8; 4usize],
    #[doc = "0x500 - no description available"]
    pub channel16: CHANNEL,
    _reserved29: [u8; 4usize],
    #[doc = "0x510 - no description available"]
    pub channel17: CHANNEL,
    _reserved30: [u8; 4usize],
    #[doc = "0x520 - no description available"]
    pub channel18: CHANNEL,
    _reserved31: [u8; 4usize],
    #[doc = "0x530 - no description available"]
    pub channel19: CHANNEL,
    _reserved32: [u8; 4usize],
    #[doc = "0x540 - no description available"]
    pub channel20: CHANNEL,
    _reserved33: [u8; 4usize],
    #[doc = "0x550 - no description available"]
    pub channel21: CHANNEL,
    _reserved34: [u8; 4usize],
    #[doc = "0x560 - no description available"]
    pub channel22: CHANNEL,
    _reserved35: [u8; 4usize],
    #[doc = "0x570 - no description available"]
    pub channel23: CHANNEL,
    _reserved36: [u8; 4usize],
    #[doc = "0x580 - no description available"]
    pub channel24: CHANNEL,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Configuration register for DMA channel ."]
    pub cfg: self::channel::CFG,
    #[doc = "0x04 - Control and status register for DMA channel ."]
    pub ctlstat: self::channel::CTLSTAT,
    #[doc = "0x08 - Transfer configuration register for DMA channel ."]
    pub xfercfg: self::channel::XFERCFG,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "DMA control."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA control."]
pub mod ctrl;
#[doc = "Interrupt status."]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status."]
pub mod intstat;
#[doc = "SRAM address of the channel configuration table."]
pub struct SRAMBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM address of the channel configuration table."]
pub mod srambase;
#[doc = "Channel Enable read and Set for all DMA channels."]
pub struct ENABLESET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable read and Set for all DMA channels."]
pub mod enableset0;
#[doc = "Channel Enable Clear for all DMA channels."]
pub struct ENABLECLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Clear for all DMA channels."]
pub mod enableclr0;
#[doc = "Channel Active status for all DMA channels."]
pub struct ACTIVE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Active status for all DMA channels."]
pub mod active0;
#[doc = "Channel Busy status for all DMA channels."]
pub struct BUSY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Busy status for all DMA channels."]
pub mod busy0;
#[doc = "Error Interrupt status for all DMA channels."]
pub struct ERRINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Interrupt status for all DMA channels."]
pub mod errint0;
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub struct INTENSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub mod intenset0;
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub struct INTENCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub mod intenclr0;
#[doc = "Interrupt A status for all DMA channels."]
pub struct INTA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt A status for all DMA channels."]
pub mod inta0;
#[doc = "Interrupt B status for all DMA channels."]
pub struct INTB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt B status for all DMA channels."]
pub mod intb0;
#[doc = "Set ValidPending control bits for all DMA channels."]
pub struct SETVALID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set ValidPending control bits for all DMA channels."]
pub mod setvalid0;
#[doc = "Set Trigger control bits for all DMA channels."]
pub struct SETTRIG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Trigger control bits for all DMA channels."]
pub mod settrig0;
#[doc = "Channel Abort control for all DMA channels."]
pub struct ABORT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Abort control for all DMA channels."]
pub mod abort0;
