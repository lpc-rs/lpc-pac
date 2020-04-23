#[doc = "Configuration information for EMC_DYCSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicconfig](dynamicconfig) module"]
pub type DYNAMICCONFIG = crate::Reg<u32, _DYNAMICCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICCONFIG;
#[doc = "`read()` method returns [dynamicconfig::R](dynamicconfig::R) reader structure"]
impl crate::Readable for DYNAMICCONFIG {}
#[doc = "`write(|w| ..)` method takes [dynamicconfig::W](dynamicconfig::W) writer structure"]
impl crate::Writable for DYNAMICCONFIG {}
#[doc = "Configuration information for EMC_DYCSx"]
pub mod dynamicconfig;
#[doc = "RAS and CAS latencies for EMC_DYCSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrascas](dynamicrascas) module"]
pub type DYNAMICRASCAS = crate::Reg<u32, _DYNAMICRASCAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRASCAS;
#[doc = "`read()` method returns [dynamicrascas::R](dynamicrascas::R) reader structure"]
impl crate::Readable for DYNAMICRASCAS {}
#[doc = "`write(|w| ..)` method takes [dynamicrascas::W](dynamicrascas::W) writer structure"]
impl crate::Writable for DYNAMICRASCAS {}
#[doc = "RAS and CAS latencies for EMC_DYCSx"]
pub mod dynamicrascas;
