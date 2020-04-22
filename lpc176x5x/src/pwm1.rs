#[doc = r"Register block"]
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
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
pub mod ir;
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions."]
pub mod tcr;
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc](tc) module"]
pub type TC = crate::Reg<u32, _TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC;
#[doc = "`read()` method returns [tc::R](tc::R) reader structure"]
impl crate::Readable for TC {}
#[doc = "`write(|w| ..)` method takes [tc::W](tc::W) writer structure"]
impl crate::Writable for TC {}
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
pub mod tc;
#[doc = "Prescale Register. Determines how often the PWM counter is incremented.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "`write(|w| ..)` method takes [pr::W](pr::W) writer structure"]
impl crate::Writable for PR {}
#[doc = "Prescale Register. Determines how often the PWM counter is incremented."]
pub mod pr;
#[doc = "Prescale Counter. Prescaler for the main PWM counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "`write(|w| ..)` method takes [pc::W](pc::W) writer structure"]
impl crate::Writable for PC {}
#[doc = "Prescale Counter. Prescaler for the main PWM counter."]
pub mod pc;
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
pub mod mcr;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr;
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
pub mod ccr;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod cr;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod pcr;
#[doc = "Load Enable Register. Enables use of updated PWM match values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ler](ler) module"]
pub type LER = crate::Reg<u32, _LER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LER;
#[doc = "`read()` method returns [ler::R](ler::R) reader structure"]
impl crate::Readable for LER {}
#[doc = "`write(|w| ..)` method takes [ler::W](ler::W) writer structure"]
impl crate::Writable for LER {}
#[doc = "Load Enable Register. Enables use of updated PWM match values."]
pub mod ler;
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctcr](ctcr) module"]
pub type CTCR = crate::Reg<u32, _CTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTCR;
#[doc = "`read()` method returns [ctcr::R](ctcr::R) reader structure"]
impl crate::Readable for CTCR {}
#[doc = "`write(|w| ..)` method takes [ctcr::W](ctcr::W) writer structure"]
impl crate::Writable for CTCR {}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod ctcr;
