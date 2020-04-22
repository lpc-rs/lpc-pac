#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register"]
    pub mac_config: MAC_CONFIG,
    #[doc = "0x04 - no description available"]
    pub mac_ext_config: MAC_EXT_CONFIG,
    #[doc = "0x08 - MAC frame filter register"]
    pub mac_frame_filter: MAC_FRAME_FILTER,
    #[doc = "0x0c - MAC watchdog Timeout register"]
    pub mac_wd_timerout: MAC_WD_TIMEROUT,
    _reserved4: [u8; 64usize],
    #[doc = "0x50 - MAC vlan tag register"]
    pub mac_vlan_tag: MAC_VLAN_TAG,
    _reserved5: [u8; 28usize],
    #[doc = "0x70 - Transmit flow control register"]
    pub mac_tx_flow_ctrl_q: [MAC_TX_FLOW_CTRL_Q; 2],
    _reserved6: [u8; 24usize],
    #[doc = "0x90 - Receive flow control register"]
    pub mac_rx_flow_ctrl: MAC_RX_FLOW_CTRL,
    _reserved7: [u8; 4usize],
    #[doc = "0x98 - no description available"]
    pub mac_txq_prio_map: MAC_TXQ_PRIO_MAP,
    _reserved8: [u8; 4usize],
    #[doc = "0xa0 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl0: MAC_RXQ_CTRL0,
    #[doc = "0xa4 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl1: MAC_RXQ_CTRL1,
    #[doc = "0xa8 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl2: MAC_RXQ_CTRL2,
    _reserved11: [u8; 4usize],
    #[doc = "0xb0 - Interrupt status register 0x0000"]
    pub mac_intr_stat: MAC_INTR_STAT,
    #[doc = "0xb4 - Interrupt enable register 0x0000"]
    pub mac_intr_en: MAC_INTR_EN,
    #[doc = "0xb8 - Receive Transmit Status register"]
    pub mac_rxtx_stat: MAC_RXTX_STAT,
    _reserved14: [u8; 4usize],
    #[doc = "0xc0 - no description available"]
    pub mac_pmt_crtl_stat: MAC_PMT_CRTL_STAT,
    #[doc = "0xc4 - Remote wake-up frame filter"]
    pub mac_rwake_frflt: MAC_RWAKE_FRFLT,
    _reserved16: [u8; 8usize],
    #[doc = "0xd0 - LPI Control and Status Register"]
    pub mac_lpi_ctrl_stat: MAC_LPI_CTRL_STAT,
    #[doc = "0xd4 - LPI Timers Control register"]
    pub mac_lpi_timer_ctrl: MAC_LPI_TIMER_CTRL,
    #[doc = "0xd8 - LPI entry Timer register"]
    pub mac_lpi_entr_timr: MAC_LPI_ENTR_TIMR,
    #[doc = "0xdc - no description available"]
    pub mac_1us_tic_countr: MAC_1US_TIC_COUNTR,
    _reserved20: [u8; 48usize],
    #[doc = "0x110 - MAC version register"]
    pub mac_version: MAC_VERSION,
    #[doc = "0x114 - MAC debug register"]
    pub mac_dbg: MAC_DBG,
    _reserved22: [u8; 4usize],
    #[doc = "0x11c - MAC hardware feature register 0x0201"]
    pub mac_hw_feat0: MAC_HW_FEAT0,
    #[doc = "0x120 - MAC hardware feature register 0x0201"]
    pub mac_hw_feat1: MAC_HW_FEAT1,
    #[doc = "0x124 - MAC hardware feature register 0x0201"]
    pub mac_hw_feat2: MAC_HW_FEAT2,
    _reserved25: [u8; 216usize],
    #[doc = "0x200 - MIDO address Register"]
    pub mac_mdio_addr: MAC_MDIO_ADDR,
    #[doc = "0x204 - MDIO Data register"]
    pub mac_mdio_data: MAC_MDIO_DATA,
    _reserved27: [u8; 248usize],
    #[doc = "0x300 - MAC address0 high register"]
    pub mac_addr_high: MAC_ADDR_HIGH,
    #[doc = "0x304 - MAC address0 low register"]
    pub mac_addr_low: MAC_ADDR_LOW,
    _reserved29: [u8; 2040usize],
    #[doc = "0xb00 - Time stamp control register"]
    pub mac_timestamp_ctrl: MAC_TIMESTAMP_CTRL,
    #[doc = "0xb04 - Sub-second increment register"]
    pub mac_sub_scnd_incr: MAC_SUB_SCND_INCR,
    #[doc = "0xb08 - System time seconds register"]
    pub mac_sys_time_scnd: MAC_SYS_TIME_SCND,
    #[doc = "0xb0c - System time nanoseconds register"]
    pub mac_sys_time_nscnd: MAC_SYS_TIME_NSCND,
    #[doc = "0xb10 - no description available"]
    pub mac_sys_time_scnd_upd: MAC_SYS_TIME_SCND_UPD,
    #[doc = "0xb14 - no description available"]
    pub mac_sys_time_nscnd_upd: MAC_SYS_TIME_NSCND_UPD,
    #[doc = "0xb18 - Time stamp addend register"]
    pub mac_sys_timestmp_addend: MAC_SYS_TIMESTMP_ADDEND,
    #[doc = "0xb1c - no description available"]
    pub mac_sys_time_hword_scnd: MAC_SYS_TIME_HWORD_SCND,
    #[doc = "0xb20 - Time stamp status register"]
    pub mac_sys_timestmp_stat: MAC_SYS_TIMESTMP_STAT,
    _reserved38: [u8; 12usize],
    #[doc = "0xb30 - Tx timestamp status nanoseconds"]
    pub mac_tx_timestamp_status_nanoseconds: MAC_TX_TIMESTAMP_STATUS_NANOSECONDS,
    #[doc = "0xb34 - Tx timestamp status seconds"]
    pub mac_tx_timestamp_status_seconds: MAC_TX_TIMESTAMP_STATUS_SECONDS,
    _reserved40: [u8; 32usize],
    #[doc = "0xb58 - Timestamp ingress correction"]
    pub mac_timestamp_ingress_corr_nanosecond: MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND,
    #[doc = "0xb5c - Timestamp egress correction"]
    pub mac_timestamp_egress_corr_nanosecond: MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND,
    _reserved42: [u8; 160usize],
    #[doc = "0xc00 - MTL Operation Mode Register"]
    pub mtl_op_mode: MTL_OP_MODE,
    _reserved43: [u8; 28usize],
    #[doc = "0xc20 - MTL Interrupt Status register"]
    pub mtl_intr_stat: MTL_INTR_STAT,
    _reserved44: [u8; 12usize],
    #[doc = "0xc30 - MTL Receive Queue and DMA Channel Mapping register"]
    pub mtl_rxq_dma_map: MTL_RXQ_DMA_MAP,
    _reserved45: [u8; 204usize],
    #[doc = "0xd00 - no description available"]
    pub mtl_queue: [MTL_QUEUE; 2],
    _reserved46: [u8; 640usize],
    #[doc = "0x1000 - DMA mode register"]
    pub dma_mode: DMA_MODE,
    #[doc = "0x1004 - DMA System Bus mode"]
    pub dma_sysbus_mode: DMA_SYSBUS_MODE,
    #[doc = "0x1008 - DMA Interrupt status"]
    pub dma_intr_stat: DMA_INTR_STAT,
    #[doc = "0x100c - DMA Debug Status"]
    pub dma_dbg_stat: DMA_DBG_STAT,
    _reserved50: [u8; 240usize],
    #[doc = "0x1100 - no description available"]
    pub dma_ch0: DMA_CH,
    _reserved51: [u8; 28usize],
    #[doc = "0x1180 - no description available"]
    pub dma_ch1: DMA_CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MTL_QUEUE {
    #[doc = "0x00 - MTL TxQx Operation Mode register"]
    pub mtl_txqx_op_mode: self::mtl_queue::MTL_TXQX_OP_MODE,
    #[doc = "0x04 - MTL TxQx Underflow register"]
    pub mtl_txqx_undrflw: self::mtl_queue::MTL_TXQX_UNDRFLW,
    #[doc = "0x08 - MTL TxQx Debug register"]
    pub mtl_txqx_dbg: self::mtl_queue::MTL_TXQX_DBG,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - MTL TxQx ETS control register, only TxQ1 support"]
    pub mtl_txqx_ets_ctrl: self::mtl_queue::MTL_TXQX_ETS_CTRL,
    #[doc = "0x14 - MTL TxQx ETS Status register"]
    pub mtl_txqx_ets_stat: self::mtl_queue::MTL_TXQX_ETS_STAT,
    #[doc = "0x18 - no description available"]
    pub mtl_txqx_qntm_wght: self::mtl_queue::MTL_TXQX_QNTM_WGHT,
    #[doc = "0x1c - MTL TxQx SendSlopCredit register, only TxQ1 support"]
    pub mtl_txqx_sndslp_crdt: self::mtl_queue::MTL_TXQX_SNDSLP_CRDT,
    #[doc = "0x20 - MTL TxQx hiCredit register, only TxQ1 support"]
    pub mtl_txqx_hi_crdt: self::mtl_queue::MTL_TXQX_HI_CRDT,
    #[doc = "0x24 - MTL TxQx loCredit register, only TxQ1 support"]
    pub mtl_txqx_lo_crdt: self::mtl_queue::MTL_TXQX_LO_CRDT,
    _reserved9: [u8; 4usize],
    #[doc = "0x2c - no description available"]
    pub mtl_txqx_intctrl_stat: self::mtl_queue::MTL_TXQX_INTCTRL_STAT,
    #[doc = "0x30 - MTL RxQx Operation Mode register"]
    pub mtl_rxqx_op_mode: self::mtl_queue::MTL_RXQX_OP_MODE,
    #[doc = "0x34 - MTL RxQx Missed Packet Overflow Counter register"]
    pub mtl_rxqx_misspkt_ovrflw_cnt: self::mtl_queue::MTL_RXQX_MISSPKT_OVRFLW_CNT,
    #[doc = "0x38 - MTL RxQx Debug register"]
    pub mtl_rxqx_dbg: self::mtl_queue::MTL_RXQX_DBG,
    #[doc = "0x3c - MTL RxQx Control register"]
    pub mtl_rxqx_ctrl: self::mtl_queue::MTL_RXQX_CTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod mtl_queue;
#[doc = r"Register block"]
#[repr(C)]
pub struct DMA_CH {
    #[doc = "0x00 - DMA Channelx Control"]
    pub dma_chx_ctrl: self::dma_ch::DMA_CHX_CTRL,
    #[doc = "0x04 - DMA Channelx Transmit Control"]
    pub dma_chx_tx_ctrl: self::dma_ch::DMA_CHX_TX_CTRL,
    #[doc = "0x08 - DMA Channelx Receive Control"]
    pub dma_chx_rx_ctrl: self::dma_ch::DMA_CHX_RX_CTRL,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - no description available"]
    pub dma_chx_txdesc_list_addr: self::dma_ch::DMA_CHX_TXDESC_LIST_ADDR,
    _reserved4: [u8; 4usize],
    #[doc = "0x1c - no description available"]
    pub dma_chx_rxdesc_list_addr: self::dma_ch::DMA_CHX_RXDESC_LIST_ADDR,
    #[doc = "0x20 - no description available"]
    pub dma_chx_txdesc_tail_ptr: self::dma_ch::DMA_CHX_TXDESC_TAIL_PTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x28 - no description available"]
    pub dma_chx_rxdesc_tail_ptr: self::dma_ch::DMA_CHX_RXDESC_TAIL_PTR,
    #[doc = "0x2c - no description available"]
    pub dma_chx_txdesc_ring_length: self::dma_ch::DMA_CHX_TXDESC_RING_LENGTH,
    #[doc = "0x30 - Channelx Rx descriptor Ring Length"]
    pub dma_chx_rxdesc_ring_length: self::dma_ch::DMA_CHX_RXDESC_RING_LENGTH,
    #[doc = "0x34 - Channelx Interrupt Enable"]
    pub dma_chx_int_en: self::dma_ch::DMA_CHX_INT_EN,
    #[doc = "0x38 - Receive Interrupt Watchdog Timer"]
    pub dma_chx_rx_int_wdtimer: self::dma_ch::DMA_CHX_RX_INT_WDTIMER,
    #[doc = "0x3c - Slot Function Control and Status"]
    pub dma_chx_slot_func_ctrl_stat: self::dma_ch::DMA_CHX_SLOT_FUNC_CTRL_STAT,
    _reserved12: [u8; 4usize],
    #[doc = "0x44 - Channelx Current Host Transmit descriptor"]
    pub dma_chx_cur_hst_txdesc: self::dma_ch::DMA_CHX_CUR_HST_TXDESC,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - no description available"]
    pub dma_chx_cur_hst_rxdesc: self::dma_ch::DMA_CHX_CUR_HST_RXDESC,
    _reserved14: [u8; 4usize],
    #[doc = "0x54 - no description available"]
    pub dma_chx_cur_hst_txbuf: self::dma_ch::DMA_CHX_CUR_HST_TXBUF,
    _reserved15: [u8; 4usize],
    #[doc = "0x5c - Channelx Current Application Receive Buffer Address"]
    pub dma_chx_cur_hst_rxbuf: self::dma_ch::DMA_CHX_CUR_HST_RXBUF,
    #[doc = "0x60 - Channelx DMA status register"]
    pub dma_chx_stat: self::dma_ch::DMA_CHX_STAT,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod dma_ch;
#[doc = "MAC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_config](mac_config) module"]
pub type MAC_CONFIG = crate::Reg<u32, _MAC_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_CONFIG;
#[doc = "`read()` method returns [mac_config::R](mac_config::R) reader structure"]
impl crate::Readable for MAC_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mac_config::W](mac_config::W) writer structure"]
impl crate::Writable for MAC_CONFIG {}
#[doc = "MAC configuration register"]
pub mod mac_config;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_ext_config](mac_ext_config) module"]
pub type MAC_EXT_CONFIG = crate::Reg<u32, _MAC_EXT_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_EXT_CONFIG;
#[doc = "`read()` method returns [mac_ext_config::R](mac_ext_config::R) reader structure"]
impl crate::Readable for MAC_EXT_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mac_ext_config::W](mac_ext_config::W) writer structure"]
impl crate::Writable for MAC_EXT_CONFIG {}
#[doc = "no description available"]
pub mod mac_ext_config;
#[doc = "MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_frame_filter](mac_frame_filter) module"]
pub type MAC_FRAME_FILTER = crate::Reg<u32, _MAC_FRAME_FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_FRAME_FILTER;
#[doc = "`read()` method returns [mac_frame_filter::R](mac_frame_filter::R) reader structure"]
impl crate::Readable for MAC_FRAME_FILTER {}
#[doc = "`write(|w| ..)` method takes [mac_frame_filter::W](mac_frame_filter::W) writer structure"]
impl crate::Writable for MAC_FRAME_FILTER {}
#[doc = "MAC frame filter register"]
pub mod mac_frame_filter;
#[doc = "MAC watchdog Timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_wd_timerout](mac_wd_timerout) module"]
pub type MAC_WD_TIMEROUT = crate::Reg<u32, _MAC_WD_TIMEROUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_WD_TIMEROUT;
#[doc = "`read()` method returns [mac_wd_timerout::R](mac_wd_timerout::R) reader structure"]
impl crate::Readable for MAC_WD_TIMEROUT {}
#[doc = "`write(|w| ..)` method takes [mac_wd_timerout::W](mac_wd_timerout::W) writer structure"]
impl crate::Writable for MAC_WD_TIMEROUT {}
#[doc = "MAC watchdog Timeout register"]
pub mod mac_wd_timerout;
#[doc = "MAC vlan tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_vlan_tag](mac_vlan_tag) module"]
pub type MAC_VLAN_TAG = crate::Reg<u32, _MAC_VLAN_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_VLAN_TAG;
#[doc = "`read()` method returns [mac_vlan_tag::R](mac_vlan_tag::R) reader structure"]
impl crate::Readable for MAC_VLAN_TAG {}
#[doc = "`write(|w| ..)` method takes [mac_vlan_tag::W](mac_vlan_tag::W) writer structure"]
impl crate::Writable for MAC_VLAN_TAG {}
#[doc = "MAC vlan tag register"]
pub mod mac_vlan_tag;
#[doc = "Transmit flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_flow_ctrl_q](mac_tx_flow_ctrl_q) module"]
pub type MAC_TX_FLOW_CTRL_Q = crate::Reg<u32, _MAC_TX_FLOW_CTRL_Q>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_TX_FLOW_CTRL_Q;
#[doc = "`read()` method returns [mac_tx_flow_ctrl_q::R](mac_tx_flow_ctrl_q::R) reader structure"]
impl crate::Readable for MAC_TX_FLOW_CTRL_Q {}
#[doc = "`write(|w| ..)` method takes [mac_tx_flow_ctrl_q::W](mac_tx_flow_ctrl_q::W) writer structure"]
impl crate::Writable for MAC_TX_FLOW_CTRL_Q {}
#[doc = "Transmit flow control register"]
pub mod mac_tx_flow_ctrl_q;
#[doc = "Receive flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rx_flow_ctrl](mac_rx_flow_ctrl) module"]
pub type MAC_RX_FLOW_CTRL = crate::Reg<u32, _MAC_RX_FLOW_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_RX_FLOW_CTRL;
#[doc = "`read()` method returns [mac_rx_flow_ctrl::R](mac_rx_flow_ctrl::R) reader structure"]
impl crate::Readable for MAC_RX_FLOW_CTRL {}
#[doc = "`write(|w| ..)` method takes [mac_rx_flow_ctrl::W](mac_rx_flow_ctrl::W) writer structure"]
impl crate::Writable for MAC_RX_FLOW_CTRL {}
#[doc = "Receive flow control register"]
pub mod mac_rx_flow_ctrl;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_txq_prio_map](mac_txq_prio_map) module"]
pub type MAC_TXQ_PRIO_MAP = crate::Reg<u32, _MAC_TXQ_PRIO_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_TXQ_PRIO_MAP;
#[doc = "`read()` method returns [mac_txq_prio_map::R](mac_txq_prio_map::R) reader structure"]
impl crate::Readable for MAC_TXQ_PRIO_MAP {}
#[doc = "`write(|w| ..)` method takes [mac_txq_prio_map::W](mac_txq_prio_map::W) writer structure"]
impl crate::Writable for MAC_TXQ_PRIO_MAP {}
#[doc = "no description available"]
pub mod mac_txq_prio_map;
#[doc = "Receive Queue Control 0 register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxq_ctrl0](mac_rxq_ctrl0) module"]
pub type MAC_RXQ_CTRL0 = crate::Reg<u32, _MAC_RXQ_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_RXQ_CTRL0;
#[doc = "`read()` method returns [mac_rxq_ctrl0::R](mac_rxq_ctrl0::R) reader structure"]
impl crate::Readable for MAC_RXQ_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [mac_rxq_ctrl0::W](mac_rxq_ctrl0::W) writer structure"]
impl crate::Writable for MAC_RXQ_CTRL0 {}
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl0;
#[doc = "Receive Queue Control 0 register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxq_ctrl1](mac_rxq_ctrl1) module"]
pub type MAC_RXQ_CTRL1 = crate::Reg<u32, _MAC_RXQ_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_RXQ_CTRL1;
#[doc = "`read()` method returns [mac_rxq_ctrl1::R](mac_rxq_ctrl1::R) reader structure"]
impl crate::Readable for MAC_RXQ_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [mac_rxq_ctrl1::W](mac_rxq_ctrl1::W) writer structure"]
impl crate::Writable for MAC_RXQ_CTRL1 {}
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl1;
#[doc = "Receive Queue Control 0 register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxq_ctrl2](mac_rxq_ctrl2) module"]
pub type MAC_RXQ_CTRL2 = crate::Reg<u32, _MAC_RXQ_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_RXQ_CTRL2;
#[doc = "`read()` method returns [mac_rxq_ctrl2::R](mac_rxq_ctrl2::R) reader structure"]
impl crate::Readable for MAC_RXQ_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [mac_rxq_ctrl2::W](mac_rxq_ctrl2::W) writer structure"]
impl crate::Writable for MAC_RXQ_CTRL2 {}
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl2;
#[doc = "Interrupt status register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intr_stat](mac_intr_stat) module"]
pub type MAC_INTR_STAT = crate::Reg<u32, _MAC_INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_INTR_STAT;
#[doc = "`read()` method returns [mac_intr_stat::R](mac_intr_stat::R) reader structure"]
impl crate::Readable for MAC_INTR_STAT {}
#[doc = "Interrupt status register 0x0000"]
pub mod mac_intr_stat;
#[doc = "Interrupt enable register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intr_en](mac_intr_en) module"]
pub type MAC_INTR_EN = crate::Reg<u32, _MAC_INTR_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_INTR_EN;
#[doc = "`read()` method returns [mac_intr_en::R](mac_intr_en::R) reader structure"]
impl crate::Readable for MAC_INTR_EN {}
#[doc = "`write(|w| ..)` method takes [mac_intr_en::W](mac_intr_en::W) writer structure"]
impl crate::Writable for MAC_INTR_EN {}
#[doc = "Interrupt enable register 0x0000"]
pub mod mac_intr_en;
#[doc = "Receive Transmit Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxtx_stat](mac_rxtx_stat) module"]
pub type MAC_RXTX_STAT = crate::Reg<u32, _MAC_RXTX_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_RXTX_STAT;
#[doc = "`read()` method returns [mac_rxtx_stat::R](mac_rxtx_stat::R) reader structure"]
impl crate::Readable for MAC_RXTX_STAT {}
#[doc = "Receive Transmit Status register"]
pub mod mac_rxtx_stat;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_pmt_crtl_stat](mac_pmt_crtl_stat) module"]
pub type MAC_PMT_CRTL_STAT = crate::Reg<u32, _MAC_PMT_CRTL_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_PMT_CRTL_STAT;
#[doc = "`read()` method returns [mac_pmt_crtl_stat::R](mac_pmt_crtl_stat::R) reader structure"]
impl crate::Readable for MAC_PMT_CRTL_STAT {}
#[doc = "`write(|w| ..)` method takes [mac_pmt_crtl_stat::W](mac_pmt_crtl_stat::W) writer structure"]
impl crate::Writable for MAC_PMT_CRTL_STAT {}
#[doc = "no description available"]
pub mod mac_pmt_crtl_stat;
#[doc = "Remote wake-up frame filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rwake_frflt](mac_rwake_frflt) module"]
pub type MAC_RWAKE_FRFLT = crate::Reg<u32, _MAC_RWAKE_FRFLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_RWAKE_FRFLT;
#[doc = "`read()` method returns [mac_rwake_frflt::R](mac_rwake_frflt::R) reader structure"]
impl crate::Readable for MAC_RWAKE_FRFLT {}
#[doc = "`write(|w| ..)` method takes [mac_rwake_frflt::W](mac_rwake_frflt::W) writer structure"]
impl crate::Writable for MAC_RWAKE_FRFLT {}
#[doc = "Remote wake-up frame filter"]
pub mod mac_rwake_frflt;
#[doc = "LPI Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_lpi_ctrl_stat](mac_lpi_ctrl_stat) module"]
pub type MAC_LPI_CTRL_STAT = crate::Reg<u32, _MAC_LPI_CTRL_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_LPI_CTRL_STAT;
#[doc = "`read()` method returns [mac_lpi_ctrl_stat::R](mac_lpi_ctrl_stat::R) reader structure"]
impl crate::Readable for MAC_LPI_CTRL_STAT {}
#[doc = "`write(|w| ..)` method takes [mac_lpi_ctrl_stat::W](mac_lpi_ctrl_stat::W) writer structure"]
impl crate::Writable for MAC_LPI_CTRL_STAT {}
#[doc = "LPI Control and Status Register"]
pub mod mac_lpi_ctrl_stat;
#[doc = "LPI Timers Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_lpi_timer_ctrl](mac_lpi_timer_ctrl) module"]
pub type MAC_LPI_TIMER_CTRL = crate::Reg<u32, _MAC_LPI_TIMER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_LPI_TIMER_CTRL;
#[doc = "`read()` method returns [mac_lpi_timer_ctrl::R](mac_lpi_timer_ctrl::R) reader structure"]
impl crate::Readable for MAC_LPI_TIMER_CTRL {}
#[doc = "`write(|w| ..)` method takes [mac_lpi_timer_ctrl::W](mac_lpi_timer_ctrl::W) writer structure"]
impl crate::Writable for MAC_LPI_TIMER_CTRL {}
#[doc = "LPI Timers Control register"]
pub mod mac_lpi_timer_ctrl;
#[doc = "LPI entry Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_lpi_entr_timr](mac_lpi_entr_timr) module"]
pub type MAC_LPI_ENTR_TIMR = crate::Reg<u32, _MAC_LPI_ENTR_TIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_LPI_ENTR_TIMR;
#[doc = "`read()` method returns [mac_lpi_entr_timr::R](mac_lpi_entr_timr::R) reader structure"]
impl crate::Readable for MAC_LPI_ENTR_TIMR {}
#[doc = "`write(|w| ..)` method takes [mac_lpi_entr_timr::W](mac_lpi_entr_timr::W) writer structure"]
impl crate::Writable for MAC_LPI_ENTR_TIMR {}
#[doc = "LPI entry Timer register"]
pub mod mac_lpi_entr_timr;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_1us_tic_countr](mac_1us_tic_countr) module"]
pub type MAC_1US_TIC_COUNTR = crate::Reg<u32, _MAC_1US_TIC_COUNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_1US_TIC_COUNTR;
#[doc = "`read()` method returns [mac_1us_tic_countr::R](mac_1us_tic_countr::R) reader structure"]
impl crate::Readable for MAC_1US_TIC_COUNTR {}
#[doc = "`write(|w| ..)` method takes [mac_1us_tic_countr::W](mac_1us_tic_countr::W) writer structure"]
impl crate::Writable for MAC_1US_TIC_COUNTR {}
#[doc = "no description available"]
pub mod mac_1us_tic_countr;
#[doc = "MAC version register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_version](mac_version) module"]
pub type MAC_VERSION = crate::Reg<u32, _MAC_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_VERSION;
#[doc = "`read()` method returns [mac_version::R](mac_version::R) reader structure"]
impl crate::Readable for MAC_VERSION {}
#[doc = "`write(|w| ..)` method takes [mac_version::W](mac_version::W) writer structure"]
impl crate::Writable for MAC_VERSION {}
#[doc = "MAC version register"]
pub mod mac_version;
#[doc = "MAC debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_dbg](mac_dbg) module"]
pub type MAC_DBG = crate::Reg<u32, _MAC_DBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_DBG;
#[doc = "`read()` method returns [mac_dbg::R](mac_dbg::R) reader structure"]
impl crate::Readable for MAC_DBG {}
#[doc = "MAC debug register"]
pub mod mac_dbg;
#[doc = "MAC hardware feature register 0x0201\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hw_feat0](mac_hw_feat0) module"]
pub type MAC_HW_FEAT0 = crate::Reg<u32, _MAC_HW_FEAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_HW_FEAT0;
#[doc = "`read()` method returns [mac_hw_feat0::R](mac_hw_feat0::R) reader structure"]
impl crate::Readable for MAC_HW_FEAT0 {}
#[doc = "`write(|w| ..)` method takes [mac_hw_feat0::W](mac_hw_feat0::W) writer structure"]
impl crate::Writable for MAC_HW_FEAT0 {}
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat0;
#[doc = "MAC hardware feature register 0x0201\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hw_feat1](mac_hw_feat1) module"]
pub type MAC_HW_FEAT1 = crate::Reg<u32, _MAC_HW_FEAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_HW_FEAT1;
#[doc = "`read()` method returns [mac_hw_feat1::R](mac_hw_feat1::R) reader structure"]
impl crate::Readable for MAC_HW_FEAT1 {}
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat1;
#[doc = "MAC hardware feature register 0x0201\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hw_feat2](mac_hw_feat2) module"]
pub type MAC_HW_FEAT2 = crate::Reg<u32, _MAC_HW_FEAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_HW_FEAT2;
#[doc = "`read()` method returns [mac_hw_feat2::R](mac_hw_feat2::R) reader structure"]
impl crate::Readable for MAC_HW_FEAT2 {}
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat2;
#[doc = "MIDO address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_mdio_addr](mac_mdio_addr) module"]
pub type MAC_MDIO_ADDR = crate::Reg<u32, _MAC_MDIO_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_MDIO_ADDR;
#[doc = "`read()` method returns [mac_mdio_addr::R](mac_mdio_addr::R) reader structure"]
impl crate::Readable for MAC_MDIO_ADDR {}
#[doc = "`write(|w| ..)` method takes [mac_mdio_addr::W](mac_mdio_addr::W) writer structure"]
impl crate::Writable for MAC_MDIO_ADDR {}
#[doc = "MIDO address Register"]
pub mod mac_mdio_addr;
#[doc = "MDIO Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_mdio_data](mac_mdio_data) module"]
pub type MAC_MDIO_DATA = crate::Reg<u32, _MAC_MDIO_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_MDIO_DATA;
#[doc = "`read()` method returns [mac_mdio_data::R](mac_mdio_data::R) reader structure"]
impl crate::Readable for MAC_MDIO_DATA {}
#[doc = "`write(|w| ..)` method takes [mac_mdio_data::W](mac_mdio_data::W) writer structure"]
impl crate::Writable for MAC_MDIO_DATA {}
#[doc = "MDIO Data register"]
pub mod mac_mdio_data;
#[doc = "MAC address0 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_high](mac_addr_high) module"]
pub type MAC_ADDR_HIGH = crate::Reg<u32, _MAC_ADDR_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDR_HIGH;
#[doc = "`read()` method returns [mac_addr_high::R](mac_addr_high::R) reader structure"]
impl crate::Readable for MAC_ADDR_HIGH {}
#[doc = "`write(|w| ..)` method takes [mac_addr_high::W](mac_addr_high::W) writer structure"]
impl crate::Writable for MAC_ADDR_HIGH {}
#[doc = "MAC address0 high register"]
pub mod mac_addr_high;
#[doc = "MAC address0 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_low](mac_addr_low) module"]
pub type MAC_ADDR_LOW = crate::Reg<u32, _MAC_ADDR_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDR_LOW;
#[doc = "`read()` method returns [mac_addr_low::R](mac_addr_low::R) reader structure"]
impl crate::Readable for MAC_ADDR_LOW {}
#[doc = "`write(|w| ..)` method takes [mac_addr_low::W](mac_addr_low::W) writer structure"]
impl crate::Writable for MAC_ADDR_LOW {}
#[doc = "MAC address0 low register"]
pub mod mac_addr_low;
#[doc = "Time stamp control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_timestamp_ctrl](mac_timestamp_ctrl) module"]
pub type MAC_TIMESTAMP_CTRL = crate::Reg<u32, _MAC_TIMESTAMP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_TIMESTAMP_CTRL;
#[doc = "`read()` method returns [mac_timestamp_ctrl::R](mac_timestamp_ctrl::R) reader structure"]
impl crate::Readable for MAC_TIMESTAMP_CTRL {}
#[doc = "`write(|w| ..)` method takes [mac_timestamp_ctrl::W](mac_timestamp_ctrl::W) writer structure"]
impl crate::Writable for MAC_TIMESTAMP_CTRL {}
#[doc = "Time stamp control register"]
pub mod mac_timestamp_ctrl;
#[doc = "Sub-second increment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sub_scnd_incr](mac_sub_scnd_incr) module"]
pub type MAC_SUB_SCND_INCR = crate::Reg<u32, _MAC_SUB_SCND_INCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SUB_SCND_INCR;
#[doc = "`read()` method returns [mac_sub_scnd_incr::R](mac_sub_scnd_incr::R) reader structure"]
impl crate::Readable for MAC_SUB_SCND_INCR {}
#[doc = "`write(|w| ..)` method takes [mac_sub_scnd_incr::W](mac_sub_scnd_incr::W) writer structure"]
impl crate::Writable for MAC_SUB_SCND_INCR {}
#[doc = "Sub-second increment register"]
pub mod mac_sub_scnd_incr;
#[doc = "System time seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_time_scnd](mac_sys_time_scnd) module"]
pub type MAC_SYS_TIME_SCND = crate::Reg<u32, _MAC_SYS_TIME_SCND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SYS_TIME_SCND;
#[doc = "`read()` method returns [mac_sys_time_scnd::R](mac_sys_time_scnd::R) reader structure"]
impl crate::Readable for MAC_SYS_TIME_SCND {}
#[doc = "System time seconds register"]
pub mod mac_sys_time_scnd;
#[doc = "System time nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_time_nscnd](mac_sys_time_nscnd) module"]
pub type MAC_SYS_TIME_NSCND = crate::Reg<u32, _MAC_SYS_TIME_NSCND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SYS_TIME_NSCND;
#[doc = "`read()` method returns [mac_sys_time_nscnd::R](mac_sys_time_nscnd::R) reader structure"]
impl crate::Readable for MAC_SYS_TIME_NSCND {}
#[doc = "System time nanoseconds register"]
pub mod mac_sys_time_nscnd;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_time_scnd_upd](mac_sys_time_scnd_upd) module"]
pub type MAC_SYS_TIME_SCND_UPD = crate::Reg<u32, _MAC_SYS_TIME_SCND_UPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SYS_TIME_SCND_UPD;
#[doc = "`read()` method returns [mac_sys_time_scnd_upd::R](mac_sys_time_scnd_upd::R) reader structure"]
impl crate::Readable for MAC_SYS_TIME_SCND_UPD {}
#[doc = "`write(|w| ..)` method takes [mac_sys_time_scnd_upd::W](mac_sys_time_scnd_upd::W) writer structure"]
impl crate::Writable for MAC_SYS_TIME_SCND_UPD {}
#[doc = "no description available"]
pub mod mac_sys_time_scnd_upd;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_time_nscnd_upd](mac_sys_time_nscnd_upd) module"]
pub type MAC_SYS_TIME_NSCND_UPD = crate::Reg<u32, _MAC_SYS_TIME_NSCND_UPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SYS_TIME_NSCND_UPD;
#[doc = "`read()` method returns [mac_sys_time_nscnd_upd::R](mac_sys_time_nscnd_upd::R) reader structure"]
impl crate::Readable for MAC_SYS_TIME_NSCND_UPD {}
#[doc = "`write(|w| ..)` method takes [mac_sys_time_nscnd_upd::W](mac_sys_time_nscnd_upd::W) writer structure"]
impl crate::Writable for MAC_SYS_TIME_NSCND_UPD {}
#[doc = "no description available"]
pub mod mac_sys_time_nscnd_upd;
#[doc = "Time stamp addend register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_timestmp_addend](mac_sys_timestmp_addend) module"]
pub type MAC_SYS_TIMESTMP_ADDEND = crate::Reg<u32, _MAC_SYS_TIMESTMP_ADDEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SYS_TIMESTMP_ADDEND;
#[doc = "`read()` method returns [mac_sys_timestmp_addend::R](mac_sys_timestmp_addend::R) reader structure"]
impl crate::Readable for MAC_SYS_TIMESTMP_ADDEND {}
#[doc = "`write(|w| ..)` method takes [mac_sys_timestmp_addend::W](mac_sys_timestmp_addend::W) writer structure"]
impl crate::Writable for MAC_SYS_TIMESTMP_ADDEND {}
#[doc = "Time stamp addend register"]
pub mod mac_sys_timestmp_addend;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_time_hword_scnd](mac_sys_time_hword_scnd) module"]
pub type MAC_SYS_TIME_HWORD_SCND = crate::Reg<u32, _MAC_SYS_TIME_HWORD_SCND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SYS_TIME_HWORD_SCND;
#[doc = "`read()` method returns [mac_sys_time_hword_scnd::R](mac_sys_time_hword_scnd::R) reader structure"]
impl crate::Readable for MAC_SYS_TIME_HWORD_SCND {}
#[doc = "`write(|w| ..)` method takes [mac_sys_time_hword_scnd::W](mac_sys_time_hword_scnd::W) writer structure"]
impl crate::Writable for MAC_SYS_TIME_HWORD_SCND {}
#[doc = "no description available"]
pub mod mac_sys_time_hword_scnd;
#[doc = "Time stamp status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_timestmp_stat](mac_sys_timestmp_stat) module"]
pub type MAC_SYS_TIMESTMP_STAT = crate::Reg<u32, _MAC_SYS_TIMESTMP_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_SYS_TIMESTMP_STAT;
#[doc = "`read()` method returns [mac_sys_timestmp_stat::R](mac_sys_timestmp_stat::R) reader structure"]
impl crate::Readable for MAC_SYS_TIMESTMP_STAT {}
#[doc = "Time stamp status register"]
pub mod mac_sys_timestmp_stat;
#[doc = "Tx timestamp status nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_timestamp_status_nanoseconds](mac_tx_timestamp_status_nanoseconds) module"]
pub type MAC_TX_TIMESTAMP_STATUS_NANOSECONDS =
    crate::Reg<u32, _MAC_TX_TIMESTAMP_STATUS_NANOSECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_TX_TIMESTAMP_STATUS_NANOSECONDS;
#[doc = "`read()` method returns [mac_tx_timestamp_status_nanoseconds::R](mac_tx_timestamp_status_nanoseconds::R) reader structure"]
impl crate::Readable for MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {}
#[doc = "Tx timestamp status nanoseconds"]
pub mod mac_tx_timestamp_status_nanoseconds;
#[doc = "Tx timestamp status seconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_timestamp_status_seconds](mac_tx_timestamp_status_seconds) module"]
pub type MAC_TX_TIMESTAMP_STATUS_SECONDS = crate::Reg<u32, _MAC_TX_TIMESTAMP_STATUS_SECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_TX_TIMESTAMP_STATUS_SECONDS;
#[doc = "`read()` method returns [mac_tx_timestamp_status_seconds::R](mac_tx_timestamp_status_seconds::R) reader structure"]
impl crate::Readable for MAC_TX_TIMESTAMP_STATUS_SECONDS {}
#[doc = "Tx timestamp status seconds"]
pub mod mac_tx_timestamp_status_seconds;
#[doc = "Timestamp ingress correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_timestamp_ingress_corr_nanosecond](mac_timestamp_ingress_corr_nanosecond) module"]
pub type MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND =
    crate::Reg<u32, _MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND;
#[doc = "`read()` method returns [mac_timestamp_ingress_corr_nanosecond::R](mac_timestamp_ingress_corr_nanosecond::R) reader structure"]
impl crate::Readable for MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND {}
#[doc = "`write(|w| ..)` method takes [mac_timestamp_ingress_corr_nanosecond::W](mac_timestamp_ingress_corr_nanosecond::W) writer structure"]
impl crate::Writable for MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND {}
#[doc = "Timestamp ingress correction"]
pub mod mac_timestamp_ingress_corr_nanosecond;
#[doc = "Timestamp egress correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_timestamp_egress_corr_nanosecond](mac_timestamp_egress_corr_nanosecond) module"]
pub type MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND =
    crate::Reg<u32, _MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND;
#[doc = "`read()` method returns [mac_timestamp_egress_corr_nanosecond::R](mac_timestamp_egress_corr_nanosecond::R) reader structure"]
impl crate::Readable for MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND {}
#[doc = "`write(|w| ..)` method takes [mac_timestamp_egress_corr_nanosecond::W](mac_timestamp_egress_corr_nanosecond::W) writer structure"]
impl crate::Writable for MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND {}
#[doc = "Timestamp egress correction"]
pub mod mac_timestamp_egress_corr_nanosecond;
#[doc = "MTL Operation Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_op_mode](mtl_op_mode) module"]
pub type MTL_OP_MODE = crate::Reg<u32, _MTL_OP_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_OP_MODE;
#[doc = "`read()` method returns [mtl_op_mode::R](mtl_op_mode::R) reader structure"]
impl crate::Readable for MTL_OP_MODE {}
#[doc = "`write(|w| ..)` method takes [mtl_op_mode::W](mtl_op_mode::W) writer structure"]
impl crate::Writable for MTL_OP_MODE {}
#[doc = "MTL Operation Mode Register"]
pub mod mtl_op_mode;
#[doc = "MTL Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_intr_stat](mtl_intr_stat) module"]
pub type MTL_INTR_STAT = crate::Reg<u32, _MTL_INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_INTR_STAT;
#[doc = "`read()` method returns [mtl_intr_stat::R](mtl_intr_stat::R) reader structure"]
impl crate::Readable for MTL_INTR_STAT {}
#[doc = "MTL Interrupt Status register"]
pub mod mtl_intr_stat;
#[doc = "MTL Receive Queue and DMA Channel Mapping register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxq_dma_map](mtl_rxq_dma_map) module"]
pub type MTL_RXQ_DMA_MAP = crate::Reg<u32, _MTL_RXQ_DMA_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTL_RXQ_DMA_MAP;
#[doc = "`read()` method returns [mtl_rxq_dma_map::R](mtl_rxq_dma_map::R) reader structure"]
impl crate::Readable for MTL_RXQ_DMA_MAP {}
#[doc = "`write(|w| ..)` method takes [mtl_rxq_dma_map::W](mtl_rxq_dma_map::W) writer structure"]
impl crate::Writable for MTL_RXQ_DMA_MAP {}
#[doc = "MTL Receive Queue and DMA Channel Mapping register"]
pub mod mtl_rxq_dma_map;
#[doc = "DMA mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_mode](dma_mode) module"]
pub type DMA_MODE = crate::Reg<u32, _DMA_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_MODE;
#[doc = "`read()` method returns [dma_mode::R](dma_mode::R) reader structure"]
impl crate::Readable for DMA_MODE {}
#[doc = "`write(|w| ..)` method takes [dma_mode::W](dma_mode::W) writer structure"]
impl crate::Writable for DMA_MODE {}
#[doc = "DMA mode register"]
pub mod dma_mode;
#[doc = "DMA System Bus mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sysbus_mode](dma_sysbus_mode) module"]
pub type DMA_SYSBUS_MODE = crate::Reg<u32, _DMA_SYSBUS_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SYSBUS_MODE;
#[doc = "`read()` method returns [dma_sysbus_mode::R](dma_sysbus_mode::R) reader structure"]
impl crate::Readable for DMA_SYSBUS_MODE {}
#[doc = "`write(|w| ..)` method takes [dma_sysbus_mode::W](dma_sysbus_mode::W) writer structure"]
impl crate::Writable for DMA_SYSBUS_MODE {}
#[doc = "DMA System Bus mode"]
pub mod dma_sysbus_mode;
#[doc = "DMA Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_intr_stat](dma_intr_stat) module"]
pub type DMA_INTR_STAT = crate::Reg<u32, _DMA_INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INTR_STAT;
#[doc = "`read()` method returns [dma_intr_stat::R](dma_intr_stat::R) reader structure"]
impl crate::Readable for DMA_INTR_STAT {}
#[doc = "`write(|w| ..)` method takes [dma_intr_stat::W](dma_intr_stat::W) writer structure"]
impl crate::Writable for DMA_INTR_STAT {}
#[doc = "DMA Interrupt status"]
pub mod dma_intr_stat;
#[doc = "DMA Debug Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dbg_stat](dma_dbg_stat) module"]
pub type DMA_DBG_STAT = crate::Reg<u32, _DMA_DBG_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DBG_STAT;
#[doc = "`read()` method returns [dma_dbg_stat::R](dma_dbg_stat::R) reader structure"]
impl crate::Readable for DMA_DBG_STAT {}
#[doc = "`write(|w| ..)` method takes [dma_dbg_stat::W](dma_dbg_stat::W) writer structure"]
impl crate::Writable for DMA_DBG_STAT {}
#[doc = "DMA Debug Status"]
pub mod dma_dbg_stat;
