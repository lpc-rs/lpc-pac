///MTL_TXQx_OP_MODE register accessor: an alias for `Reg<MTL_TXQX_OP_MODE_SPEC>`
pub type MTL_TXQX_OP_MODE = crate::Reg<mtl_txqx_op_mode::MTL_TXQX_OP_MODE_SPEC>;
///MTL TxQx Operation Mode register
pub mod mtl_txqx_op_mode;
///MTL_TXQx_UNDRFLW register accessor: an alias for `Reg<MTL_TXQX_UNDRFLW_SPEC>`
pub type MTL_TXQX_UNDRFLW = crate::Reg<mtl_txqx_undrflw::MTL_TXQX_UNDRFLW_SPEC>;
///MTL TxQx Underflow register
pub mod mtl_txqx_undrflw;
///MTL_TXQx_DBG register accessor: an alias for `Reg<MTL_TXQX_DBG_SPEC>`
pub type MTL_TXQX_DBG = crate::Reg<mtl_txqx_dbg::MTL_TXQX_DBG_SPEC>;
///MTL TxQx Debug register
pub mod mtl_txqx_dbg;
///MTL_TXQx_ETS_CTRL register accessor: an alias for `Reg<MTL_TXQX_ETS_CTRL_SPEC>`
pub type MTL_TXQX_ETS_CTRL = crate::Reg<mtl_txqx_ets_ctrl::MTL_TXQX_ETS_CTRL_SPEC>;
///MTL TxQx ETS control register, only TxQ1 support
pub mod mtl_txqx_ets_ctrl;
///MTL_TXQx_ETS_STAT register accessor: an alias for `Reg<MTL_TXQX_ETS_STAT_SPEC>`
pub type MTL_TXQX_ETS_STAT = crate::Reg<mtl_txqx_ets_stat::MTL_TXQX_ETS_STAT_SPEC>;
///MTL TxQx ETS Status register
pub mod mtl_txqx_ets_stat;
///MTL_TXQx_QNTM_WGHT register accessor: an alias for `Reg<MTL_TXQX_QNTM_WGHT_SPEC>`
pub type MTL_TXQX_QNTM_WGHT = crate::Reg<mtl_txqx_qntm_wght::MTL_TXQX_QNTM_WGHT_SPEC>;
///no description available
pub mod mtl_txqx_qntm_wght;
///MTL_TXQx_SNDSLP_CRDT register accessor: an alias for `Reg<MTL_TXQX_SNDSLP_CRDT_SPEC>`
pub type MTL_TXQX_SNDSLP_CRDT = crate::Reg<mtl_txqx_sndslp_crdt::MTL_TXQX_SNDSLP_CRDT_SPEC>;
///MTL TxQx SendSlopCredit register, only TxQ1 support
pub mod mtl_txqx_sndslp_crdt;
///MTL_TXQx_HI_CRDT register accessor: an alias for `Reg<MTL_TXQX_HI_CRDT_SPEC>`
pub type MTL_TXQX_HI_CRDT = crate::Reg<mtl_txqx_hi_crdt::MTL_TXQX_HI_CRDT_SPEC>;
///MTL TxQx hiCredit register, only TxQ1 support
pub mod mtl_txqx_hi_crdt;
///MTL_TXQx_LO_CRDT register accessor: an alias for `Reg<MTL_TXQX_LO_CRDT_SPEC>`
pub type MTL_TXQX_LO_CRDT = crate::Reg<mtl_txqx_lo_crdt::MTL_TXQX_LO_CRDT_SPEC>;
///MTL TxQx loCredit register, only TxQ1 support
pub mod mtl_txqx_lo_crdt;
///MTL_TXQx_INTCTRL_STAT register accessor: an alias for `Reg<MTL_TXQX_INTCTRL_STAT_SPEC>`
pub type MTL_TXQX_INTCTRL_STAT = crate::Reg<mtl_txqx_intctrl_stat::MTL_TXQX_INTCTRL_STAT_SPEC>;
///no description available
pub mod mtl_txqx_intctrl_stat;
///MTL_RXQx_OP_MODE register accessor: an alias for `Reg<MTL_RXQX_OP_MODE_SPEC>`
pub type MTL_RXQX_OP_MODE = crate::Reg<mtl_rxqx_op_mode::MTL_RXQX_OP_MODE_SPEC>;
///MTL RxQx Operation Mode register
pub mod mtl_rxqx_op_mode;
///MTL_RXQx_MISSPKT_OVRFLW_CNT register accessor: an alias for `Reg<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>`
pub type MTL_RXQX_MISSPKT_OVRFLW_CNT =
    crate::Reg<mtl_rxqx_misspkt_ovrflw_cnt::MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>;
///MTL RxQx Missed Packet Overflow Counter register
pub mod mtl_rxqx_misspkt_ovrflw_cnt;
///MTL_RXQx_DBG register accessor: an alias for `Reg<MTL_RXQX_DBG_SPEC>`
pub type MTL_RXQX_DBG = crate::Reg<mtl_rxqx_dbg::MTL_RXQX_DBG_SPEC>;
///MTL RxQx Debug register
pub mod mtl_rxqx_dbg;
///MTL_RXQx_CTRL register accessor: an alias for `Reg<MTL_RXQX_CTRL_SPEC>`
pub type MTL_RXQX_CTRL = crate::Reg<mtl_rxqx_ctrl::MTL_RXQX_CTRL_SPEC>;
///MTL RxQx Control register
pub mod mtl_rxqx_ctrl;
