#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - DAC Control register. This register controls DMA and timer operation."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
    pub cntval: crate::Reg<cntval::CNTVAL_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
pub mod cr;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DAC Control register. This register controls DMA and timer operation."]
pub mod ctrl;
#[doc = "CNTVAL register accessor: an alias for `Reg<CNTVAL_SPEC>`"]
pub type CNTVAL = crate::Reg<cntval::CNTVAL_SPEC>;
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
pub mod cntval;
