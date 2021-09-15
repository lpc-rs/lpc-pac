#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending."]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of the APB bus clock. The TC is controlled through the TCR."]
    pub tc: crate::Reg<tc::TC_SPEC>,
    #[doc = "0x0c - Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC."]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x10 - Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface."]
    pub pc: crate::Reg<pc::PC_SPEC>,
    #[doc = "0x14 - Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs."]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x18..0x28 - Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC."]
    pub mr: [crate::Reg<mr::MR_SPEC>; 4],
    #[doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x2c..0x3c - Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input."]
    pub cr: [crate::Reg<cr::CR_SPEC>; 4],
    #[doc = "0x3c - External Match Register. The EMR controls the match function and the external match pins."]
    pub emr: crate::Reg<emr::EMR_SPEC>,
    _reserved10: [u8; 0x30],
    #[doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
    pub ctcr: crate::Reg<ctcr::CTCR_SPEC>,
    #[doc = "0x74 - PWM Control Register. The PWMCON enables PWM mode for the external match pins."]
    pub pwmc: crate::Reg<pwmc::PWMC_SPEC>,
    #[doc = "0x78..0x88 - Match Shadow Register . If enabled, the Match Register will be automatically reloaded with the contents of this register whenever the TC is reset to zero."]
    pub msr: [crate::Reg<msr::MSR_SPEC>; 4],
}
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending."]
pub mod ir;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
pub mod tcr;
#[doc = "TC register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of the APB bus clock. The TC is controlled through the TCR."]
pub mod tc;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC."]
pub mod pr;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface."]
pub mod pc;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs."]
pub mod mcr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC."]
pub mod mr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
pub mod ccr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input."]
pub mod cr;
#[doc = "EMR register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "External Match Register. The EMR controls the match function and the external match pins."]
pub mod emr;
#[doc = "CTCR register accessor: an alias for `Reg<CTCR_SPEC>`"]
pub type CTCR = crate::Reg<ctcr::CTCR_SPEC>;
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod ctcr;
#[doc = "PWMC register accessor: an alias for `Reg<PWMC_SPEC>`"]
pub type PWMC = crate::Reg<pwmc::PWMC_SPEC>;
#[doc = "PWM Control Register. The PWMCON enables PWM mode for the external match pins."]
pub mod pwmc;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Match Shadow Register . If enabled, the Match Register will be automatically reloaded with the contents of this register whenever the TC is reset to zero."]
pub mod msr;
