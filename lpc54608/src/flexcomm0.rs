#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4088usize],
    #[doc = "0xff8 - Peripheral Select and Flexcomm ID register."]
    pub pselid: crate::Reg<pselid::PSELID_SPEC>,
    #[doc = "0xffc - Peripheral identification register."]
    pub pid: crate::Reg<pid::PID_SPEC>,
}
#[doc = "PSELID register accessor: an alias for `Reg<PSELID_SPEC>`"]
pub type PSELID = crate::Reg<pselid::PSELID_SPEC>;
#[doc = "Peripheral Select and Flexcomm ID register."]
pub mod pselid;
#[doc = "PID register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Peripheral identification register."]
pub mod pid;
