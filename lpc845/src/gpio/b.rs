#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b_](b_) module"]
pub type B_ = crate::Reg<u8, _B_>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B_;
#[doc = "`read()` method returns [b_::R](b_::R) reader structure"]
impl crate::Readable for B_ {}
#[doc = "`write(|w| ..)` method takes [b_::W](b_::W) writer structure"]
impl crate::Writable for B_ {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b_;
