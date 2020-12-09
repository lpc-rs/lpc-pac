#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trigger select register for DMA channel"]
    pub sct0_inmux: [SCT0_INMUX; 7],
    _reserved1: [u8; 164usize],
    #[doc = "0xc0 - Pin interrupt select register"]
    pub pintsel: [PINTSEL; 8],
    #[doc = "0xe0 - Trigger select register for DMA channel"]
    pub dma_itrig_inmux: [DMA_ITRIG_INMUX; 30],
    _reserved3: [u8; 8usize],
    #[doc = "0x160 - DMA output trigger selection to become DMA trigger"]
    pub dma_otrig_inmux: [DMA_OTRIG_INMUX; 4],
    _reserved4: [u8; 16usize],
    #[doc = "0x180 - Selection for frequency measurement reference clock"]
    pub freqmeas_ref: FREQMEAS_REF,
    #[doc = "0x184 - Selection for frequency measurement target clock"]
    pub freqmeas_target: FREQMEAS_TARGET,
}
#[doc = "Trigger select register for DMA channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct0_inmux](sct0_inmux) module"]
pub type SCT0_INMUX = crate::Reg<u32, _SCT0_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCT0_INMUX;
#[doc = "`read()` method returns [sct0_inmux::R](sct0_inmux::R) reader structure"]
impl crate::Readable for SCT0_INMUX {}
#[doc = "`write(|w| ..)` method takes [sct0_inmux::W](sct0_inmux::W) writer structure"]
impl crate::Writable for SCT0_INMUX {}
#[doc = "Trigger select register for DMA channel"]
pub mod sct0_inmux;
#[doc = "Pin interrupt select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsel](pintsel) module"]
pub type PINTSEL = crate::Reg<u32, _PINTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTSEL;
#[doc = "`read()` method returns [pintsel::R](pintsel::R) reader structure"]
impl crate::Readable for PINTSEL {}
#[doc = "`write(|w| ..)` method takes [pintsel::W](pintsel::W) writer structure"]
impl crate::Writable for PINTSEL {}
#[doc = "Pin interrupt select register"]
pub mod pintsel;
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
#[doc = "DMA output trigger selection to become DMA trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_otrig_inmux](dma_otrig_inmux) module"]
pub type DMA_OTRIG_INMUX = crate::Reg<u32, _DMA_OTRIG_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_OTRIG_INMUX;
#[doc = "`read()` method returns [dma_otrig_inmux::R](dma_otrig_inmux::R) reader structure"]
impl crate::Readable for DMA_OTRIG_INMUX {}
#[doc = "`write(|w| ..)` method takes [dma_otrig_inmux::W](dma_otrig_inmux::W) writer structure"]
impl crate::Writable for DMA_OTRIG_INMUX {}
#[doc = "DMA output trigger selection to become DMA trigger"]
pub mod dma_otrig_inmux;
#[doc = "Selection for frequency measurement reference clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmeas_ref](freqmeas_ref) module"]
pub type FREQMEAS_REF = crate::Reg<u32, _FREQMEAS_REF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQMEAS_REF;
#[doc = "`read()` method returns [freqmeas_ref::R](freqmeas_ref::R) reader structure"]
impl crate::Readable for FREQMEAS_REF {}
#[doc = "`write(|w| ..)` method takes [freqmeas_ref::W](freqmeas_ref::W) writer structure"]
impl crate::Writable for FREQMEAS_REF {}
#[doc = "Selection for frequency measurement reference clock"]
pub mod freqmeas_ref;
#[doc = "Selection for frequency measurement target clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmeas_target](freqmeas_target) module"]
pub type FREQMEAS_TARGET = crate::Reg<u32, _FREQMEAS_TARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQMEAS_TARGET;
#[doc = "`read()` method returns [freqmeas_target::R](freqmeas_target::R) reader structure"]
impl crate::Readable for FREQMEAS_TARGET {}
#[doc = "`write(|w| ..)` method takes [freqmeas_target::W](freqmeas_target::W) writer structure"]
impl crate::Writable for FREQMEAS_TARGET {}
#[doc = "Selection for frequency measurement target clock"]
pub mod freqmeas_target;
