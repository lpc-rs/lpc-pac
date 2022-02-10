///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending.
    pub ir: crate::Reg<ir::IR_SPEC>,
    ///0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    ///0x08 - Timer Counter
    pub tc: crate::Reg<tc::TC_SPEC>,
    ///0x0c - Prescale Register
    pub pr: crate::Reg<pr::PR_SPEC>,
    ///0x10 - Prescale Counter
    pub pc: crate::Reg<pc::PC_SPEC>,
    ///0x14 - Match Control Register
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    ///0x18..0x28 - Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC.
    pub mr: [crate::Reg<mr::MR_SPEC>; 4],
    ///0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    ///0x2c..0x3c - Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input.
    pub cr: [crate::Reg<cr::CR_SPEC>; 4],
    ///0x3c - External Match Register. The EMR controls the match function and the external match pins.
    pub emr: crate::Reg<emr::EMR_SPEC>,
    _reserved10: [u8; 0x30],
    ///0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.
    pub ctcr: crate::Reg<ctcr::CTCR_SPEC>,
    ///0x74 - PWM Control Register. The PWMCON enables PWM mode for the external match pins.
    pub pwmc: crate::Reg<pwmc::PWMC_SPEC>,
    ///0x78..0x88 - Match Shadow Register
    pub msr: [crate::Reg<msr::MSR_SPEC>; 4],
}
///IR register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending.
pub mod ir;
///TCR register accessor: an alias for `Reg<TCR_SPEC>`
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
///Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.
pub mod tcr;
///TC register accessor: an alias for `Reg<TC_SPEC>`
pub type TC = crate::Reg<tc::TC_SPEC>;
///Timer Counter
pub mod tc;
///PR register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///Prescale Register
pub mod pr;
///PC register accessor: an alias for `Reg<PC_SPEC>`
pub type PC = crate::Reg<pc::PC_SPEC>;
///Prescale Counter
pub mod pc;
///MCR register accessor: an alias for `Reg<MCR_SPEC>`
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
///Match Control Register
pub mod mcr;
///MR register accessor: an alias for `Reg<MR_SPEC>`
pub type MR = crate::Reg<mr::MR_SPEC>;
///Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC.
pub mod mr;
///CCR register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.
pub mod ccr;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input.
pub mod cr;
///EMR register accessor: an alias for `Reg<EMR_SPEC>`
pub type EMR = crate::Reg<emr::EMR_SPEC>;
///External Match Register. The EMR controls the match function and the external match pins.
pub mod emr;
///CTCR register accessor: an alias for `Reg<CTCR_SPEC>`
pub type CTCR = crate::Reg<ctcr::CTCR_SPEC>;
///Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.
pub mod ctcr;
///PWMC register accessor: an alias for `Reg<PWMC_SPEC>`
pub type PWMC = crate::Reg<pwmc::PWMC_SPEC>;
///PWM Control Register. The PWMCON enables PWM mode for the external match pins.
pub mod pwmc;
///MSR register accessor: an alias for `Reg<MSR_SPEC>`
pub type MSR = crate::Reg<msr::MSR_SPEC>;
///Match Shadow Register
pub mod msr;
