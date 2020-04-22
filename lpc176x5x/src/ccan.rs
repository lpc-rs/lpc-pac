#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Central Transmit Status Register"]
    pub txsr: TXSR,
    #[doc = "0x04 - CAN Central Receive Status Register"]
    pub rxsr: RXSR,
    #[doc = "0x08 - CAN Central Miscellaneous Register"]
    pub msr: MSR,
}
#[doc = "CAN Central Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txsr](txsr) module"]
pub type TXSR = crate::Reg<u32, _TXSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXSR;
#[doc = "`read()` method returns [txsr::R](txsr::R) reader structure"]
impl crate::Readable for TXSR {}
#[doc = "CAN Central Transmit Status Register"]
pub mod txsr;
#[doc = "CAN Central Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxsr](rxsr) module"]
pub type RXSR = crate::Reg<u32, _RXSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXSR;
#[doc = "`read()` method returns [rxsr::R](rxsr::R) reader structure"]
impl crate::Readable for RXSR {}
#[doc = "CAN Central Receive Status Register"]
pub mod rxsr;
#[doc = "CAN Central Miscellaneous Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "CAN Central Miscellaneous Register"]
pub mod msr;
