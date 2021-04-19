#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POSITION Register"]
    pub position: crate::Reg<position::POSITION_SPEC>,
    #[doc = "0x04 - MASTER Register"]
    pub master: crate::Reg<master::MASTER_SPEC>,
    #[doc = "0x08 - FLOW Register"]
    pub flow: crate::Reg<flow::FLOW_SPEC>,
    #[doc = "0x0c - Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
    pub base: crate::Reg<base::BASE_SPEC>,
}
#[doc = "POSITION register accessor: an alias for `Reg<POSITION_SPEC>`"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "POSITION Register"]
pub mod position;
#[doc = "MASTER register accessor: an alias for `Reg<MASTER_SPEC>`"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MASTER Register"]
pub mod master;
#[doc = "FLOW register accessor: an alias for `Reg<FLOW_SPEC>`"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "FLOW Register"]
pub mod flow;
#[doc = "BASE register accessor: an alias for `Reg<BASE_SPEC>`"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent."]
pub mod base;
