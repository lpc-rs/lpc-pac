#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests."]
    pub dma_itrig_inmux: [DMA_ITRIG_INMUX; 18],
}
#[doc = "Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests."]
pub struct DMA_ITRIG_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests."]
pub mod dma_itrig_inmux;
