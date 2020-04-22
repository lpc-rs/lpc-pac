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
    #[doc = "0x400 - no description available"]
    pub channel0: CHANNEL,
    _reserved16: [u8; 4usize],
    #[doc = "0x410 - no description available"]
    pub channel1: CHANNEL,
    _reserved17: [u8; 4usize],
    #[doc = "0x420 - no description available"]
    pub channel2: CHANNEL,
    _reserved18: [u8; 4usize],
    #[doc = "0x430 - no description available"]
    pub channel3: CHANNEL,
    _reserved19: [u8; 4usize],
    #[doc = "0x440 - no description available"]
    pub channel4: CHANNEL,
    _reserved20: [u8; 4usize],
    #[doc = "0x450 - no description available"]
    pub channel5: CHANNEL,
    _reserved21: [u8; 4usize],
    #[doc = "0x460 - no description available"]
    pub channel6: CHANNEL,
    _reserved22: [u8; 4usize],
    #[doc = "0x470 - no description available"]
    pub channel7: CHANNEL,
    _reserved23: [u8; 4usize],
    #[doc = "0x480 - no description available"]
    pub channel8: CHANNEL,
    _reserved24: [u8; 4usize],
    #[doc = "0x490 - no description available"]
    pub channel9: CHANNEL,
    _reserved25: [u8; 4usize],
    #[doc = "0x4a0 - no description available"]
    pub channel10: CHANNEL,
    _reserved26: [u8; 4usize],
    #[doc = "0x4b0 - no description available"]
    pub channel11: CHANNEL,
    _reserved27: [u8; 4usize],
    #[doc = "0x4c0 - no description available"]
    pub channel12: CHANNEL,
    _reserved28: [u8; 4usize],
    #[doc = "0x4d0 - no description available"]
    pub channel13: CHANNEL,
    _reserved29: [u8; 4usize],
    #[doc = "0x4e0 - no description available"]
    pub channel14: CHANNEL,
    _reserved30: [u8; 4usize],
    #[doc = "0x4f0 - no description available"]
    pub channel15: CHANNEL,
    _reserved31: [u8; 4usize],
    #[doc = "0x500 - no description available"]
    pub channel16: CHANNEL,
    _reserved32: [u8; 4usize],
    #[doc = "0x510 - no description available"]
    pub channel17: CHANNEL,
    _reserved33: [u8; 4usize],
    #[doc = "0x520 - no description available"]
    pub channel18: CHANNEL,
    _reserved34: [u8; 4usize],
    #[doc = "0x530 - no description available"]
    pub channel19: CHANNEL,
    _reserved35: [u8; 4usize],
    #[doc = "0x540 - no description available"]
    pub channel20: CHANNEL,
    _reserved36: [u8; 4usize],
    #[doc = "0x550 - no description available"]
    pub channel21: CHANNEL,
    _reserved37: [u8; 4usize],
    #[doc = "0x560 - no description available"]
    pub channel22: CHANNEL,
    _reserved38: [u8; 4usize],
    #[doc = "0x570 - no description available"]
    pub channel23: CHANNEL,
    _reserved39: [u8; 4usize],
    #[doc = "0x580 - no description available"]
    pub channel24: CHANNEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Configuration register for DMA channel ."]
    pub cfg: self::channel::CFG,
    #[doc = "0x04 - Control and status register for DMA channel ."]
    pub ctlstat: self::channel::CTLSTAT,
    #[doc = "0x08 - Transfer configuration register for DMA channel ."]
    pub xfercfg: self::channel::XFERCFG,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "DMA control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "DMA control."]
pub mod ctrl;
#[doc = "Interrupt status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "Interrupt status."]
pub mod intstat;
#[doc = "SRAM address of the channel configuration table.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srambase](srambase) module"]
pub type SRAMBASE = crate::Reg<u32, _SRAMBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMBASE;
#[doc = "`read()` method returns [srambase::R](srambase::R) reader structure"]
impl crate::Readable for SRAMBASE {}
#[doc = "`write(|w| ..)` method takes [srambase::W](srambase::W) writer structure"]
impl crate::Writable for SRAMBASE {}
#[doc = "SRAM address of the channel configuration table."]
pub mod srambase;
#[doc = "Channel Enable read and Set for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableset0](enableset0) module"]
pub type ENABLESET0 = crate::Reg<u32, _ENABLESET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLESET0;
#[doc = "`read()` method returns [enableset0::R](enableset0::R) reader structure"]
impl crate::Readable for ENABLESET0 {}
#[doc = "`write(|w| ..)` method takes [enableset0::W](enableset0::W) writer structure"]
impl crate::Writable for ENABLESET0 {}
#[doc = "Channel Enable read and Set for all DMA channels."]
pub mod enableset0;
#[doc = "Channel Enable Clear for all DMA channels.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableclr0](enableclr0) module"]
pub type ENABLECLR0 = crate::Reg<u32, _ENABLECLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLECLR0;
#[doc = "`write(|w| ..)` method takes [enableclr0::W](enableclr0::W) writer structure"]
impl crate::Writable for ENABLECLR0 {}
#[doc = "Channel Enable Clear for all DMA channels."]
pub mod enableclr0;
#[doc = "Channel Active status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active0](active0) module"]
pub type ACTIVE0 = crate::Reg<u32, _ACTIVE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTIVE0;
#[doc = "`read()` method returns [active0::R](active0::R) reader structure"]
impl crate::Readable for ACTIVE0 {}
#[doc = "Channel Active status for all DMA channels."]
pub mod active0;
#[doc = "Channel Busy status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy0](busy0) module"]
pub type BUSY0 = crate::Reg<u32, _BUSY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSY0;
#[doc = "`read()` method returns [busy0::R](busy0::R) reader structure"]
impl crate::Readable for BUSY0 {}
#[doc = "Channel Busy status for all DMA channels."]
pub mod busy0;
#[doc = "Error Interrupt status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errint0](errint0) module"]
pub type ERRINT0 = crate::Reg<u32, _ERRINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRINT0;
#[doc = "`read()` method returns [errint0::R](errint0::R) reader structure"]
impl crate::Readable for ERRINT0 {}
#[doc = "`write(|w| ..)` method takes [errint0::W](errint0::W) writer structure"]
impl crate::Writable for ERRINT0 {}
#[doc = "Error Interrupt status for all DMA channels."]
pub mod errint0;
#[doc = "Interrupt Enable read and Set for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset0](intenset0) module"]
pub type INTENSET0 = crate::Reg<u32, _INTENSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET0;
#[doc = "`read()` method returns [intenset0::R](intenset0::R) reader structure"]
impl crate::Readable for INTENSET0 {}
#[doc = "`write(|w| ..)` method takes [intenset0::W](intenset0::W) writer structure"]
impl crate::Writable for INTENSET0 {}
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub mod intenset0;
#[doc = "Interrupt Enable Clear for all DMA channels.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr0](intenclr0) module"]
pub type INTENCLR0 = crate::Reg<u32, _INTENCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR0;
#[doc = "`write(|w| ..)` method takes [intenclr0::W](intenclr0::W) writer structure"]
impl crate::Writable for INTENCLR0 {}
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub mod intenclr0;
#[doc = "Interrupt A status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inta0](inta0) module"]
pub type INTA0 = crate::Reg<u32, _INTA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTA0;
#[doc = "`read()` method returns [inta0::R](inta0::R) reader structure"]
impl crate::Readable for INTA0 {}
#[doc = "`write(|w| ..)` method takes [inta0::W](inta0::W) writer structure"]
impl crate::Writable for INTA0 {}
#[doc = "Interrupt A status for all DMA channels."]
pub mod inta0;
#[doc = "Interrupt B status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intb0](intb0) module"]
pub type INTB0 = crate::Reg<u32, _INTB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTB0;
#[doc = "`read()` method returns [intb0::R](intb0::R) reader structure"]
impl crate::Readable for INTB0 {}
#[doc = "`write(|w| ..)` method takes [intb0::W](intb0::W) writer structure"]
impl crate::Writable for INTB0 {}
#[doc = "Interrupt B status for all DMA channels."]
pub mod intb0;
#[doc = "Set ValidPending control bits for all DMA channels.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setvalid0](setvalid0) module"]
pub type SETVALID0 = crate::Reg<u32, _SETVALID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETVALID0;
#[doc = "`write(|w| ..)` method takes [setvalid0::W](setvalid0::W) writer structure"]
impl crate::Writable for SETVALID0 {}
#[doc = "Set ValidPending control bits for all DMA channels."]
pub mod setvalid0;
#[doc = "Set Trigger control bits for all DMA channels.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [settrig0](settrig0) module"]
pub type SETTRIG0 = crate::Reg<u32, _SETTRIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETTRIG0;
#[doc = "`write(|w| ..)` method takes [settrig0::W](settrig0::W) writer structure"]
impl crate::Writable for SETTRIG0 {}
#[doc = "Set Trigger control bits for all DMA channels."]
pub mod settrig0;
#[doc = "Channel Abort control for all DMA channels.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abort0](abort0) module"]
pub type ABORT0 = crate::Reg<u32, _ABORT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABORT0;
#[doc = "`write(|w| ..)` method takes [abort0::W](abort0::W) writer structure"]
impl crate::Writable for ABORT0 {}
#[doc = "Channel Abort control for all DMA channels."]
pub mod abort0;
