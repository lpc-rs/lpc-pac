#[doc = "Configuration for EMC_CSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticconfig](staticconfig) module"]
pub type STATICCONFIG = crate::Reg<u32, _STATICCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICCONFIG;
#[doc = "`read()` method returns [staticconfig::R](staticconfig::R) reader structure"]
impl crate::Readable for STATICCONFIG {}
#[doc = "`write(|w| ..)` method takes [staticconfig::W](staticconfig::W) writer structure"]
impl crate::Writable for STATICCONFIG {}
#[doc = "Configuration for EMC_CSx"]
pub mod staticconfig;
#[doc = "Delay from EMC_CSx to write enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitwen](staticwaitwen) module"]
pub type STATICWAITWEN = crate::Reg<u32, _STATICWAITWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITWEN;
#[doc = "`read()` method returns [staticwaitwen::R](staticwaitwen::R) reader structure"]
impl crate::Readable for STATICWAITWEN {}
#[doc = "`write(|w| ..)` method takes [staticwaitwen::W](staticwaitwen::W) writer structure"]
impl crate::Writable for STATICWAITWEN {}
#[doc = "Delay from EMC_CSx to write enable"]
pub mod staticwaitwen;
#[doc = "Delay from EMC_CSx or address change, whichever is later, to output enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitoen](staticwaitoen) module"]
pub type STATICWAITOEN = crate::Reg<u32, _STATICWAITOEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITOEN;
#[doc = "`read()` method returns [staticwaitoen::R](staticwaitoen::R) reader structure"]
impl crate::Readable for STATICWAITOEN {}
#[doc = "`write(|w| ..)` method takes [staticwaitoen::W](staticwaitoen::W) writer structure"]
impl crate::Writable for STATICWAITOEN {}
#[doc = "Delay from EMC_CSx or address change, whichever is later, to output enable"]
pub mod staticwaitoen;
#[doc = "Delay from EMC_CSx to a read access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitrd](staticwaitrd) module"]
pub type STATICWAITRD = crate::Reg<u32, _STATICWAITRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITRD;
#[doc = "`read()` method returns [staticwaitrd::R](staticwaitrd::R) reader structure"]
impl crate::Readable for STATICWAITRD {}
#[doc = "`write(|w| ..)` method takes [staticwaitrd::W](staticwaitrd::W) writer structure"]
impl crate::Writable for STATICWAITRD {}
#[doc = "Delay from EMC_CSx to a read access"]
pub mod staticwaitrd;
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitpage](staticwaitpage) module"]
pub type STATICWAITPAGE = crate::Reg<u32, _STATICWAITPAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITPAGE;
#[doc = "`read()` method returns [staticwaitpage::R](staticwaitpage::R) reader structure"]
impl crate::Readable for STATICWAITPAGE {}
#[doc = "`write(|w| ..)` method takes [staticwaitpage::W](staticwaitpage::W) writer structure"]
impl crate::Writable for STATICWAITPAGE {}
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CSx"]
pub mod staticwaitpage;
#[doc = "Delay from EMC_CSx to a write access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitwr](staticwaitwr) module"]
pub type STATICWAITWR = crate::Reg<u32, _STATICWAITWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITWR;
#[doc = "`read()` method returns [staticwaitwr::R](staticwaitwr::R) reader structure"]
impl crate::Readable for STATICWAITWR {}
#[doc = "`write(|w| ..)` method takes [staticwaitwr::W](staticwaitwr::W) writer structure"]
impl crate::Writable for STATICWAITWR {}
#[doc = "Delay from EMC_CSx to a write access"]
pub mod staticwaitwr;
#[doc = "Number of bus turnaround cycles EMC_CSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitturn](staticwaitturn) module"]
pub type STATICWAITTURN = crate::Reg<u32, _STATICWAITTURN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITTURN;
#[doc = "`read()` method returns [staticwaitturn::R](staticwaitturn::R) reader structure"]
impl crate::Readable for STATICWAITTURN {}
#[doc = "`write(|w| ..)` method takes [staticwaitturn::W](staticwaitturn::W) writer structure"]
impl crate::Writable for STATICWAITTURN {}
#[doc = "Number of bus turnaround cycles EMC_CSx"]
pub mod staticwaitturn;
