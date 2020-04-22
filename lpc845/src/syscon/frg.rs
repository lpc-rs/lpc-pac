#[doc = "fractional generator N divider value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frgdiv](frgdiv) module"]
pub type FRGDIV = crate::Reg<u32, _FRGDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRGDIV;
#[doc = "`read()` method returns [frgdiv::R](frgdiv::R) reader structure"]
impl crate::Readable for FRGDIV {}
#[doc = "`write(|w| ..)` method takes [frgdiv::W](frgdiv::W) writer structure"]
impl crate::Writable for FRGDIV {}
#[doc = "fractional generator N divider value register"]
pub mod frgdiv;
#[doc = "fractional generator N multiplier value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frgmult](frgmult) module"]
pub type FRGMULT = crate::Reg<u32, _FRGMULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRGMULT;
#[doc = "`read()` method returns [frgmult::R](frgmult::R) reader structure"]
impl crate::Readable for FRGMULT {}
#[doc = "`write(|w| ..)` method takes [frgmult::W](frgmult::W) writer structure"]
impl crate::Writable for FRGMULT {}
#[doc = "fractional generator N multiplier value register"]
pub mod frgmult;
#[doc = "FRG N clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frgclksel](frgclksel) module"]
pub type FRGCLKSEL = crate::Reg<u32, _FRGCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRGCLKSEL;
#[doc = "`read()` method returns [frgclksel::R](frgclksel::R) reader structure"]
impl crate::Readable for FRGCLKSEL {}
#[doc = "`write(|w| ..)` method takes [frgclksel::W](frgclksel::W) writer structure"]
impl crate::Writable for FRGCLKSEL {}
#[doc = "FRG N clock source select register"]
pub mod frgclksel;
