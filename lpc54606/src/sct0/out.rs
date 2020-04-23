#[doc = "SCT output 0 set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_set](out_set) module"]
pub type OUT_SET = crate::Reg<u32, _OUT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_SET;
#[doc = "`read()` method returns [out_set::R](out_set::R) reader structure"]
impl crate::Readable for OUT_SET {}
#[doc = "`write(|w| ..)` method takes [out_set::W](out_set::W) writer structure"]
impl crate::Writable for OUT_SET {}
#[doc = "SCT output 0 set register"]
pub mod out_set;
#[doc = "SCT output 0 clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_clr](out_clr) module"]
pub type OUT_CLR = crate::Reg<u32, _OUT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CLR;
#[doc = "`read()` method returns [out_clr::R](out_clr::R) reader structure"]
impl crate::Readable for OUT_CLR {}
#[doc = "`write(|w| ..)` method takes [out_clr::W](out_clr::W) writer structure"]
impl crate::Writable for OUT_CLR {}
#[doc = "SCT output 0 clear register"]
pub mod out_clr;
