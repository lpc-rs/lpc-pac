#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration register for DMA channel ."]
pub mod cfg;
#[doc = "CTLSTAT register accessor: an alias for `Reg<CTLSTAT_SPEC>`"]
pub type CTLSTAT = crate::Reg<ctlstat::CTLSTAT_SPEC>;
#[doc = "Control and status register for DMA channel ."]
pub mod ctlstat;
#[doc = "XFERCFG register accessor: an alias for `Reg<XFERCFG_SPEC>`"]
pub type XFERCFG = crate::Reg<xfercfg::XFERCFG_SPEC>;
#[doc = "Transfer configuration register for DMA channel ."]
pub mod xfercfg;
