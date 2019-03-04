#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
    pub cr: CR,
    #[doc = "0x04 - DAC Control register. This register controls DMA and timer operation."]
    pub ctrl: CTRL,
    #[doc = "0x08 - DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
    pub cntval: CNTVAL,
}
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "D/A Converter Register. This register contains the digital value to be converted to analog and a power control bit."]
pub mod cr;
#[doc = "DAC Control register. This register controls DMA and timer operation."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Control register. This register controls DMA and timer operation."]
pub mod ctrl;
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
pub struct CNTVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer."]
pub mod cntval;
