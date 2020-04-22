#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w_](w_) module"]
pub type W_ = crate::Reg<u32, _W_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W_;
#[doc = "`read()` method returns [w_::R](w_::R) reader structure"]
impl crate::Readable for W_ {}
#[doc = "`write(|w| ..)` method takes [w_::W](w_::W) writer structure"]
impl crate::Writable for W_ {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w_;
