#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POSITION Register"]
    pub position: POSITION,
    #[doc = "0x04 - MASTER Register"]
    pub master: MASTER,
    #[doc = "0x08 - FLOW Register"]
    pub flow: FLOW,
    #[doc = "0x0c - Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
    pub base: BASE,
}
#[doc = "POSITION Register"]
pub struct POSITION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSITION Register"]
pub mod position;
#[doc = "MASTER Register"]
pub struct MASTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MASTER Register"]
pub mod master;
#[doc = "FLOW Register"]
pub struct FLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FLOW Register"]
pub mod flow;
#[doc = "Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
pub struct BASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
pub mod base;
