#[doc = "MTL TxQx Operation Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_op_mode](mtl_txqx_op_mode) module"]
pub type MTL_TXQX_OP_MODE = crate::Reg<u32, _MTL_TXQX_OP_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_OP_MODE;
#[doc = "`read()` method returns [mtl_txqx_op_mode::R](mtl_txqx_op_mode::R) reader structure"]
impl crate::Readable for MTL_TXQX_OP_MODE {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_op_mode::W](mtl_txqx_op_mode::W) writer structure"]
impl crate::Writable for MTL_TXQX_OP_MODE {}
#[doc = "MTL TxQx Operation Mode register"]
pub mod mtl_txqx_op_mode;
#[doc = "MTL TxQx Underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_undrflw](mtl_txqx_undrflw) module"]
pub type MTL_TXQX_UNDRFLW = crate::Reg<u32, _MTL_TXQX_UNDRFLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_UNDRFLW;
#[doc = "`read()` method returns [mtl_txqx_undrflw::R](mtl_txqx_undrflw::R) reader structure"]
impl crate::Readable for MTL_TXQX_UNDRFLW {}
#[doc = "MTL TxQx Underflow register"]
pub mod mtl_txqx_undrflw;
#[doc = "MTL TxQx Debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_dbg](mtl_txqx_dbg) module"]
pub type MTL_TXQX_DBG = crate::Reg<u32, _MTL_TXQX_DBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_DBG;
#[doc = "`read()` method returns [mtl_txqx_dbg::R](mtl_txqx_dbg::R) reader structure"]
impl crate::Readable for MTL_TXQX_DBG {}
#[doc = "MTL TxQx Debug register"]
pub mod mtl_txqx_dbg;
#[doc = "MTL TxQx ETS control register, only TxQ1 support\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_ets_ctrl](mtl_txqx_ets_ctrl) module"]
pub type MTL_TXQX_ETS_CTRL = crate::Reg<u32, _MTL_TXQX_ETS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_ETS_CTRL;
#[doc = "`read()` method returns [mtl_txqx_ets_ctrl::R](mtl_txqx_ets_ctrl::R) reader structure"]
impl crate::Readable for MTL_TXQX_ETS_CTRL {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_ets_ctrl::W](mtl_txqx_ets_ctrl::W) writer structure"]
impl crate::Writable for MTL_TXQX_ETS_CTRL {}
#[doc = "MTL TxQx ETS control register, only TxQ1 support"]
pub mod mtl_txqx_ets_ctrl;
#[doc = "MTL TxQx ETS Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_ets_stat](mtl_txqx_ets_stat) module"]
pub type MTL_TXQX_ETS_STAT = crate::Reg<u32, _MTL_TXQX_ETS_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_ETS_STAT;
#[doc = "`read()` method returns [mtl_txqx_ets_stat::R](mtl_txqx_ets_stat::R) reader structure"]
impl crate::Readable for MTL_TXQX_ETS_STAT {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_ets_stat::W](mtl_txqx_ets_stat::W) writer structure"]
impl crate::Writable for MTL_TXQX_ETS_STAT {}
#[doc = "MTL TxQx ETS Status register"]
pub mod mtl_txqx_ets_stat;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_qntm_wght](mtl_txqx_qntm_wght) module"]
pub type MTL_TXQX_QNTM_WGHT = crate::Reg<u32, _MTL_TXQX_QNTM_WGHT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_QNTM_WGHT;
#[doc = "`read()` method returns [mtl_txqx_qntm_wght::R](mtl_txqx_qntm_wght::R) reader structure"]
impl crate::Readable for MTL_TXQX_QNTM_WGHT {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_qntm_wght::W](mtl_txqx_qntm_wght::W) writer structure"]
impl crate::Writable for MTL_TXQX_QNTM_WGHT {}
#[doc = "no description available"]
pub mod mtl_txqx_qntm_wght;
#[doc = "MTL TxQx SendSlopCredit register, only TxQ1 support\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_sndslp_crdt](mtl_txqx_sndslp_crdt) module"]
pub type MTL_TXQX_SNDSLP_CRDT = crate::Reg<u32, _MTL_TXQX_SNDSLP_CRDT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_SNDSLP_CRDT;
#[doc = "`read()` method returns [mtl_txqx_sndslp_crdt::R](mtl_txqx_sndslp_crdt::R) reader structure"]
impl crate::Readable for MTL_TXQX_SNDSLP_CRDT {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_sndslp_crdt::W](mtl_txqx_sndslp_crdt::W) writer structure"]
impl crate::Writable for MTL_TXQX_SNDSLP_CRDT {}
#[doc = "MTL TxQx SendSlopCredit register, only TxQ1 support"]
pub mod mtl_txqx_sndslp_crdt;
#[doc = "MTL TxQx hiCredit register, only TxQ1 support\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_hi_crdt](mtl_txqx_hi_crdt) module"]
pub type MTL_TXQX_HI_CRDT = crate::Reg<u32, _MTL_TXQX_HI_CRDT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_HI_CRDT;
#[doc = "`read()` method returns [mtl_txqx_hi_crdt::R](mtl_txqx_hi_crdt::R) reader structure"]
impl crate::Readable for MTL_TXQX_HI_CRDT {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_hi_crdt::W](mtl_txqx_hi_crdt::W) writer structure"]
impl crate::Writable for MTL_TXQX_HI_CRDT {}
#[doc = "MTL TxQx hiCredit register, only TxQ1 support"]
pub mod mtl_txqx_hi_crdt;
#[doc = "MTL TxQx loCredit register, only TxQ1 support\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_lo_crdt](mtl_txqx_lo_crdt) module"]
pub type MTL_TXQX_LO_CRDT = crate::Reg<u32, _MTL_TXQX_LO_CRDT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_LO_CRDT;
#[doc = "`read()` method returns [mtl_txqx_lo_crdt::R](mtl_txqx_lo_crdt::R) reader structure"]
impl crate::Readable for MTL_TXQX_LO_CRDT {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_lo_crdt::W](mtl_txqx_lo_crdt::W) writer structure"]
impl crate::Writable for MTL_TXQX_LO_CRDT {}
#[doc = "MTL TxQx loCredit register, only TxQ1 support"]
pub mod mtl_txqx_lo_crdt;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_intctrl_stat](mtl_txqx_intctrl_stat) module"]
pub type MTL_TXQX_INTCTRL_STAT = crate::Reg<u32, _MTL_TXQX_INTCTRL_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_TXQX_INTCTRL_STAT;
#[doc = "`read()` method returns [mtl_txqx_intctrl_stat::R](mtl_txqx_intctrl_stat::R) reader structure"]
impl crate::Readable for MTL_TXQX_INTCTRL_STAT {}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_intctrl_stat::W](mtl_txqx_intctrl_stat::W) writer structure"]
impl crate::Writable for MTL_TXQX_INTCTRL_STAT {}
#[doc = "no description available"]
pub mod mtl_txqx_intctrl_stat;
#[doc = "MTL RxQx Operation Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_op_mode](mtl_rxqx_op_mode) module"]
pub type MTL_RXQX_OP_MODE = crate::Reg<u32, _MTL_RXQX_OP_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_RXQX_OP_MODE;
#[doc = "`read()` method returns [mtl_rxqx_op_mode::R](mtl_rxqx_op_mode::R) reader structure"]
impl crate::Readable for MTL_RXQX_OP_MODE {}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_op_mode::W](mtl_rxqx_op_mode::W) writer structure"]
impl crate::Writable for MTL_RXQX_OP_MODE {}
#[doc = "MTL RxQx Operation Mode register"]
pub mod mtl_rxqx_op_mode;
#[doc = "MTL RxQx Missed Packet Overflow Counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_misspkt_ovrflw_cnt](mtl_rxqx_misspkt_ovrflw_cnt) module"]
pub type MTL_RXQX_MISSPKT_OVRFLW_CNT = crate::Reg<u32, _MTL_RXQX_MISSPKT_OVRFLW_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_RXQX_MISSPKT_OVRFLW_CNT;
#[doc = "`read()` method returns [mtl_rxqx_misspkt_ovrflw_cnt::R](mtl_rxqx_misspkt_ovrflw_cnt::R) reader structure"]
impl crate::Readable for MTL_RXQX_MISSPKT_OVRFLW_CNT {}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_misspkt_ovrflw_cnt::W](mtl_rxqx_misspkt_ovrflw_cnt::W) writer structure"]
impl crate::Writable for MTL_RXQX_MISSPKT_OVRFLW_CNT {}
#[doc = "MTL RxQx Missed Packet Overflow Counter register"]
pub mod mtl_rxqx_misspkt_ovrflw_cnt;
#[doc = "MTL RxQx Debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_dbg](mtl_rxqx_dbg) module"]
pub type MTL_RXQX_DBG = crate::Reg<u32, _MTL_RXQX_DBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_RXQX_DBG;
#[doc = "`read()` method returns [mtl_rxqx_dbg::R](mtl_rxqx_dbg::R) reader structure"]
impl crate::Readable for MTL_RXQX_DBG {}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_dbg::W](mtl_rxqx_dbg::W) writer structure"]
impl crate::Writable for MTL_RXQX_DBG {}
#[doc = "MTL RxQx Debug register"]
pub mod mtl_rxqx_dbg;
#[doc = "MTL RxQx Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_ctrl](mtl_rxqx_ctrl) module"]
pub type MTL_RXQX_CTRL = crate::Reg<u32, _MTL_RXQX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_RXQX_CTRL;
#[doc = "`read()` method returns [mtl_rxqx_ctrl::R](mtl_rxqx_ctrl::R) reader structure"]
impl crate::Readable for MTL_RXQX_CTRL {}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_ctrl::W](mtl_rxqx_ctrl::W) writer structure"]
impl crate::Writable for MTL_RXQX_CTRL {}
#[doc = "MTL RxQx Control register"]
pub mod mtl_rxqx_ctrl;
