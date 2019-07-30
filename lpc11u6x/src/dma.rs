#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA control."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Interrupt status."]
    pub intstat: INTSTAT,
    #[doc = "0x08 - SRAM address of the channel configuration table."]
    pub srambase: SRAMBASE,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - Channel Enable read and Set for all DMA channels."]
    pub enableset0: ENABLESET0,
    _reserved4: [u8; 4usize],
    #[doc = "0x28 - Channel Enable Clear for all DMA channels."]
    pub enableclr0: ENABLECLR0,
    _reserved5: [u8; 4usize],
    #[doc = "0x30 - Channel Active status for all DMA channels."]
    pub active0: ACTIVE0,
    _reserved6: [u8; 4usize],
    #[doc = "0x38 - Channel Busy status for all DMA channels."]
    pub busy0: BUSY0,
    _reserved7: [u8; 4usize],
    #[doc = "0x40 - Error Interrupt status for all DMA channels."]
    pub errint0: ERRINT0,
    _reserved8: [u8; 4usize],
    #[doc = "0x48 - Interrupt Enable read and Set for all DMA channels."]
    pub intenset0: INTENSET0,
    _reserved9: [u8; 4usize],
    #[doc = "0x50 - Interrupt Enable Clear for all DMA channels."]
    pub intenclr0: INTENCLR0,
    _reserved10: [u8; 4usize],
    #[doc = "0x58 - Interrupt A status for all DMA channels."]
    pub inta0: INTA0,
    _reserved11: [u8; 4usize],
    #[doc = "0x60 - Interrupt B status for all DMA channels."]
    pub intb0: INTB0,
    _reserved12: [u8; 4usize],
    #[doc = "0x68 - Set ValidPending control bits for all DMA channels."]
    pub setvalid0: SETVALID0,
    _reserved13: [u8; 4usize],
    #[doc = "0x70 - Set Trigger control bits for all DMA channels."]
    pub settrig0: SETTRIG0,
    _reserved14: [u8; 4usize],
    #[doc = "0x78 - Channel Abort control for all DMA channels."]
    pub abort0: ABORT0,
    _reserved15: [u8; 900usize],
    #[doc = "0x400 - Configuration register for DMA channel 0."]
    pub cfg0: CFG,
    #[doc = "0x404 - Control and status register for DMA channel 0."]
    pub ctlstat0: CTLSTAT,
    #[doc = "0x408 - Transfer configuration register for DMA channel 0."]
    pub xfercfg0: XFERCFG,
    _reserved18: [u8; 4usize],
    #[doc = "0x410 - Configuration register for DMA channel 0."]
    pub cfg1: CFG,
    #[doc = "0x414 - Control and status register for DMA channel 0."]
    pub ctlstat1: CTLSTAT,
    #[doc = "0x418 - Transfer configuration register for DMA channel 0."]
    pub xfercfg1: XFERCFG,
    _reserved21: [u8; 4usize],
    #[doc = "0x420 - Configuration register for DMA channel 0."]
    pub cfg2: CFG,
    #[doc = "0x424 - Control and status register for DMA channel 0."]
    pub ctlstat2: CTLSTAT,
    #[doc = "0x428 - Transfer configuration register for DMA channel 0."]
    pub xfercfg2: XFERCFG,
    _reserved24: [u8; 4usize],
    #[doc = "0x430 - Configuration register for DMA channel 0."]
    pub cfg3: CFG,
    #[doc = "0x434 - Control and status register for DMA channel 0."]
    pub ctlstat3: CTLSTAT,
    #[doc = "0x438 - Transfer configuration register for DMA channel 0."]
    pub xfercfg3: XFERCFG,
    _reserved27: [u8; 4usize],
    #[doc = "0x440 - Configuration register for DMA channel 0."]
    pub cfg4: CFG,
    #[doc = "0x444 - Control and status register for DMA channel 0."]
    pub ctlstat4: CTLSTAT,
    #[doc = "0x448 - Transfer configuration register for DMA channel 0."]
    pub xfercfg4: XFERCFG,
    _reserved30: [u8; 4usize],
    #[doc = "0x450 - Configuration register for DMA channel 0."]
    pub cfg5: CFG,
    #[doc = "0x454 - Control and status register for DMA channel 0."]
    pub ctlstat5: CTLSTAT,
    #[doc = "0x458 - Transfer configuration register for DMA channel 0."]
    pub xfercfg5: XFERCFG,
    _reserved33: [u8; 4usize],
    #[doc = "0x460 - Configuration register for DMA channel 0."]
    pub cfg6: CFG,
    #[doc = "0x464 - Control and status register for DMA channel 0."]
    pub ctlstat6: CTLSTAT,
    #[doc = "0x468 - Transfer configuration register for DMA channel 0."]
    pub xfercfg6: XFERCFG,
    _reserved36: [u8; 4usize],
    #[doc = "0x470 - Configuration register for DMA channel 0."]
    pub cfg7: CFG,
    #[doc = "0x474 - Control and status register for DMA channel 0."]
    pub ctlstat7: CTLSTAT,
    #[doc = "0x478 - Transfer configuration register for DMA channel 0."]
    pub xfercfg7: XFERCFG,
    _reserved39: [u8; 4usize],
    #[doc = "0x480 - Configuration register for DMA channel 0."]
    pub cfg8: CFG,
    #[doc = "0x484 - Control and status register for DMA channel 0."]
    pub ctlstat8: CTLSTAT,
    #[doc = "0x488 - Transfer configuration register for DMA channel 0."]
    pub xfercfg8: XFERCFG,
    _reserved42: [u8; 4usize],
    #[doc = "0x490 - Configuration register for DMA channel 0."]
    pub cfg9: CFG,
    #[doc = "0x494 - Control and status register for DMA channel 0."]
    pub ctlstat9: CTLSTAT,
    #[doc = "0x498 - Transfer configuration register for DMA channel 0."]
    pub xfercfg9: XFERCFG,
    _reserved45: [u8; 4usize],
    #[doc = "0x4a0 - Configuration register for DMA channel 0."]
    pub cfg10: CFG,
    #[doc = "0x4a4 - Control and status register for DMA channel 0."]
    pub ctlstat10: CTLSTAT,
    #[doc = "0x4a8 - Transfer configuration register for DMA channel 0."]
    pub xfercfg10: XFERCFG,
    _reserved48: [u8; 4usize],
    #[doc = "0x4b0 - Configuration register for DMA channel 0."]
    pub cfg11: CFG,
    #[doc = "0x4b4 - Control and status register for DMA channel 0."]
    pub ctlstat11: CTLSTAT,
    #[doc = "0x4b8 - Transfer configuration register for DMA channel 0."]
    pub xfercfg11: XFERCFG,
    _reserved51: [u8; 4usize],
    #[doc = "0x4c0 - Configuration register for DMA channel 0."]
    pub cfg12: CFG,
    #[doc = "0x4c4 - Control and status register for DMA channel 0."]
    pub ctlstat12: CTLSTAT,
    #[doc = "0x4c8 - Transfer configuration register for DMA channel 0."]
    pub xfercfg12: XFERCFG,
    _reserved54: [u8; 4usize],
    #[doc = "0x4d0 - Configuration register for DMA channel 0."]
    pub cfg13: CFG,
    #[doc = "0x4d4 - Control and status register for DMA channel 0."]
    pub ctlstat13: CTLSTAT,
    #[doc = "0x4d8 - Transfer configuration register for DMA channel 0."]
    pub xfercfg13: XFERCFG,
    _reserved57: [u8; 4usize],
    #[doc = "0x4e0 - Configuration register for DMA channel 0."]
    pub cfg14: CFG,
    #[doc = "0x4e4 - Control and status register for DMA channel 0."]
    pub ctlstat14: CTLSTAT,
    #[doc = "0x4e8 - Transfer configuration register for DMA channel 0."]
    pub xfercfg14: XFERCFG,
    _reserved60: [u8; 4usize],
    #[doc = "0x4f0 - Configuration register for DMA channel 0."]
    pub cfg15: CFG,
    #[doc = "0x4f4 - Control and status register for DMA channel 0."]
    pub ctlstat15: CTLSTAT,
    #[doc = "0x4f8 - Transfer configuration register for DMA channel 0."]
    pub xfercfg15: XFERCFG,
}
#[doc = "DMA control."]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DMA control."]
pub mod ctrl;
#[doc = "Interrupt status."]
pub struct INTSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status."]
pub mod intstat;
#[doc = "SRAM address of the channel configuration table."]
pub struct SRAMBASE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SRAM address of the channel configuration table."]
pub mod srambase;
#[doc = "Channel Enable read and Set for all DMA channels."]
pub struct ENABLESET0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable read and Set for all DMA channels."]
pub mod enableset0;
#[doc = "Channel Enable Clear for all DMA channels."]
pub struct ENABLECLR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Clear for all DMA channels."]
pub mod enableclr0;
#[doc = "Channel Active status for all DMA channels."]
pub struct ACTIVE0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Active status for all DMA channels."]
pub mod active0;
#[doc = "Channel Busy status for all DMA channels."]
pub struct BUSY0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Busy status for all DMA channels."]
pub mod busy0;
#[doc = "Error Interrupt status for all DMA channels."]
pub struct ERRINT0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Error Interrupt status for all DMA channels."]
pub mod errint0;
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub struct INTENSET0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub mod intenset0;
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub struct INTENCLR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub mod intenclr0;
#[doc = "Interrupt A status for all DMA channels."]
pub struct INTA0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt A status for all DMA channels."]
pub mod inta0;
#[doc = "Interrupt B status for all DMA channels."]
pub struct INTB0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt B status for all DMA channels."]
pub mod intb0;
#[doc = "Set ValidPending control bits for all DMA channels."]
pub struct SETVALID0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set ValidPending control bits for all DMA channels."]
pub mod setvalid0;
#[doc = "Set Trigger control bits for all DMA channels."]
pub struct SETTRIG0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set Trigger control bits for all DMA channels."]
pub mod settrig0;
#[doc = "Channel Abort control for all DMA channels."]
pub struct ABORT0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Abort control for all DMA channels."]
pub mod abort0;
#[doc = "Configuration register for DMA channel 0."]
pub struct CFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configuration register for DMA channel 0."]
pub mod cfg;
#[doc = "Control and status register for DMA channel 0."]
pub struct CTLSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control and status register for DMA channel 0."]
pub mod ctlstat;
#[doc = "Transfer configuration register for DMA channel 0."]
pub struct XFERCFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transfer configuration register for DMA channel 0."]
pub mod xfercfg;
