#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input mux register for DMA trigger input 20. Selects from 18 DMA trigger outputs."]
    pub dma_inmux_inmux: [DMA_INMUX_INMUX; 2],
    _reserved0: [u8; 24usize],
    #[doc = "0x20 - Input mux register for SCT input 0"]
    pub sct0_inmux: [SCT0_INMUX; 4],
}
#[doc = "Input mux register for DMA trigger input 20. Selects from 18 DMA trigger outputs."]
pub struct DMA_INMUX_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input mux register for DMA trigger input 20. Selects from 18 DMA trigger outputs."]
pub mod dma_inmux_inmux;
#[doc = "Input mux register for SCT input 0"]
pub struct SCT0_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input mux register for SCT input 0"]
pub mod sct0_inmux;
