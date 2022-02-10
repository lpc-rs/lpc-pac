///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ///0x08 - Auxiliary Control Register,
    pub actlr: crate::Reg<actlr::ACTLR_SPEC>,
    _reserved1: [u8; 0x0cf4],
    ///0xd00 - CPUID Base Register
    pub cpuid: crate::Reg<cpuid::CPUID_SPEC>,
    ///0xd04 - Interrupt Control and State Register
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    ///0xd08 - Vector Table Offset Register
    pub vtor: crate::Reg<vtor::VTOR_SPEC>,
    ///0xd0c - Application Interrupt and Reset Control Register
    pub aircr: crate::Reg<aircr::AIRCR_SPEC>,
    ///0xd10 - System Control Register
    pub scr: crate::Reg<scr::SCR_SPEC>,
    ///0xd14 - Configuration and Control Register
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    ///0xd18 - System Handler Priority Register 1
    pub shpr1: crate::Reg<shpr1::SHPR1_SPEC>,
    ///0xd1c - System Handler Priority Register 2
    pub shpr2: crate::Reg<shpr2::SHPR2_SPEC>,
    ///0xd20 - System Handler Priority Register 3
    pub shpr3: crate::Reg<shpr3::SHPR3_SPEC>,
    ///0xd24 - System Handler Control and State Register
    pub shcsr: crate::Reg<shcsr::SHCSR_SPEC>,
    ///0xd28 - Configurable Fault Status Registers
    pub cfsr: crate::Reg<cfsr::CFSR_SPEC>,
    ///0xd2c - HardFault Status register
    pub hfsr: crate::Reg<hfsr::HFSR_SPEC>,
    ///0xd30 - Debug Fault Status Register
    pub dfsr: crate::Reg<dfsr::DFSR_SPEC>,
    ///0xd34 - MemManage Address Register
    pub mmfar: crate::Reg<mmfar::MMFAR_SPEC>,
    ///0xd38 - BusFault Address Register
    pub bfar: crate::Reg<bfar::BFAR_SPEC>,
    ///0xd3c - Auxiliary Fault Status Register
    pub afsr: crate::Reg<afsr::AFSR_SPEC>,
    _reserved17: [u8; 0x48],
    ///0xd88 - Coprocessor Access Control Register
    pub cpacr: crate::Reg<cpacr::CPACR_SPEC>,
    _reserved18: [u8; 0x01a8],
    ///0xf34 - Floating-point Context Control Register
    pub fpccr: crate::Reg<fpccr::FPCCR_SPEC>,
    ///0xf38 - Floating-point Context Address Register
    pub fpcar: crate::Reg<fpcar::FPCAR_SPEC>,
    ///0xf3c - Floating-point Default Status Control Register
    pub fpdscr: crate::Reg<fpdscr::FPDSCR_SPEC>,
}
///ACTLR register accessor: an alias for `Reg<ACTLR_SPEC>`
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
///Auxiliary Control Register,
pub mod actlr;
///CPUID register accessor: an alias for `Reg<CPUID_SPEC>`
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
///CPUID Base Register
pub mod cpuid;
///ICSR register accessor: an alias for `Reg<ICSR_SPEC>`
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
///Interrupt Control and State Register
pub mod icsr;
///VTOR register accessor: an alias for `Reg<VTOR_SPEC>`
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
///Vector Table Offset Register
pub mod vtor;
///AIRCR register accessor: an alias for `Reg<AIRCR_SPEC>`
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
///Application Interrupt and Reset Control Register
pub mod aircr;
///SCR register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///System Control Register
pub mod scr;
///CCR register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///Configuration and Control Register
pub mod ccr;
///SHPR1 register accessor: an alias for `Reg<SHPR1_SPEC>`
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
///System Handler Priority Register 1
pub mod shpr1;
///SHPR2 register accessor: an alias for `Reg<SHPR2_SPEC>`
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
///System Handler Priority Register 2
pub mod shpr2;
///SHPR3 register accessor: an alias for `Reg<SHPR3_SPEC>`
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
///System Handler Priority Register 3
pub mod shpr3;
///SHCSR register accessor: an alias for `Reg<SHCSR_SPEC>`
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
///System Handler Control and State Register
pub mod shcsr;
///CFSR register accessor: an alias for `Reg<CFSR_SPEC>`
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
///Configurable Fault Status Registers
pub mod cfsr;
///HFSR register accessor: an alias for `Reg<HFSR_SPEC>`
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
///HardFault Status register
pub mod hfsr;
///DFSR register accessor: an alias for `Reg<DFSR_SPEC>`
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
///Debug Fault Status Register
pub mod dfsr;
///MMFAR register accessor: an alias for `Reg<MMFAR_SPEC>`
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
///MemManage Address Register
pub mod mmfar;
///BFAR register accessor: an alias for `Reg<BFAR_SPEC>`
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
///BusFault Address Register
pub mod bfar;
///AFSR register accessor: an alias for `Reg<AFSR_SPEC>`
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
///Auxiliary Fault Status Register
pub mod afsr;
///CPACR register accessor: an alias for `Reg<CPACR_SPEC>`
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
///Coprocessor Access Control Register
pub mod cpacr;
///FPCCR register accessor: an alias for `Reg<FPCCR_SPEC>`
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
///Floating-point Context Control Register
pub mod fpccr;
///FPCAR register accessor: an alias for `Reg<FPCAR_SPEC>`
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
///Floating-point Context Address Register
pub mod fpcar;
///FPDSCR register accessor: an alias for `Reg<FPDSCR_SPEC>`
pub type FPDSCR = crate::Reg<fpdscr::FPDSCR_SPEC>;
///Floating-point Default Status Control Register
pub mod fpdscr;
