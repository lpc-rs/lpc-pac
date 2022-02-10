///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SysTick Control and Status Register
    pub csr: crate::Reg<csr::CSR_SPEC>,
    ///0x04 - SysTick Reload Value Register
    pub rvr: crate::Reg<rvr::RVR_SPEC>,
    ///0x08 - SysTick Current Value Register
    pub cvr: crate::Reg<cvr::CVR_SPEC>,
    ///0x0c - SysTick Calibration Value Register
    pub calib: crate::Reg<calib::CALIB_SPEC>,
}
///CSR register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///SysTick Control and Status Register
pub mod csr;
///RVR register accessor: an alias for `Reg<RVR_SPEC>`
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
///SysTick Reload Value Register
pub mod rvr;
///CVR register accessor: an alias for `Reg<CVR_SPEC>`
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
///SysTick Current Value Register
pub mod cvr;
///CALIB register accessor: an alias for `Reg<CALIB_SPEC>`
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
///SysTick Calibration Value Register
pub mod calib;
