///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Pin Interrupt Mode register
    pub isel: crate::Reg<isel::ISEL_SPEC>,
    ///0x04 - Pin interrupt level or rising edge interrupt enable register
    pub ienr: crate::Reg<ienr::IENR_SPEC>,
    ///0x08 - Pin interrupt level or rising edge interrupt set register
    pub sienr: crate::Reg<sienr::SIENR_SPEC>,
    ///0x0c - Pin interrupt level (rising edge interrupt) clear register
    pub cienr: crate::Reg<cienr::CIENR_SPEC>,
    ///0x10 - Pin interrupt active level or falling edge interrupt enable register
    pub ienf: crate::Reg<ienf::IENF_SPEC>,
    ///0x14 - Pin interrupt active level or falling edge interrupt set register
    pub sienf: crate::Reg<sienf::SIENF_SPEC>,
    ///0x18 - Pin interrupt active level or falling edge interrupt clear register
    pub cienf: crate::Reg<cienf::CIENF_SPEC>,
    ///0x1c - Pin interrupt rising edge register
    pub rise: crate::Reg<rise::RISE_SPEC>,
    ///0x20 - Pin interrupt falling edge register
    pub fall: crate::Reg<fall::FALL_SPEC>,
    ///0x24 - Pin interrupt status register
    pub ist: crate::Reg<ist::IST_SPEC>,
    ///0x28 - Pattern match interrupt control register
    pub pmctrl: crate::Reg<pmctrl::PMCTRL_SPEC>,
    ///0x2c - Pattern match interrupt bit-slice source register
    pub pmsrc: crate::Reg<pmsrc::PMSRC_SPEC>,
    ///0x30 - Pattern match interrupt bit slice configuration register
    pub pmcfg: crate::Reg<pmcfg::PMCFG_SPEC>,
}
///ISEL register accessor: an alias for `Reg<ISEL_SPEC>`
pub type ISEL = crate::Reg<isel::ISEL_SPEC>;
///Pin Interrupt Mode register
pub mod isel;
///IENR register accessor: an alias for `Reg<IENR_SPEC>`
pub type IENR = crate::Reg<ienr::IENR_SPEC>;
///Pin interrupt level or rising edge interrupt enable register
pub mod ienr;
///SIENR register accessor: an alias for `Reg<SIENR_SPEC>`
pub type SIENR = crate::Reg<sienr::SIENR_SPEC>;
///Pin interrupt level or rising edge interrupt set register
pub mod sienr;
///CIENR register accessor: an alias for `Reg<CIENR_SPEC>`
pub type CIENR = crate::Reg<cienr::CIENR_SPEC>;
///Pin interrupt level (rising edge interrupt) clear register
pub mod cienr;
///IENF register accessor: an alias for `Reg<IENF_SPEC>`
pub type IENF = crate::Reg<ienf::IENF_SPEC>;
///Pin interrupt active level or falling edge interrupt enable register
pub mod ienf;
///SIENF register accessor: an alias for `Reg<SIENF_SPEC>`
pub type SIENF = crate::Reg<sienf::SIENF_SPEC>;
///Pin interrupt active level or falling edge interrupt set register
pub mod sienf;
///CIENF register accessor: an alias for `Reg<CIENF_SPEC>`
pub type CIENF = crate::Reg<cienf::CIENF_SPEC>;
///Pin interrupt active level or falling edge interrupt clear register
pub mod cienf;
///RISE register accessor: an alias for `Reg<RISE_SPEC>`
pub type RISE = crate::Reg<rise::RISE_SPEC>;
///Pin interrupt rising edge register
pub mod rise;
///FALL register accessor: an alias for `Reg<FALL_SPEC>`
pub type FALL = crate::Reg<fall::FALL_SPEC>;
///Pin interrupt falling edge register
pub mod fall;
///IST register accessor: an alias for `Reg<IST_SPEC>`
pub type IST = crate::Reg<ist::IST_SPEC>;
///Pin interrupt status register
pub mod ist;
///PMCTRL register accessor: an alias for `Reg<PMCTRL_SPEC>`
pub type PMCTRL = crate::Reg<pmctrl::PMCTRL_SPEC>;
///Pattern match interrupt control register
pub mod pmctrl;
///PMSRC register accessor: an alias for `Reg<PMSRC_SPEC>`
pub type PMSRC = crate::Reg<pmsrc::PMSRC_SPEC>;
///Pattern match interrupt bit-slice source register
pub mod pmsrc;
///PMCFG register accessor: an alias for `Reg<PMCFG_SPEC>`
pub type PMCFG = crate::Reg<pmcfg::PMCFG_SPEC>;
///Pattern match interrupt bit slice configuration register
pub mod pmcfg;
