#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Voltage ladder register"]
    pub lad: LAD,
}
#[doc = "Comparator control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Comparator control register"]
pub mod ctrl;
#[doc = "Voltage ladder register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lad](lad) module"]
pub type LAD = crate::Reg<u32, _LAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAD;
#[doc = "`read()` method returns [lad::R](lad::R) reader structure"]
impl crate::Readable for LAD {}
#[doc = "`write(|w| ..)` method takes [lad::W](lad::W) writer structure"]
impl crate::Writable for LAD {}
#[doc = "Voltage ladder register"]
pub mod lad;
