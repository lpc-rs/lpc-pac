#[doc = "Configuration register 1 for channel pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg1](pcfg1) module"]
pub type PCFG1 = crate::Reg<u32, _PCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCFG1;
#[doc = "`read()` method returns [pcfg1::R](pcfg1::R) reader structure"]
impl crate::Readable for PCFG1 {}
#[doc = "`write(|w| ..)` method takes [pcfg1::W](pcfg1::W) writer structure"]
impl crate::Writable for PCFG1 {}
#[doc = "Configuration register 1 for channel pair"]
pub mod pcfg1;
#[doc = "Configuration register 2 for channel pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg2](pcfg2) module"]
pub type PCFG2 = crate::Reg<u32, _PCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCFG2;
#[doc = "`read()` method returns [pcfg2::R](pcfg2::R) reader structure"]
impl crate::Readable for PCFG2 {}
#[doc = "`write(|w| ..)` method takes [pcfg2::W](pcfg2::W) writer structure"]
impl crate::Writable for PCFG2 {}
#[doc = "Configuration register 2 for channel pair"]
pub mod pcfg2;
#[doc = "Status register for channel pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstat](pstat) module"]
pub type PSTAT = crate::Reg<u32, _PSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTAT;
#[doc = "`read()` method returns [pstat::R](pstat::R) reader structure"]
impl crate::Readable for PSTAT {}
#[doc = "`write(|w| ..)` method takes [pstat::W](pstat::W) writer structure"]
impl crate::Writable for PSTAT {}
#[doc = "Status register for channel pair"]
pub mod pstat;
