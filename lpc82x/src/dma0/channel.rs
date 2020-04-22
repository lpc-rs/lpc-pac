#[doc = "Configuration register for DMA channel .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration register for DMA channel ."]
pub mod cfg;
#[doc = "Control and status register for DMA channel .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlstat](ctlstat) module"]
pub type CTLSTAT = crate::Reg<u32, _CTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLSTAT;
#[doc = "`read()` method returns [ctlstat::R](ctlstat::R) reader structure"]
impl crate::Readable for CTLSTAT {}
#[doc = "Control and status register for DMA channel ."]
pub mod ctlstat;
#[doc = "Transfer configuration register for DMA channel .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xfercfg](xfercfg) module"]
pub type XFERCFG = crate::Reg<u32, _XFERCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XFERCFG;
#[doc = "`read()` method returns [xfercfg::R](xfercfg::R) reader structure"]
impl crate::Readable for XFERCFG {}
#[doc = "`write(|w| ..)` method takes [xfercfg::W](xfercfg::W) writer structure"]
impl crate::Writable for XFERCFG {}
#[doc = "Transfer configuration register for DMA channel ."]
pub mod xfercfg;
