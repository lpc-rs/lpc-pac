#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - RTC match register"]
    pub match_: MATCH,
    #[doc = "0x08 - RTC counter register"]
    pub count: COUNT,
    #[doc = "0x0c - High-resolution/wake-up timer control register"]
    pub wake: WAKE,
    _reserved4: [u8; 48usize],
    #[doc = "0x40 - General Purpose register"]
    pub gpreg: [GPREG; 8],
}
#[doc = "RTC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RTC control register"]
pub mod ctrl;
#[doc = "RTC match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_](match_) module"]
pub type MATCH = crate::Reg<u32, _MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH;
#[doc = "`read()` method returns [match_::R](match_::R) reader structure"]
impl crate::Readable for MATCH {}
#[doc = "`write(|w| ..)` method takes [match_::W](match_::W) writer structure"]
impl crate::Writable for MATCH {}
#[doc = "RTC match register"]
pub mod match_;
#[doc = "RTC counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count](count) module"]
pub type COUNT = crate::Reg<u32, _COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT;
#[doc = "`read()` method returns [count::R](count::R) reader structure"]
impl crate::Readable for COUNT {}
#[doc = "`write(|w| ..)` method takes [count::W](count::W) writer structure"]
impl crate::Writable for COUNT {}
#[doc = "RTC counter register"]
pub mod count;
#[doc = "High-resolution/wake-up timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake](wake) module"]
pub type WAKE = crate::Reg<u32, _WAKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE;
#[doc = "`read()` method returns [wake::R](wake::R) reader structure"]
impl crate::Readable for WAKE {}
#[doc = "`write(|w| ..)` method takes [wake::W](wake::W) writer structure"]
impl crate::Writable for WAKE {}
#[doc = "High-resolution/wake-up timer control register"]
pub mod wake;
#[doc = "General Purpose register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpreg](gpreg) module"]
pub type GPREG = crate::Reg<u32, _GPREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREG;
#[doc = "`read()` method returns [gpreg::R](gpreg::R) reader structure"]
impl crate::Readable for GPREG {}
#[doc = "`write(|w| ..)` method takes [gpreg::W](gpreg::W) writer structure"]
impl crate::Writable for GPREG {}
#[doc = "General Purpose register"]
pub mod gpreg;
