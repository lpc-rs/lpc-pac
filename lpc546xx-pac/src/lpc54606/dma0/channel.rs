///CFG register accessor: an alias for `Reg<CFG_SPEC>`
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
///Configuration register for DMA channel .
pub mod cfg;
///CTLSTAT register accessor: an alias for `Reg<CTLSTAT_SPEC>`
pub type CTLSTAT = crate::Reg<ctlstat::CTLSTAT_SPEC>;
///Control and status register for DMA channel .
pub mod ctlstat;
///XFERCFG register accessor: an alias for `Reg<XFERCFG_SPEC>`
pub type XFERCFG = crate::Reg<xfercfg::XFERCFG_SPEC>;
///Transfer configuration register for DMA channel .
pub mod xfercfg;
