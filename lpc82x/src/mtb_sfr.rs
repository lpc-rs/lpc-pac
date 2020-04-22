#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POSITION Register"]
    pub position: POSITION,
    #[doc = "0x04 - MASTER Register"]
    pub master: MASTER,
    #[doc = "0x08 - FLOW Register"]
    pub flow: FLOW,
    #[doc = "0x0c - Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
    pub base: BASE,
}
#[doc = "POSITION Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [position](position) module"]
pub type POSITION = crate::Reg<u32, _POSITION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POSITION;
#[doc = "`read()` method returns [position::R](position::R) reader structure"]
impl crate::Readable for POSITION {}
#[doc = "`write(|w| ..)` method takes [position::W](position::W) writer structure"]
impl crate::Writable for POSITION {}
#[doc = "POSITION Register"]
pub mod position;
#[doc = "MASTER Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master](master) module"]
pub type MASTER = crate::Reg<u32, _MASTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASTER;
#[doc = "`read()` method returns [master::R](master::R) reader structure"]
impl crate::Readable for MASTER {}
#[doc = "`write(|w| ..)` method takes [master::W](master::W) writer structure"]
impl crate::Writable for MASTER {}
#[doc = "MASTER Register"]
pub mod master;
#[doc = "FLOW Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow](flow) module"]
pub type FLOW = crate::Reg<u32, _FLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW;
#[doc = "`read()` method returns [flow::R](flow::R) reader structure"]
impl crate::Readable for FLOW {}
#[doc = "`write(|w| ..)` method takes [flow::W](flow::W) writer structure"]
impl crate::Writable for FLOW {}
#[doc = "FLOW Register"]
pub mod flow;
#[doc = "Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base](base) module"]
pub type BASE = crate::Reg<u32, _BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE;
#[doc = "`read()` method returns [base::R](base::R) reader structure"]
impl crate::Readable for BASE {}
#[doc = "Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
pub mod base;
