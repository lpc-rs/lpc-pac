#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x48 - Trigger select register for DMA channel"]
    pub dma_itrig_inmux: [crate::Reg<dma_itrig_inmux::DMA_ITRIG_INMUX_SPEC>; 18],
    _reserved1: [u8; 0x3fb8],
    #[doc = "0x4000..0x4008 - DMA output trigger selection to become DMA trigger 7 and DMA trigger 8"]
    pub dma_inmux_inmux: [crate::Reg<dma_inmux_inmux::DMA_INMUX_INMUX_SPEC>; 2],
    _reserved2: [u8; 0x18],
    #[doc = "0x4020..0x4030 - input select register for SCT"]
    pub sct0_inmux: [crate::Reg<sct0_inmux::SCT0_INMUX_SPEC>; 4],
}
#[doc = "DMA_ITRIG_INMUX register accessor: an alias for `Reg<DMA_ITRIG_INMUX_SPEC>`"]
pub type DMA_ITRIG_INMUX = crate::Reg<dma_itrig_inmux::DMA_ITRIG_INMUX_SPEC>;
#[doc = "Trigger select register for DMA channel"]
pub mod dma_itrig_inmux;
#[doc = "DMA_INMUX_INMUX register accessor: an alias for `Reg<DMA_INMUX_INMUX_SPEC>`"]
pub type DMA_INMUX_INMUX = crate::Reg<dma_inmux_inmux::DMA_INMUX_INMUX_SPEC>;
#[doc = "DMA output trigger selection to become DMA trigger 7 and DMA trigger 8"]
pub mod dma_inmux_inmux;
#[doc = "SCT0_INMUX register accessor: an alias for `Reg<SCT0_INMUX_SPEC>`"]
pub type SCT0_INMUX = crate::Reg<sct0_inmux::SCT0_INMUX_SPEC>;
#[doc = "input select register for SCT"]
pub mod sct0_inmux;
