#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare value LSB register"]
    pub compval: COMPVAL,
    #[doc = "0x04 - Mask LSB register"]
    pub mask: MASK,
    #[doc = "0x08 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Counter LSB register"]
    pub counter: COUNTER,
    #[doc = "0x10 - Compare value MSB register"]
    pub compval_h: COMPVAL_H,
    #[doc = "0x14 - Mask MSB register"]
    pub mask_h: MASK_H,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Counter MSB register"]
    pub counter_h: COUNTER_H,
}
#[doc = "Compare value LSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compval](compval) module"]
pub type COMPVAL = crate::Reg<u32, _COMPVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPVAL;
#[doc = "`read()` method returns [compval::R](compval::R) reader structure"]
impl crate::Readable for COMPVAL {}
#[doc = "`write(|w| ..)` method takes [compval::W](compval::W) writer structure"]
impl crate::Writable for COMPVAL {}
#[doc = "Compare value LSB register"]
pub mod compval;
#[doc = "Mask LSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "Mask LSB register"]
pub mod mask;
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control register"]
pub mod ctrl;
#[doc = "Counter LSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter](counter) module"]
pub type COUNTER = crate::Reg<u32, _COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER;
#[doc = "`read()` method returns [counter::R](counter::R) reader structure"]
impl crate::Readable for COUNTER {}
#[doc = "`write(|w| ..)` method takes [counter::W](counter::W) writer structure"]
impl crate::Writable for COUNTER {}
#[doc = "Counter LSB register"]
pub mod counter;
#[doc = "Compare value MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compval_h](compval_h) module"]
pub type COMPVAL_H = crate::Reg<u32, _COMPVAL_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPVAL_H;
#[doc = "`read()` method returns [compval_h::R](compval_h::R) reader structure"]
impl crate::Readable for COMPVAL_H {}
#[doc = "`write(|w| ..)` method takes [compval_h::W](compval_h::W) writer structure"]
impl crate::Writable for COMPVAL_H {}
#[doc = "Compare value MSB register"]
pub mod compval_h;
#[doc = "Mask MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_h](mask_h) module"]
pub type MASK_H = crate::Reg<u32, _MASK_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK_H;
#[doc = "`read()` method returns [mask_h::R](mask_h::R) reader structure"]
impl crate::Readable for MASK_H {}
#[doc = "`write(|w| ..)` method takes [mask_h::W](mask_h::W) writer structure"]
impl crate::Writable for MASK_H {}
#[doc = "Mask MSB register"]
pub mod mask_h;
#[doc = "Counter MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter_h](counter_h) module"]
pub type COUNTER_H = crate::Reg<u32, _COUNTER_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER_H;
#[doc = "`read()` method returns [counter_h::R](counter_h::R) reader structure"]
impl crate::Readable for COUNTER_H {}
#[doc = "`write(|w| ..)` method takes [counter_h::W](counter_h::W) writer structure"]
impl crate::Writable for COUNTER_H {}
#[doc = "Counter MSB register"]
pub mod counter_h;
