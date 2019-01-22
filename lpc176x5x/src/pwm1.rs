#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
    pub ir: IR,
    #[doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions."]
    pub tcr: TCR,
    #[doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
    pub tc: TC,
    #[doc = "0x0c - Prescale Register. Determines how often the PWM counter is incremented."]
    pub pr: PR,
    #[doc = "0x10 - Prescale Counter. Prescaler for the main PWM counter."]
    pub pc: PC,
    #[doc = "0x14 - Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
    pub mcr: MCR,
    #[doc = "0x18 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr: [MR; 4],
    #[doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
    pub ccr: CCR,
    #[doc = "0x2c - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    pub cr: [CR; 2],
    _reserved9: [u8; 12usize],
    #[doc = "0x40 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr4: MR,
    #[doc = "0x44 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr5: MR,
    #[doc = "0x48 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    pub mr6: MR,
    #[doc = "0x4c - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    pub pcr: PCR,
    #[doc = "0x50 - Load Enable Register. Enables use of updated PWM match values."]
    pub ler: LER,
    _reserved14: [u8; 28usize],
    #[doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
    pub ctcr: CTCR,
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
pub mod ir;
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions."]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions."]
pub mod tcr;
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
pub struct TC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
pub mod tc;
#[doc = "Prescale Register. Determines how often the PWM counter is incremented."]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescale Register. Determines how often the PWM counter is incremented."]
pub mod pr;
#[doc = "Prescale Counter. Prescaler for the main PWM counter."]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescale Counter. Prescaler for the main PWM counter."]
pub mod pc;
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
pub mod mcr;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr;
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
pub mod ccr;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod cr;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub struct PCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod pcr;
#[doc = "Load Enable Register. Enables use of updated PWM match values."]
pub struct LER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Load Enable Register. Enables use of updated PWM match values."]
pub mod ler;
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub struct CTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod ctcr;
