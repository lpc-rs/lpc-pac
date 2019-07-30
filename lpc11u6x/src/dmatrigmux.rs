#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trigger input select register for DMA channel 0."]
    pub dma_itrig_pinmux: [DMA_ITRIG_PINMUX; 16],
}
#[doc = "Trigger input select register for DMA channel 0."]
pub struct DMA_ITRIG_PINMUX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Trigger input select register for DMA channel 0."]
pub mod dma_itrig_pinmux;
