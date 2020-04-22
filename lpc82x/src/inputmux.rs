#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trigger select register for DMA channel"]
    pub dma_itrig_inmux: [DMA_ITRIG_INMUX; 18],
    _reserved1: [u8; 16312usize],
    #[doc = "0x4000 - DMA output trigger selection to become DMA trigger 7 and DMA trigger 8"]
    pub dma_inmux_inmux: [DMA_INMUX_INMUX; 2],
    _reserved2: [u8; 24usize],
    #[doc = "0x4020 - input select register for SCT"]
    pub sct0_inmux: [SCT0_INMUX; 4],
}
#[doc = "Trigger select register for DMA channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_itrig_inmux](dma_itrig_inmux) module"]
pub type DMA_ITRIG_INMUX = crate::Reg<u32, _DMA_ITRIG_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ITRIG_INMUX;
#[doc = "`read()` method returns [dma_itrig_inmux::R](dma_itrig_inmux::R) reader structure"]
impl crate::Readable for DMA_ITRIG_INMUX {}
#[doc = "`write(|w| ..)` method takes [dma_itrig_inmux::W](dma_itrig_inmux::W) writer structure"]
impl crate::Writable for DMA_ITRIG_INMUX {}
#[doc = "Trigger select register for DMA channel"]
pub mod dma_itrig_inmux;
#[doc = "DMA output trigger selection to become DMA trigger 7 and DMA trigger 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_inmux_inmux](dma_inmux_inmux) module"]
pub type DMA_INMUX_INMUX = crate::Reg<u32, _DMA_INMUX_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INMUX_INMUX;
#[doc = "`read()` method returns [dma_inmux_inmux::R](dma_inmux_inmux::R) reader structure"]
impl crate::Readable for DMA_INMUX_INMUX {}
#[doc = "`write(|w| ..)` method takes [dma_inmux_inmux::W](dma_inmux_inmux::W) writer structure"]
impl crate::Writable for DMA_INMUX_INMUX {}
#[doc = "DMA output trigger selection to become DMA trigger 7 and DMA trigger 8"]
pub mod dma_inmux_inmux;
#[doc = "input select register for SCT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct0_inmux](sct0_inmux) module"]
pub type SCT0_INMUX = crate::Reg<u32, _SCT0_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCT0_INMUX;
#[doc = "`read()` method returns [sct0_inmux::R](sct0_inmux::R) reader structure"]
impl crate::Readable for SCT0_INMUX {}
#[doc = "`write(|w| ..)` method takes [sct0_inmux::W](sct0_inmux::W) writer structure"]
impl crate::Writable for SCT0_INMUX {}
#[doc = "input select register for SCT"]
pub mod sct0_inmux;
