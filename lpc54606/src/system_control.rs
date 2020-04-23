#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Auxiliary Control Register,"]
    pub actlr: ACTLR,
    _reserved1: [u8; 3316usize],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status Registers"]
    pub cfsr: CFSR,
    #[doc = "0xd2c - HardFault Status register"]
    pub hfsr: HFSR,
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: DFSR,
    #[doc = "0xd34 - MemManage Address Register"]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: BFAR,
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    pub afsr: AFSR,
    _reserved17: [u8; 72usize],
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: CPACR,
    _reserved18: [u8; 424usize],
    #[doc = "0xf34 - Floating-point Context Control Register"]
    pub fpccr: FPCCR,
    #[doc = "0xf38 - Floating-point Context Address Register"]
    pub fpcar: FPCAR,
    #[doc = "0xf3c - Floating-point Default Status Control Register"]
    pub fpdscr: FPDSCR,
}
#[doc = "Auxiliary Control Register,\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](actlr) module"]
pub type ACTLR = crate::Reg<u32, _ACTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTLR;
#[doc = "`read()` method returns [actlr::R](actlr::R) reader structure"]
impl crate::Readable for ACTLR {}
#[doc = "`write(|w| ..)` method takes [actlr::W](actlr::W) writer structure"]
impl crate::Writable for ACTLR {}
#[doc = "Auxiliary Control Register,"]
pub mod actlr;
#[doc = "CPUID Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](icsr) module"]
pub type ICSR = crate::Reg<u32, _ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSR;
#[doc = "`read()` method returns [icsr::R](icsr::R) reader structure"]
impl crate::Readable for ICSR {}
#[doc = "`write(|w| ..)` method takes [icsr::W](icsr::W) writer structure"]
impl crate::Writable for ICSR {}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Vector Table Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtor](vtor) module"]
pub type VTOR = crate::Reg<u32, _VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTOR;
#[doc = "`read()` method returns [vtor::R](vtor::R) reader structure"]
impl crate::Readable for VTOR {}
#[doc = "`write(|w| ..)` method takes [vtor::W](vtor::W) writer structure"]
impl crate::Writable for VTOR {}
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](aircr) module"]
pub type AIRCR = crate::Reg<u32, _AIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIRCR;
#[doc = "`read()` method returns [aircr::R](aircr::R) reader structure"]
impl crate::Readable for AIRCR {}
#[doc = "`write(|w| ..)` method takes [aircr::W](aircr::W) writer structure"]
impl crate::Writable for AIRCR {}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr1](shpr1) module"]
pub type SHPR1 = crate::Reg<u32, _SHPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR1;
#[doc = "`read()` method returns [shpr1::R](shpr1::R) reader structure"]
impl crate::Readable for SHPR1 {}
#[doc = "`write(|w| ..)` method takes [shpr1::W](shpr1::W) writer structure"]
impl crate::Writable for SHPR1 {}
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "System Handler Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr2](shpr2) module"]
pub type SHPR2 = crate::Reg<u32, _SHPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR2;
#[doc = "`read()` method returns [shpr2::R](shpr2::R) reader structure"]
impl crate::Readable for SHPR2 {}
#[doc = "`write(|w| ..)` method takes [shpr2::W](shpr2::W) writer structure"]
impl crate::Writable for SHPR2 {}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr3](shpr3) module"]
pub type SHPR3 = crate::Reg<u32, _SHPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR3;
#[doc = "`read()` method returns [shpr3::R](shpr3::R) reader structure"]
impl crate::Readable for SHPR3 {}
#[doc = "`write(|w| ..)` method takes [shpr3::W](shpr3::W) writer structure"]
impl crate::Writable for SHPR3 {}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](shcsr) module"]
pub type SHCSR = crate::Reg<u32, _SHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHCSR;
#[doc = "`read()` method returns [shcsr::R](shcsr::R) reader structure"]
impl crate::Readable for SHCSR {}
#[doc = "`write(|w| ..)` method takes [shcsr::W](shcsr::W) writer structure"]
impl crate::Writable for SHCSR {}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "Configurable Fault Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](cfsr) module"]
pub type CFSR = crate::Reg<u32, _CFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFSR;
#[doc = "`read()` method returns [cfsr::R](cfsr::R) reader structure"]
impl crate::Readable for CFSR {}
#[doc = "`write(|w| ..)` method takes [cfsr::W](cfsr::W) writer structure"]
impl crate::Writable for CFSR {}
#[doc = "Configurable Fault Status Registers"]
pub mod cfsr;
#[doc = "HardFault Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfsr](hfsr) module"]
pub type HFSR = crate::Reg<u32, _HFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFSR;
#[doc = "`read()` method returns [hfsr::R](hfsr::R) reader structure"]
impl crate::Readable for HFSR {}
#[doc = "`write(|w| ..)` method takes [hfsr::W](hfsr::W) writer structure"]
impl crate::Writable for HFSR {}
#[doc = "HardFault Status register"]
pub mod hfsr;
#[doc = "Debug Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](dfsr) module"]
pub type DFSR = crate::Reg<u32, _DFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSR;
#[doc = "`read()` method returns [dfsr::R](dfsr::R) reader structure"]
impl crate::Readable for DFSR {}
#[doc = "`write(|w| ..)` method takes [dfsr::W](dfsr::W) writer structure"]
impl crate::Writable for DFSR {}
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MemManage Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfar](mmfar) module"]
pub type MMFAR = crate::Reg<u32, _MMFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMFAR;
#[doc = "`read()` method returns [mmfar::R](mmfar::R) reader structure"]
impl crate::Readable for MMFAR {}
#[doc = "`write(|w| ..)` method takes [mmfar::W](mmfar::W) writer structure"]
impl crate::Writable for MMFAR {}
#[doc = "MemManage Address Register"]
pub mod mmfar;
#[doc = "BusFault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfar](bfar) module"]
pub type BFAR = crate::Reg<u32, _BFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFAR;
#[doc = "`read()` method returns [bfar::R](bfar::R) reader structure"]
impl crate::Readable for BFAR {}
#[doc = "`write(|w| ..)` method takes [bfar::W](bfar::W) writer structure"]
impl crate::Writable for BFAR {}
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "Auxiliary Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsr](afsr) module"]
pub type AFSR = crate::Reg<u32, _AFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSR;
#[doc = "`read()` method returns [afsr::R](afsr::R) reader structure"]
impl crate::Readable for AFSR {}
#[doc = "`write(|w| ..)` method takes [afsr::W](afsr::W) writer structure"]
impl crate::Writable for AFSR {}
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "Coprocessor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpacr](cpacr) module"]
pub type CPACR = crate::Reg<u32, _CPACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPACR;
#[doc = "`read()` method returns [cpacr::R](cpacr::R) reader structure"]
impl crate::Readable for CPACR {}
#[doc = "`write(|w| ..)` method takes [cpacr::W](cpacr::W) writer structure"]
impl crate::Writable for CPACR {}
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "Floating-point Context Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpccr](fpccr) module"]
pub type FPCCR = crate::Reg<u32, _FPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCCR;
#[doc = "`read()` method returns [fpccr::R](fpccr::R) reader structure"]
impl crate::Readable for FPCCR {}
#[doc = "`write(|w| ..)` method takes [fpccr::W](fpccr::W) writer structure"]
impl crate::Writable for FPCCR {}
#[doc = "Floating-point Context Control Register"]
pub mod fpccr;
#[doc = "Floating-point Context Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcar](fpcar) module"]
pub type FPCAR = crate::Reg<u32, _FPCAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCAR;
#[doc = "`read()` method returns [fpcar::R](fpcar::R) reader structure"]
impl crate::Readable for FPCAR {}
#[doc = "`write(|w| ..)` method takes [fpcar::W](fpcar::W) writer structure"]
impl crate::Writable for FPCAR {}
#[doc = "Floating-point Context Address Register"]
pub mod fpcar;
#[doc = "Floating-point Default Status Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpdscr](fpdscr) module"]
pub type FPDSCR = crate::Reg<u32, _FPDSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPDSCR;
#[doc = "`read()` method returns [fpdscr::R](fpdscr::R) reader structure"]
impl crate::Readable for FPDSCR {}
#[doc = "`write(|w| ..)` method takes [fpdscr::W](fpdscr::W) writer structure"]
impl crate::Writable for FPDSCR {}
#[doc = "Floating-point Default Status Control Register"]
pub mod fpdscr;
