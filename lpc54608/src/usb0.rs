#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Command/Status register"]
    pub devcmdstat: DEVCMDSTAT,
    #[doc = "0x04 - USB Info register"]
    pub info: INFO,
    #[doc = "0x08 - USB EP Command/Status List start address"]
    pub epliststart: EPLISTSTART,
    #[doc = "0x0c - USB Data buffer start address"]
    pub databufstart: DATABUFSTART,
    #[doc = "0x10 - USB Link Power Management register"]
    pub lpm: LPM,
    #[doc = "0x14 - USB Endpoint skip"]
    pub epskip: EPSKIP,
    #[doc = "0x18 - USB Endpoint Buffer in use"]
    pub epinuse: EPINUSE,
    #[doc = "0x1c - USB Endpoint Buffer Configuration register"]
    pub epbufcfg: EPBUFCFG,
    #[doc = "0x20 - USB interrupt status register"]
    pub intstat: INTSTAT,
    #[doc = "0x24 - USB interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x28 - USB set interrupt status register"]
    pub intsetstat: INTSETSTAT,
    _reserved11: [u8; 8usize],
    #[doc = "0x34 - USB Endpoint toggle register"]
    pub eptoggle: EPTOGGLE,
}
#[doc = "USB Device Command/Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devcmdstat](devcmdstat) module"]
pub type DEVCMDSTAT = crate::Reg<u32, _DEVCMDSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVCMDSTAT;
#[doc = "`read()` method returns [devcmdstat::R](devcmdstat::R) reader structure"]
impl crate::Readable for DEVCMDSTAT {}
#[doc = "`write(|w| ..)` method takes [devcmdstat::W](devcmdstat::W) writer structure"]
impl crate::Writable for DEVCMDSTAT {}
#[doc = "USB Device Command/Status register"]
pub mod devcmdstat;
#[doc = "USB Info register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](info) module"]
pub type INFO = crate::Reg<u32, _INFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INFO;
#[doc = "`read()` method returns [info::R](info::R) reader structure"]
impl crate::Readable for INFO {}
#[doc = "`write(|w| ..)` method takes [info::W](info::W) writer structure"]
impl crate::Writable for INFO {}
#[doc = "USB Info register"]
pub mod info;
#[doc = "USB EP Command/Status List start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epliststart](epliststart) module"]
pub type EPLISTSTART = crate::Reg<u32, _EPLISTSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPLISTSTART;
#[doc = "`read()` method returns [epliststart::R](epliststart::R) reader structure"]
impl crate::Readable for EPLISTSTART {}
#[doc = "`write(|w| ..)` method takes [epliststart::W](epliststart::W) writer structure"]
impl crate::Writable for EPLISTSTART {}
#[doc = "USB EP Command/Status List start address"]
pub mod epliststart;
#[doc = "USB Data buffer start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databufstart](databufstart) module"]
pub type DATABUFSTART = crate::Reg<u32, _DATABUFSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATABUFSTART;
#[doc = "`read()` method returns [databufstart::R](databufstart::R) reader structure"]
impl crate::Readable for DATABUFSTART {}
#[doc = "`write(|w| ..)` method takes [databufstart::W](databufstart::W) writer structure"]
impl crate::Writable for DATABUFSTART {}
#[doc = "USB Data buffer start address"]
pub mod databufstart;
#[doc = "USB Link Power Management register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpm](lpm) module"]
pub type LPM = crate::Reg<u32, _LPM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPM;
#[doc = "`read()` method returns [lpm::R](lpm::R) reader structure"]
impl crate::Readable for LPM {}
#[doc = "`write(|w| ..)` method takes [lpm::W](lpm::W) writer structure"]
impl crate::Writable for LPM {}
#[doc = "USB Link Power Management register"]
pub mod lpm;
#[doc = "USB Endpoint skip\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epskip](epskip) module"]
pub type EPSKIP = crate::Reg<u32, _EPSKIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSKIP;
#[doc = "`read()` method returns [epskip::R](epskip::R) reader structure"]
impl crate::Readable for EPSKIP {}
#[doc = "`write(|w| ..)` method takes [epskip::W](epskip::W) writer structure"]
impl crate::Writable for EPSKIP {}
#[doc = "USB Endpoint skip"]
pub mod epskip;
#[doc = "USB Endpoint Buffer in use\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinuse](epinuse) module"]
pub type EPINUSE = crate::Reg<u32, _EPINUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINUSE;
#[doc = "`read()` method returns [epinuse::R](epinuse::R) reader structure"]
impl crate::Readable for EPINUSE {}
#[doc = "`write(|w| ..)` method takes [epinuse::W](epinuse::W) writer structure"]
impl crate::Writable for EPINUSE {}
#[doc = "USB Endpoint Buffer in use"]
pub mod epinuse;
#[doc = "USB Endpoint Buffer Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epbufcfg](epbufcfg) module"]
pub type EPBUFCFG = crate::Reg<u32, _EPBUFCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPBUFCFG;
#[doc = "`read()` method returns [epbufcfg::R](epbufcfg::R) reader structure"]
impl crate::Readable for EPBUFCFG {}
#[doc = "`write(|w| ..)` method takes [epbufcfg::W](epbufcfg::W) writer structure"]
impl crate::Writable for EPBUFCFG {}
#[doc = "USB Endpoint Buffer Configuration register"]
pub mod epbufcfg;
#[doc = "USB interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "USB interrupt status register"]
pub mod intstat;
#[doc = "USB interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "USB interrupt enable register"]
pub mod inten;
#[doc = "USB set interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsetstat](intsetstat) module"]
pub type INTSETSTAT = crate::Reg<u32, _INTSETSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSETSTAT;
#[doc = "`read()` method returns [intsetstat::R](intsetstat::R) reader structure"]
impl crate::Readable for INTSETSTAT {}
#[doc = "`write(|w| ..)` method takes [intsetstat::W](intsetstat::W) writer structure"]
impl crate::Writable for INTSETSTAT {}
#[doc = "USB set interrupt status register"]
pub mod intsetstat;
#[doc = "USB Endpoint toggle register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptoggle](eptoggle) module"]
pub type EPTOGGLE = crate::Reg<u32, _EPTOGGLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPTOGGLE;
#[doc = "`read()` method returns [eptoggle::R](eptoggle::R) reader structure"]
impl crate::Readable for EPTOGGLE {}
#[doc = "`write(|w| ..)` method takes [eptoggle::W](eptoggle::W) writer structure"]
impl crate::Writable for EPTOGGLE {}
#[doc = "USB Endpoint toggle register"]
pub mod eptoggle;
