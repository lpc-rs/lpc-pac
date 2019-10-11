#[doc = "SCT output 0 set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [set](set) module"]
pub type SET = crate::Reg<u32, _SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET;
#[doc = "`read()` method returns [set::R](set::R) reader structure"]
impl crate::Readable for SET {}
#[doc = "`write(|w| ..)` method takes [set::W](set::W) writer structure"]
impl crate::Writable for SET {}
#[doc = "SCT output 0 set register"]
pub mod set;
#[doc = "SCT output 0 clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr](clr) module"]
pub type CLR = crate::Reg<u32, _CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR;
#[doc = "`read()` method returns [clr::R](clr::R) reader structure"]
impl crate::Readable for CLR {}
#[doc = "`write(|w| ..)` method takes [clr::W](clr::W) writer structure"]
impl crate::Writable for CLR {}
#[doc = "SCT output 0 clear register"]
pub mod clr;
