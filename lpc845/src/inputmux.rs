#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - DMA output trigger selection to become DMA trigger"]
    pub dma_inmux_inmux: [crate::Reg<dma_inmux_inmux::DMA_INMUX_INMUX_SPEC>; 2],
    _reserved1: [u8; 0x18],
    #[doc = "0x20..0x30 - input select register for SCT"]
    pub sct_inmux: [crate::Reg<sct_inmux::SCT_INMUX_SPEC>; 4],
    _reserved2: [u8; 0x10],
    #[doc = "0x40..0xa4 - Trigger select register for DMA channel"]
    pub dma_itrig_inmux: [crate::Reg<dma_itrig_inmux::DMA_ITRIG_INMUX_SPEC>; 25],
}
#[doc = "DMA_INMUX_INMUX register accessor: an alias for `Reg<DMA_INMUX_INMUX_SPEC>`"]
pub type DMA_INMUX_INMUX = crate::Reg<dma_inmux_inmux::DMA_INMUX_INMUX_SPEC>;
#[doc = "DMA output trigger selection to become DMA trigger"]
pub mod dma_inmux_inmux;
#[doc = "SCT_INMUX register accessor: an alias for `Reg<SCT_INMUX_SPEC>`"]
pub type SCT_INMUX = crate::Reg<sct_inmux::SCT_INMUX_SPEC>;
#[doc = "input select register for SCT"]
pub mod sct_inmux;
#[doc = "DMA_ITRIG_INMUX register accessor: an alias for `Reg<DMA_ITRIG_INMUX_SPEC>`"]
pub type DMA_ITRIG_INMUX = crate::Reg<dma_itrig_inmux::DMA_ITRIG_INMUX_SPEC>;
#[doc = "Trigger select register for DMA channel"]
pub mod dma_itrig_inmux;
