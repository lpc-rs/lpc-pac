#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
    pub cr: CR,
    #[doc = "0x04 - DAC Control register. This register controls DMA and timer operation."]
    pub ctrl: CTRL,
    #[doc = "0x08 - DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
    pub cntval: CNTVAL,
}
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
pub mod cr;
#[doc = "DAC Control register. This register controls DMA and timer operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "DAC Control register. This register controls DMA and timer operation."]
pub mod ctrl;
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntval](cntval) module"]
pub type CNTVAL = crate::Reg<u32, _CNTVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTVAL;
#[doc = "`read()` method returns [cntval::R](cntval::R) reader structure"]
impl crate::Readable for CNTVAL {}
#[doc = "`write(|w| ..)` method takes [cntval::W](cntval::W) writer structure"]
impl crate::Writable for CNTVAL {}
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
pub mod cntval;
