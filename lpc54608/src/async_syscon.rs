#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Async peripheral reset control"]
    pub asyncpresetctrl: ASYNCPRESETCTRL,
    #[doc = "0x04 - Set bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlset: ASYNCPRESETCTRLSET,
    #[doc = "0x08 - Clear bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlclr: ASYNCPRESETCTRLCLR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Async peripheral clock control"]
    pub asyncapbclkctrl: ASYNCAPBCLKCTRL,
    #[doc = "0x14 - Set bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlset: ASYNCAPBCLKCTRLSET,
    #[doc = "0x18 - Clear bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlclr: ASYNCAPBCLKCTRLCLR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Async APB clock source select A"]
    pub asyncapbclksela: ASYNCAPBCLKSELA,
}
#[doc = "Async peripheral reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncpresetctrl](asyncpresetctrl) module"]
pub type ASYNCPRESETCTRL = crate::Reg<u32, _ASYNCPRESETCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCPRESETCTRL;
#[doc = "`read()` method returns [asyncpresetctrl::R](asyncpresetctrl::R) reader structure"]
impl crate::Readable for ASYNCPRESETCTRL {}
#[doc = "`write(|w| ..)` method takes [asyncpresetctrl::W](asyncpresetctrl::W) writer structure"]
impl crate::Writable for ASYNCPRESETCTRL {}
#[doc = "Async peripheral reset control"]
pub mod asyncpresetctrl;
#[doc = "Set bits in ASYNCPRESETCTRL\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncpresetctrlset](asyncpresetctrlset) module"]
pub type ASYNCPRESETCTRLSET = crate::Reg<u32, _ASYNCPRESETCTRLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCPRESETCTRLSET;
#[doc = "`write(|w| ..)` method takes [asyncpresetctrlset::W](asyncpresetctrlset::W) writer structure"]
impl crate::Writable for ASYNCPRESETCTRLSET {}
#[doc = "Set bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlset;
#[doc = "Clear bits in ASYNCPRESETCTRL\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncpresetctrlclr](asyncpresetctrlclr) module"]
pub type ASYNCPRESETCTRLCLR = crate::Reg<u32, _ASYNCPRESETCTRLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCPRESETCTRLCLR;
#[doc = "`write(|w| ..)` method takes [asyncpresetctrlclr::W](asyncpresetctrlclr::W) writer structure"]
impl crate::Writable for ASYNCPRESETCTRLCLR {}
#[doc = "Clear bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlclr;
#[doc = "Async peripheral clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbclkctrl](asyncapbclkctrl) module"]
pub type ASYNCAPBCLKCTRL = crate::Reg<u32, _ASYNCAPBCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCAPBCLKCTRL;
#[doc = "`read()` method returns [asyncapbclkctrl::R](asyncapbclkctrl::R) reader structure"]
impl crate::Readable for ASYNCAPBCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [asyncapbclkctrl::W](asyncapbclkctrl::W) writer structure"]
impl crate::Writable for ASYNCAPBCLKCTRL {}
#[doc = "Async peripheral clock control"]
pub mod asyncapbclkctrl;
#[doc = "Set bits in ASYNCAPBCLKCTRL\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbclkctrlset](asyncapbclkctrlset) module"]
pub type ASYNCAPBCLKCTRLSET = crate::Reg<u32, _ASYNCAPBCLKCTRLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCAPBCLKCTRLSET;
#[doc = "`write(|w| ..)` method takes [asyncapbclkctrlset::W](asyncapbclkctrlset::W) writer structure"]
impl crate::Writable for ASYNCAPBCLKCTRLSET {}
#[doc = "Set bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlset;
#[doc = "Clear bits in ASYNCAPBCLKCTRL\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbclkctrlclr](asyncapbclkctrlclr) module"]
pub type ASYNCAPBCLKCTRLCLR = crate::Reg<u32, _ASYNCAPBCLKCTRLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCAPBCLKCTRLCLR;
#[doc = "`write(|w| ..)` method takes [asyncapbclkctrlclr::W](asyncapbclkctrlclr::W) writer structure"]
impl crate::Writable for ASYNCAPBCLKCTRLCLR {}
#[doc = "Clear bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlclr;
#[doc = "Async APB clock source select A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbclksela](asyncapbclksela) module"]
pub type ASYNCAPBCLKSELA = crate::Reg<u32, _ASYNCAPBCLKSELA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCAPBCLKSELA;
#[doc = "`read()` method returns [asyncapbclksela::R](asyncapbclksela::R) reader structure"]
impl crate::Readable for ASYNCAPBCLKSELA {}
#[doc = "`write(|w| ..)` method takes [asyncapbclksela::W](asyncapbclksela::W) writer structure"]
impl crate::Writable for ASYNCAPBCLKSELA {}
#[doc = "Async APB clock source select A"]
pub mod asyncapbclksela;
