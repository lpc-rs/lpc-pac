#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Auxiliary Control Register,"]
    pub actlr: crate::Reg<actlr::ACTLR_SPEC>,
    _reserved1: [u8; 0x0cf4],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: crate::Reg<cpuid::CPUID_SPEC>,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: crate::Reg<vtor::VTOR_SPEC>,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: crate::Reg<aircr::AIRCR_SPEC>,
    #[doc = "0xd10 - System Control Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: crate::Reg<shpr1::SHPR1_SPEC>,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: crate::Reg<shpr2::SHPR2_SPEC>,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: crate::Reg<shpr3::SHPR3_SPEC>,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: crate::Reg<shcsr::SHCSR_SPEC>,
    #[doc = "0xd28 - Configurable Fault Status Registers"]
    pub cfsr: crate::Reg<cfsr::CFSR_SPEC>,
    #[doc = "0xd2c - HardFault Status register"]
    pub hfsr: crate::Reg<hfsr::HFSR_SPEC>,
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: crate::Reg<dfsr::DFSR_SPEC>,
    #[doc = "0xd34 - MemManage Address Register"]
    pub mmfar: crate::Reg<mmfar::MMFAR_SPEC>,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: crate::Reg<bfar::BFAR_SPEC>,
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    pub afsr: crate::Reg<afsr::AFSR_SPEC>,
    _reserved17: [u8; 0x48],
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: crate::Reg<cpacr::CPACR_SPEC>,
    _reserved18: [u8; 0x01a8],
    #[doc = "0xf34 - Floating-point Context Control Register"]
    pub fpccr: crate::Reg<fpccr::FPCCR_SPEC>,
    #[doc = "0xf38 - Floating-point Context Address Register"]
    pub fpcar: crate::Reg<fpcar::FPCAR_SPEC>,
    #[doc = "0xf3c - Floating-point Default Status Control Register"]
    pub fpdscr: crate::Reg<fpdscr::FPDSCR_SPEC>,
}
#[doc = "ACTLR register accessor: an alias for `Reg<ACTLR_SPEC>`"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Auxiliary Control Register,"]
pub mod actlr;
#[doc = "CPUID register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "ICSR register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "VTOR register accessor: an alias for `Reg<VTOR_SPEC>`"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "AIRCR register accessor: an alias for `Reg<AIRCR_SPEC>`"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "SHPR1 register accessor: an alias for `Reg<SHPR1_SPEC>`"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "SHPR2 register accessor: an alias for `Reg<SHPR2_SPEC>`"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 register accessor: an alias for `Reg<SHPR3_SPEC>`"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "SHCSR register accessor: an alias for `Reg<SHCSR_SPEC>`"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "CFSR register accessor: an alias for `Reg<CFSR_SPEC>`"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "Configurable Fault Status Registers"]
pub mod cfsr;
#[doc = "HFSR register accessor: an alias for `Reg<HFSR_SPEC>`"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "HardFault Status register"]
pub mod hfsr;
#[doc = "DFSR register accessor: an alias for `Reg<DFSR_SPEC>`"]
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MMFAR register accessor: an alias for `Reg<MMFAR_SPEC>`"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "MemManage Address Register"]
pub mod mmfar;
#[doc = "BFAR register accessor: an alias for `Reg<BFAR_SPEC>`"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "AFSR register accessor: an alias for `Reg<AFSR_SPEC>`"]
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "CPACR register accessor: an alias for `Reg<CPACR_SPEC>`"]
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "FPCCR register accessor: an alias for `Reg<FPCCR_SPEC>`"]
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
#[doc = "Floating-point Context Control Register"]
pub mod fpccr;
#[doc = "FPCAR register accessor: an alias for `Reg<FPCAR_SPEC>`"]
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
#[doc = "Floating-point Context Address Register"]
pub mod fpcar;
#[doc = "FPDSCR register accessor: an alias for `Reg<FPDSCR_SPEC>`"]
pub type FPDSCR = crate::Reg<fpdscr::FPDSCR_SPEC>;
#[doc = "Floating-point Default Status Control Register"]
pub mod fpdscr;
