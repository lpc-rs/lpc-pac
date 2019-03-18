#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA output trigger selection to become DMA trigger"]
    pub dma_inmux_inmux: [DMA_INMUX_INMUX; 2],
    _reserved0: [u8; 24usize],
    #[doc = "0x20 - input select register for SCT"]
    pub sct_inmux: [SCT_INMUX; 4],
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Trigger select register for DMA channel"]
    pub dma_itrig_inmux: [DMA_ITRIG_INMUX; 25],
}
#[doc = "DMA output trigger selection to become DMA trigger"]
pub struct DMA_INMUX_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA output trigger selection to become DMA trigger"]
pub mod dma_inmux_inmux;
#[doc = "input select register for SCT"]
pub struct SCT_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "input select register for SCT"]
pub mod sct_inmux;
#[doc = "Trigger select register for DMA channel"]
pub struct DMA_ITRIG_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger select register for DMA channel"]
pub mod dma_itrig_inmux;
