#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register"]
    pub mac_config: crate::Reg<mac_config::MAC_CONFIG_SPEC>,
    #[doc = "0x04 - no description available"]
    pub mac_ext_config: crate::Reg<mac_ext_config::MAC_EXT_CONFIG_SPEC>,
    #[doc = "0x08 - MAC frame filter register"]
    pub mac_frame_filter: crate::Reg<mac_frame_filter::MAC_FRAME_FILTER_SPEC>,
    #[doc = "0x0c - MAC watchdog Timeout register"]
    pub mac_wd_timerout: crate::Reg<mac_wd_timerout::MAC_WD_TIMEROUT_SPEC>,
    _reserved4: [u8; 0x40],
    #[doc = "0x50 - MAC vlan tag register"]
    pub mac_vlan_tag: crate::Reg<mac_vlan_tag::MAC_VLAN_TAG_SPEC>,
    _reserved5: [u8; 0x1c],
    #[doc = "0x70..0x78 - Transmit flow control register"]
    pub mac_tx_flow_ctrl_q: [crate::Reg<mac_tx_flow_ctrl_q::MAC_TX_FLOW_CTRL_Q_SPEC>; 2],
    _reserved6: [u8; 0x18],
    #[doc = "0x90 - Receive flow control register"]
    pub mac_rx_flow_ctrl: crate::Reg<mac_rx_flow_ctrl::MAC_RX_FLOW_CTRL_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x98 - no description available"]
    pub mac_txq_prio_map: crate::Reg<mac_txq_prio_map::MAC_TXQ_PRIO_MAP_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0xa0 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl0: crate::Reg<mac_rxq_ctrl0::MAC_RXQ_CTRL0_SPEC>,
    #[doc = "0xa4 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl1: crate::Reg<mac_rxq_ctrl1::MAC_RXQ_CTRL1_SPEC>,
    #[doc = "0xa8 - Receive Queue Control 0 register 0x0000"]
    pub mac_rxq_ctrl2: crate::Reg<mac_rxq_ctrl2::MAC_RXQ_CTRL2_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0xb0 - Interrupt status register 0x0000"]
    pub mac_intr_stat: crate::Reg<mac_intr_stat::MAC_INTR_STAT_SPEC>,
    #[doc = "0xb4 - Interrupt enable register 0x0000"]
    pub mac_intr_en: crate::Reg<mac_intr_en::MAC_INTR_EN_SPEC>,
    #[doc = "0xb8 - Receive Transmit Status register"]
    pub mac_rxtx_stat: crate::Reg<mac_rxtx_stat::MAC_RXTX_STAT_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0xc0 - no description available"]
    pub mac_pmt_crtl_stat: crate::Reg<mac_pmt_crtl_stat::MAC_PMT_CRTL_STAT_SPEC>,
    #[doc = "0xc4 - Remote wake-up frame filter"]
    pub mac_rwake_frflt: crate::Reg<mac_rwake_frflt::MAC_RWAKE_FRFLT_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0xd0 - LPI Control and Status Register"]
    pub mac_lpi_ctrl_stat: crate::Reg<mac_lpi_ctrl_stat::MAC_LPI_CTRL_STAT_SPEC>,
    #[doc = "0xd4 - LPI Timers Control register"]
    pub mac_lpi_timer_ctrl: crate::Reg<mac_lpi_timer_ctrl::MAC_LPI_TIMER_CTRL_SPEC>,
    #[doc = "0xd8 - LPI entry Timer register"]
    pub mac_lpi_entr_timr: crate::Reg<mac_lpi_entr_timr::MAC_LPI_ENTR_TIMR_SPEC>,
    #[doc = "0xdc - no description available"]
    pub mac_1us_tic_countr: crate::Reg<mac_1us_tic_countr::MAC_1US_TIC_COUNTR_SPEC>,
    _reserved20: [u8; 0x30],
    #[doc = "0x110 - MAC version register"]
    pub mac_version: crate::Reg<mac_version::MAC_VERSION_SPEC>,
    #[doc = "0x114 - MAC debug register"]
    pub mac_dbg: crate::Reg<mac_dbg::MAC_DBG_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x11c - MAC hardware feature register 0x0201"]
    pub mac_hw_feat0: crate::Reg<mac_hw_feat0::MAC_HW_FEAT0_SPEC>,
    #[doc = "0x120 - MAC hardware feature register 0x0201"]
    pub mac_hw_feat1: crate::Reg<mac_hw_feat1::MAC_HW_FEAT1_SPEC>,
    #[doc = "0x124 - MAC hardware feature register 0x0201"]
    pub mac_hw_feat2: crate::Reg<mac_hw_feat2::MAC_HW_FEAT2_SPEC>,
    _reserved25: [u8; 0xd8],
    #[doc = "0x200 - MIDO address Register"]
    pub mac_mdio_addr: crate::Reg<mac_mdio_addr::MAC_MDIO_ADDR_SPEC>,
    #[doc = "0x204 - MDIO Data register"]
    pub mac_mdio_data: crate::Reg<mac_mdio_data::MAC_MDIO_DATA_SPEC>,
    _reserved27: [u8; 0xf8],
    #[doc = "0x300 - MAC address0 high register"]
    pub mac_addr_high: crate::Reg<mac_addr_high::MAC_ADDR_HIGH_SPEC>,
    #[doc = "0x304 - MAC address0 low register"]
    pub mac_addr_low: crate::Reg<mac_addr_low::MAC_ADDR_LOW_SPEC>,
    _reserved29: [u8; 0x07f8],
    #[doc = "0xb00 - Time stamp control register"]
    pub mac_timestamp_ctrl: crate::Reg<mac_timestamp_ctrl::MAC_TIMESTAMP_CTRL_SPEC>,
    #[doc = "0xb04 - Sub-second increment register"]
    pub mac_sub_scnd_incr: crate::Reg<mac_sub_scnd_incr::MAC_SUB_SCND_INCR_SPEC>,
    #[doc = "0xb08 - System time seconds register"]
    pub mac_sys_time_scnd: crate::Reg<mac_sys_time_scnd::MAC_SYS_TIME_SCND_SPEC>,
    #[doc = "0xb0c - System time nanoseconds register"]
    pub mac_sys_time_nscnd: crate::Reg<mac_sys_time_nscnd::MAC_SYS_TIME_NSCND_SPEC>,
    #[doc = "0xb10 - no description available"]
    pub mac_sys_time_scnd_upd: crate::Reg<mac_sys_time_scnd_upd::MAC_SYS_TIME_SCND_UPD_SPEC>,
    #[doc = "0xb14 - no description available"]
    pub mac_sys_time_nscnd_upd: crate::Reg<mac_sys_time_nscnd_upd::MAC_SYS_TIME_NSCND_UPD_SPEC>,
    #[doc = "0xb18 - Time stamp addend register"]
    pub mac_sys_timestmp_addend: crate::Reg<mac_sys_timestmp_addend::MAC_SYS_TIMESTMP_ADDEND_SPEC>,
    #[doc = "0xb1c - no description available"]
    pub mac_sys_time_hword_scnd: crate::Reg<mac_sys_time_hword_scnd::MAC_SYS_TIME_HWORD_SCND_SPEC>,
    #[doc = "0xb20 - Time stamp status register"]
    pub mac_sys_timestmp_stat: crate::Reg<mac_sys_timestmp_stat::MAC_SYS_TIMESTMP_STAT_SPEC>,
    _reserved38: [u8; 0x0c],
    #[doc = "0xb30 - Tx timestamp status nanoseconds"]
    pub mac_tx_timestamp_status_nanoseconds:
        crate::Reg<mac_tx_timestamp_status_nanoseconds::MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC>,
    #[doc = "0xb34 - Tx timestamp status seconds"]
    pub mac_tx_timestamp_status_seconds:
        crate::Reg<mac_tx_timestamp_status_seconds::MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC>,
    _reserved40: [u8; 0x20],
    #[doc = "0xb58 - Timestamp ingress correction"]
    pub mac_timestamp_ingress_corr_nanosecond: crate::Reg<
        mac_timestamp_ingress_corr_nanosecond::MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND_SPEC,
    >,
    #[doc = "0xb5c - Timestamp egress correction"]
    pub mac_timestamp_egress_corr_nanosecond:
        crate::Reg<mac_timestamp_egress_corr_nanosecond::MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>,
    _reserved42: [u8; 0xa0],
    #[doc = "0xc00 - MTL Operation Mode Register"]
    pub mtl_op_mode: crate::Reg<mtl_op_mode::MTL_OP_MODE_SPEC>,
    _reserved43: [u8; 0x1c],
    #[doc = "0xc20 - MTL Interrupt Status register"]
    pub mtl_intr_stat: crate::Reg<mtl_intr_stat::MTL_INTR_STAT_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0xc30 - MTL Receive Queue and DMA Channel Mapping register"]
    pub mtl_rxq_dma_map: crate::Reg<mtl_rxq_dma_map::MTL_RXQ_DMA_MAP_SPEC>,
    _reserved45: [u8; 0xcc],
    #[doc = "0xd00..0xd80 - no description available"]
    pub mtl_queue: [MTL_QUEUE; 2],
    _reserved46: [u8; 0x0280],
    #[doc = "0x1000 - DMA mode register"]
    pub dma_mode: crate::Reg<dma_mode::DMA_MODE_SPEC>,
    #[doc = "0x1004 - DMA System Bus mode"]
    pub dma_sysbus_mode: crate::Reg<dma_sysbus_mode::DMA_SYSBUS_MODE_SPEC>,
    #[doc = "0x1008 - DMA Interrupt status"]
    pub dma_intr_stat: crate::Reg<dma_intr_stat::DMA_INTR_STAT_SPEC>,
    #[doc = "0x100c - DMA Debug Status"]
    pub dma_dbg_stat: crate::Reg<dma_dbg_stat::DMA_DBG_STAT_SPEC>,
    _reserved50: [u8; 0xf0],
    #[doc = "0x1100..0x1164 - no description available"]
    pub dma_ch0: DMA_CH,
    _reserved51: [u8; 0x1c],
    #[doc = "0x1180..0x11e4 - no description available"]
    pub dma_ch1: DMA_CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MTL_QUEUE {
    #[doc = "0x00 - MTL TxQx Operation Mode register"]
    pub mtl_txqx_op_mode: crate::Reg<self::mtl_queue::mtl_txqx_op_mode::MTL_TXQX_OP_MODE_SPEC>,
    #[doc = "0x04 - MTL TxQx Underflow register"]
    pub mtl_txqx_undrflw: crate::Reg<self::mtl_queue::mtl_txqx_undrflw::MTL_TXQX_UNDRFLW_SPEC>,
    #[doc = "0x08 - MTL TxQx Debug register"]
    pub mtl_txqx_dbg: crate::Reg<self::mtl_queue::mtl_txqx_dbg::MTL_TXQX_DBG_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - MTL TxQx ETS control register, only TxQ1 support"]
    pub mtl_txqx_ets_ctrl: crate::Reg<self::mtl_queue::mtl_txqx_ets_ctrl::MTL_TXQX_ETS_CTRL_SPEC>,
    #[doc = "0x14 - MTL TxQx ETS Status register"]
    pub mtl_txqx_ets_stat: crate::Reg<self::mtl_queue::mtl_txqx_ets_stat::MTL_TXQX_ETS_STAT_SPEC>,
    #[doc = "0x18 - no description available"]
    pub mtl_txqx_qntm_wght:
        crate::Reg<self::mtl_queue::mtl_txqx_qntm_wght::MTL_TXQX_QNTM_WGHT_SPEC>,
    #[doc = "0x1c - MTL TxQx SendSlopCredit register, only TxQ1 support"]
    pub mtl_txqx_sndslp_crdt:
        crate::Reg<self::mtl_queue::mtl_txqx_sndslp_crdt::MTL_TXQX_SNDSLP_CRDT_SPEC>,
    #[doc = "0x20 - MTL TxQx hiCredit register, only TxQ1 support"]
    pub mtl_txqx_hi_crdt: crate::Reg<self::mtl_queue::mtl_txqx_hi_crdt::MTL_TXQX_HI_CRDT_SPEC>,
    #[doc = "0x24 - MTL TxQx loCredit register, only TxQ1 support"]
    pub mtl_txqx_lo_crdt: crate::Reg<self::mtl_queue::mtl_txqx_lo_crdt::MTL_TXQX_LO_CRDT_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x2c - no description available"]
    pub mtl_txqx_intctrl_stat:
        crate::Reg<self::mtl_queue::mtl_txqx_intctrl_stat::MTL_TXQX_INTCTRL_STAT_SPEC>,
    #[doc = "0x30 - MTL RxQx Operation Mode register"]
    pub mtl_rxqx_op_mode: crate::Reg<self::mtl_queue::mtl_rxqx_op_mode::MTL_RXQX_OP_MODE_SPEC>,
    #[doc = "0x34 - MTL RxQx Missed Packet Overflow Counter register"]
    pub mtl_rxqx_misspkt_ovrflw_cnt:
        crate::Reg<self::mtl_queue::mtl_rxqx_misspkt_ovrflw_cnt::MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>,
    #[doc = "0x38 - MTL RxQx Debug register"]
    pub mtl_rxqx_dbg: crate::Reg<self::mtl_queue::mtl_rxqx_dbg::MTL_RXQX_DBG_SPEC>,
    #[doc = "0x3c - MTL RxQx Control register"]
    pub mtl_rxqx_ctrl: crate::Reg<self::mtl_queue::mtl_rxqx_ctrl::MTL_RXQX_CTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod mtl_queue;
#[doc = r"Register block"]
#[repr(C)]
pub struct DMA_CH {
    #[doc = "0x00 - DMA Channelx Control"]
    pub dma_chx_ctrl: crate::Reg<self::dma_ch::dma_chx_ctrl::DMA_CHX_CTRL_SPEC>,
    #[doc = "0x04 - DMA Channelx Transmit Control"]
    pub dma_chx_tx_ctrl: crate::Reg<self::dma_ch::dma_chx_tx_ctrl::DMA_CHX_TX_CTRL_SPEC>,
    #[doc = "0x08 - DMA Channelx Receive Control"]
    pub dma_chx_rx_ctrl: crate::Reg<self::dma_ch::dma_chx_rx_ctrl::DMA_CHX_RX_CTRL_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x14 - no description available"]
    pub dma_chx_txdesc_list_addr:
        crate::Reg<self::dma_ch::dma_chx_txdesc_list_addr::DMA_CHX_TXDESC_LIST_ADDR_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x1c - no description available"]
    pub dma_chx_rxdesc_list_addr:
        crate::Reg<self::dma_ch::dma_chx_rxdesc_list_addr::DMA_CHX_RXDESC_LIST_ADDR_SPEC>,
    #[doc = "0x20 - no description available"]
    pub dma_chx_txdesc_tail_ptr:
        crate::Reg<self::dma_ch::dma_chx_txdesc_tail_ptr::DMA_CHX_TXDESC_TAIL_PTR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x28 - no description available"]
    pub dma_chx_rxdesc_tail_ptr:
        crate::Reg<self::dma_ch::dma_chx_rxdesc_tail_ptr::DMA_CHX_RXDESC_TAIL_PTR_SPEC>,
    #[doc = "0x2c - no description available"]
    pub dma_chx_txdesc_ring_length:
        crate::Reg<self::dma_ch::dma_chx_txdesc_ring_length::DMA_CHX_TXDESC_RING_LENGTH_SPEC>,
    #[doc = "0x30 - Channelx Rx descriptor Ring Length"]
    pub dma_chx_rxdesc_ring_length:
        crate::Reg<self::dma_ch::dma_chx_rxdesc_ring_length::DMA_CHX_RXDESC_RING_LENGTH_SPEC>,
    #[doc = "0x34 - Channelx Interrupt Enable"]
    pub dma_chx_int_en: crate::Reg<self::dma_ch::dma_chx_int_en::DMA_CHX_INT_EN_SPEC>,
    #[doc = "0x38 - Receive Interrupt Watchdog Timer"]
    pub dma_chx_rx_int_wdtimer:
        crate::Reg<self::dma_ch::dma_chx_rx_int_wdtimer::DMA_CHX_RX_INT_WDTIMER_SPEC>,
    #[doc = "0x3c - Slot Function Control and Status"]
    pub dma_chx_slot_func_ctrl_stat:
        crate::Reg<self::dma_ch::dma_chx_slot_func_ctrl_stat::DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x44 - Channelx Current Host Transmit descriptor"]
    pub dma_chx_cur_hst_txdesc:
        crate::Reg<self::dma_ch::dma_chx_cur_hst_txdesc::DMA_CHX_CUR_HST_TXDESC_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - no description available"]
    pub dma_chx_cur_hst_rxdesc:
        crate::Reg<self::dma_ch::dma_chx_cur_hst_rxdesc::DMA_CHX_CUR_HST_RXDESC_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x54 - no description available"]
    pub dma_chx_cur_hst_txbuf:
        crate::Reg<self::dma_ch::dma_chx_cur_hst_txbuf::DMA_CHX_CUR_HST_TXBUF_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x5c - Channelx Current Application Receive Buffer Address"]
    pub dma_chx_cur_hst_rxbuf:
        crate::Reg<self::dma_ch::dma_chx_cur_hst_rxbuf::DMA_CHX_CUR_HST_RXBUF_SPEC>,
    #[doc = "0x60 - Channelx DMA status register"]
    pub dma_chx_stat: crate::Reg<self::dma_ch::dma_chx_stat::DMA_CHX_STAT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod dma_ch;
#[doc = "MAC_CONFIG register accessor: an alias for `Reg<MAC_CONFIG_SPEC>`"]
pub type MAC_CONFIG = crate::Reg<mac_config::MAC_CONFIG_SPEC>;
#[doc = "MAC configuration register"]
pub mod mac_config;
#[doc = "MAC_EXT_CONFIG register accessor: an alias for `Reg<MAC_EXT_CONFIG_SPEC>`"]
pub type MAC_EXT_CONFIG = crate::Reg<mac_ext_config::MAC_EXT_CONFIG_SPEC>;
#[doc = "no description available"]
pub mod mac_ext_config;
#[doc = "MAC_FRAME_FILTER register accessor: an alias for `Reg<MAC_FRAME_FILTER_SPEC>`"]
pub type MAC_FRAME_FILTER = crate::Reg<mac_frame_filter::MAC_FRAME_FILTER_SPEC>;
#[doc = "MAC frame filter register"]
pub mod mac_frame_filter;
#[doc = "MAC_WD_TIMEROUT register accessor: an alias for `Reg<MAC_WD_TIMEROUT_SPEC>`"]
pub type MAC_WD_TIMEROUT = crate::Reg<mac_wd_timerout::MAC_WD_TIMEROUT_SPEC>;
#[doc = "MAC watchdog Timeout register"]
pub mod mac_wd_timerout;
#[doc = "MAC_VLAN_TAG register accessor: an alias for `Reg<MAC_VLAN_TAG_SPEC>`"]
pub type MAC_VLAN_TAG = crate::Reg<mac_vlan_tag::MAC_VLAN_TAG_SPEC>;
#[doc = "MAC vlan tag register"]
pub mod mac_vlan_tag;
#[doc = "MAC_TX_FLOW_CTRL_Q register accessor: an alias for `Reg<MAC_TX_FLOW_CTRL_Q_SPEC>`"]
pub type MAC_TX_FLOW_CTRL_Q = crate::Reg<mac_tx_flow_ctrl_q::MAC_TX_FLOW_CTRL_Q_SPEC>;
#[doc = "Transmit flow control register"]
pub mod mac_tx_flow_ctrl_q;
#[doc = "MAC_RX_FLOW_CTRL register accessor: an alias for `Reg<MAC_RX_FLOW_CTRL_SPEC>`"]
pub type MAC_RX_FLOW_CTRL = crate::Reg<mac_rx_flow_ctrl::MAC_RX_FLOW_CTRL_SPEC>;
#[doc = "Receive flow control register"]
pub mod mac_rx_flow_ctrl;
#[doc = "MAC_TXQ_PRIO_MAP register accessor: an alias for `Reg<MAC_TXQ_PRIO_MAP_SPEC>`"]
pub type MAC_TXQ_PRIO_MAP = crate::Reg<mac_txq_prio_map::MAC_TXQ_PRIO_MAP_SPEC>;
#[doc = "no description available"]
pub mod mac_txq_prio_map;
#[doc = "MAC_RXQ_CTRL0 register accessor: an alias for `Reg<MAC_RXQ_CTRL0_SPEC>`"]
pub type MAC_RXQ_CTRL0 = crate::Reg<mac_rxq_ctrl0::MAC_RXQ_CTRL0_SPEC>;
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl0;
#[doc = "MAC_RXQ_CTRL1 register accessor: an alias for `Reg<MAC_RXQ_CTRL1_SPEC>`"]
pub type MAC_RXQ_CTRL1 = crate::Reg<mac_rxq_ctrl1::MAC_RXQ_CTRL1_SPEC>;
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl1;
#[doc = "MAC_RXQ_CTRL2 register accessor: an alias for `Reg<MAC_RXQ_CTRL2_SPEC>`"]
pub type MAC_RXQ_CTRL2 = crate::Reg<mac_rxq_ctrl2::MAC_RXQ_CTRL2_SPEC>;
#[doc = "Receive Queue Control 0 register 0x0000"]
pub mod mac_rxq_ctrl2;
#[doc = "MAC_INTR_STAT register accessor: an alias for `Reg<MAC_INTR_STAT_SPEC>`"]
pub type MAC_INTR_STAT = crate::Reg<mac_intr_stat::MAC_INTR_STAT_SPEC>;
#[doc = "Interrupt status register 0x0000"]
pub mod mac_intr_stat;
#[doc = "MAC_INTR_EN register accessor: an alias for `Reg<MAC_INTR_EN_SPEC>`"]
pub type MAC_INTR_EN = crate::Reg<mac_intr_en::MAC_INTR_EN_SPEC>;
#[doc = "Interrupt enable register 0x0000"]
pub mod mac_intr_en;
#[doc = "MAC_RXTX_STAT register accessor: an alias for `Reg<MAC_RXTX_STAT_SPEC>`"]
pub type MAC_RXTX_STAT = crate::Reg<mac_rxtx_stat::MAC_RXTX_STAT_SPEC>;
#[doc = "Receive Transmit Status register"]
pub mod mac_rxtx_stat;
#[doc = "MAC_PMT_CRTL_STAT register accessor: an alias for `Reg<MAC_PMT_CRTL_STAT_SPEC>`"]
pub type MAC_PMT_CRTL_STAT = crate::Reg<mac_pmt_crtl_stat::MAC_PMT_CRTL_STAT_SPEC>;
#[doc = "no description available"]
pub mod mac_pmt_crtl_stat;
#[doc = "MAC_RWAKE_FRFLT register accessor: an alias for `Reg<MAC_RWAKE_FRFLT_SPEC>`"]
pub type MAC_RWAKE_FRFLT = crate::Reg<mac_rwake_frflt::MAC_RWAKE_FRFLT_SPEC>;
#[doc = "Remote wake-up frame filter"]
pub mod mac_rwake_frflt;
#[doc = "MAC_LPI_CTRL_STAT register accessor: an alias for `Reg<MAC_LPI_CTRL_STAT_SPEC>`"]
pub type MAC_LPI_CTRL_STAT = crate::Reg<mac_lpi_ctrl_stat::MAC_LPI_CTRL_STAT_SPEC>;
#[doc = "LPI Control and Status Register"]
pub mod mac_lpi_ctrl_stat;
#[doc = "MAC_LPI_TIMER_CTRL register accessor: an alias for `Reg<MAC_LPI_TIMER_CTRL_SPEC>`"]
pub type MAC_LPI_TIMER_CTRL = crate::Reg<mac_lpi_timer_ctrl::MAC_LPI_TIMER_CTRL_SPEC>;
#[doc = "LPI Timers Control register"]
pub mod mac_lpi_timer_ctrl;
#[doc = "MAC_LPI_ENTR_TIMR register accessor: an alias for `Reg<MAC_LPI_ENTR_TIMR_SPEC>`"]
pub type MAC_LPI_ENTR_TIMR = crate::Reg<mac_lpi_entr_timr::MAC_LPI_ENTR_TIMR_SPEC>;
#[doc = "LPI entry Timer register"]
pub mod mac_lpi_entr_timr;
#[doc = "MAC_1US_TIC_COUNTR register accessor: an alias for `Reg<MAC_1US_TIC_COUNTR_SPEC>`"]
pub type MAC_1US_TIC_COUNTR = crate::Reg<mac_1us_tic_countr::MAC_1US_TIC_COUNTR_SPEC>;
#[doc = "no description available"]
pub mod mac_1us_tic_countr;
#[doc = "MAC_VERSION register accessor: an alias for `Reg<MAC_VERSION_SPEC>`"]
pub type MAC_VERSION = crate::Reg<mac_version::MAC_VERSION_SPEC>;
#[doc = "MAC version register"]
pub mod mac_version;
#[doc = "MAC_DBG register accessor: an alias for `Reg<MAC_DBG_SPEC>`"]
pub type MAC_DBG = crate::Reg<mac_dbg::MAC_DBG_SPEC>;
#[doc = "MAC debug register"]
pub mod mac_dbg;
#[doc = "MAC_HW_FEAT0 register accessor: an alias for `Reg<MAC_HW_FEAT0_SPEC>`"]
pub type MAC_HW_FEAT0 = crate::Reg<mac_hw_feat0::MAC_HW_FEAT0_SPEC>;
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat0;
#[doc = "MAC_HW_FEAT1 register accessor: an alias for `Reg<MAC_HW_FEAT1_SPEC>`"]
pub type MAC_HW_FEAT1 = crate::Reg<mac_hw_feat1::MAC_HW_FEAT1_SPEC>;
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat1;
#[doc = "MAC_HW_FEAT2 register accessor: an alias for `Reg<MAC_HW_FEAT2_SPEC>`"]
pub type MAC_HW_FEAT2 = crate::Reg<mac_hw_feat2::MAC_HW_FEAT2_SPEC>;
#[doc = "MAC hardware feature register 0x0201"]
pub mod mac_hw_feat2;
#[doc = "MAC_MDIO_ADDR register accessor: an alias for `Reg<MAC_MDIO_ADDR_SPEC>`"]
pub type MAC_MDIO_ADDR = crate::Reg<mac_mdio_addr::MAC_MDIO_ADDR_SPEC>;
#[doc = "MIDO address Register"]
pub mod mac_mdio_addr;
#[doc = "MAC_MDIO_DATA register accessor: an alias for `Reg<MAC_MDIO_DATA_SPEC>`"]
pub type MAC_MDIO_DATA = crate::Reg<mac_mdio_data::MAC_MDIO_DATA_SPEC>;
#[doc = "MDIO Data register"]
pub mod mac_mdio_data;
#[doc = "MAC_ADDR_HIGH register accessor: an alias for `Reg<MAC_ADDR_HIGH_SPEC>`"]
pub type MAC_ADDR_HIGH = crate::Reg<mac_addr_high::MAC_ADDR_HIGH_SPEC>;
#[doc = "MAC address0 high register"]
pub mod mac_addr_high;
#[doc = "MAC_ADDR_LOW register accessor: an alias for `Reg<MAC_ADDR_LOW_SPEC>`"]
pub type MAC_ADDR_LOW = crate::Reg<mac_addr_low::MAC_ADDR_LOW_SPEC>;
#[doc = "MAC address0 low register"]
pub mod mac_addr_low;
#[doc = "MAC_TIMESTAMP_CTRL register accessor: an alias for `Reg<MAC_TIMESTAMP_CTRL_SPEC>`"]
pub type MAC_TIMESTAMP_CTRL = crate::Reg<mac_timestamp_ctrl::MAC_TIMESTAMP_CTRL_SPEC>;
#[doc = "Time stamp control register"]
pub mod mac_timestamp_ctrl;
#[doc = "MAC_SUB_SCND_INCR register accessor: an alias for `Reg<MAC_SUB_SCND_INCR_SPEC>`"]
pub type MAC_SUB_SCND_INCR = crate::Reg<mac_sub_scnd_incr::MAC_SUB_SCND_INCR_SPEC>;
#[doc = "Sub-second increment register"]
pub mod mac_sub_scnd_incr;
#[doc = "MAC_SYS_TIME_SCND register accessor: an alias for `Reg<MAC_SYS_TIME_SCND_SPEC>`"]
pub type MAC_SYS_TIME_SCND = crate::Reg<mac_sys_time_scnd::MAC_SYS_TIME_SCND_SPEC>;
#[doc = "System time seconds register"]
pub mod mac_sys_time_scnd;
#[doc = "MAC_SYS_TIME_NSCND register accessor: an alias for `Reg<MAC_SYS_TIME_NSCND_SPEC>`"]
pub type MAC_SYS_TIME_NSCND = crate::Reg<mac_sys_time_nscnd::MAC_SYS_TIME_NSCND_SPEC>;
#[doc = "System time nanoseconds register"]
pub mod mac_sys_time_nscnd;
#[doc = "MAC_SYS_TIME_SCND_UPD register accessor: an alias for `Reg<MAC_SYS_TIME_SCND_UPD_SPEC>`"]
pub type MAC_SYS_TIME_SCND_UPD = crate::Reg<mac_sys_time_scnd_upd::MAC_SYS_TIME_SCND_UPD_SPEC>;
#[doc = "no description available"]
pub mod mac_sys_time_scnd_upd;
#[doc = "MAC_SYS_TIME_NSCND_UPD register accessor: an alias for `Reg<MAC_SYS_TIME_NSCND_UPD_SPEC>`"]
pub type MAC_SYS_TIME_NSCND_UPD = crate::Reg<mac_sys_time_nscnd_upd::MAC_SYS_TIME_NSCND_UPD_SPEC>;
#[doc = "no description available"]
pub mod mac_sys_time_nscnd_upd;
#[doc = "MAC_SYS_TIMESTMP_ADDEND register accessor: an alias for `Reg<MAC_SYS_TIMESTMP_ADDEND_SPEC>`"]
pub type MAC_SYS_TIMESTMP_ADDEND =
    crate::Reg<mac_sys_timestmp_addend::MAC_SYS_TIMESTMP_ADDEND_SPEC>;
#[doc = "Time stamp addend register"]
pub mod mac_sys_timestmp_addend;
#[doc = "MAC_SYS_TIME_HWORD_SCND register accessor: an alias for `Reg<MAC_SYS_TIME_HWORD_SCND_SPEC>`"]
pub type MAC_SYS_TIME_HWORD_SCND =
    crate::Reg<mac_sys_time_hword_scnd::MAC_SYS_TIME_HWORD_SCND_SPEC>;
#[doc = "no description available"]
pub mod mac_sys_time_hword_scnd;
#[doc = "MAC_SYS_TIMESTMP_STAT register accessor: an alias for `Reg<MAC_SYS_TIMESTMP_STAT_SPEC>`"]
pub type MAC_SYS_TIMESTMP_STAT = crate::Reg<mac_sys_timestmp_stat::MAC_SYS_TIMESTMP_STAT_SPEC>;
#[doc = "Time stamp status register"]
pub mod mac_sys_timestmp_stat;
#[doc = "MAC_Tx_TIMESTAMP_STATUS_NANOSECONDS register accessor: an alias for `Reg<MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC>`"]
pub type MAC_TX_TIMESTAMP_STATUS_NANOSECONDS =
    crate::Reg<mac_tx_timestamp_status_nanoseconds::MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC>;
#[doc = "Tx timestamp status nanoseconds"]
pub mod mac_tx_timestamp_status_nanoseconds;
#[doc = "MAC_Tx_TIMESTAMP_STATUS_SECONDS register accessor: an alias for `Reg<MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC>`"]
pub type MAC_TX_TIMESTAMP_STATUS_SECONDS =
    crate::Reg<mac_tx_timestamp_status_seconds::MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC>;
#[doc = "Tx timestamp status seconds"]
pub mod mac_tx_timestamp_status_seconds;
#[doc = "MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND register accessor: an alias for `Reg<MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND_SPEC>`"]
pub type MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND =
    crate::Reg<mac_timestamp_ingress_corr_nanosecond::MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND_SPEC>;
#[doc = "Timestamp ingress correction"]
pub mod mac_timestamp_ingress_corr_nanosecond;
#[doc = "MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND register accessor: an alias for `Reg<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>`"]
pub type MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND =
    crate::Reg<mac_timestamp_egress_corr_nanosecond::MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>;
#[doc = "Timestamp egress correction"]
pub mod mac_timestamp_egress_corr_nanosecond;
#[doc = "MTL_OP_MODE register accessor: an alias for `Reg<MTL_OP_MODE_SPEC>`"]
pub type MTL_OP_MODE = crate::Reg<mtl_op_mode::MTL_OP_MODE_SPEC>;
#[doc = "MTL Operation Mode Register"]
pub mod mtl_op_mode;
#[doc = "MTL_INTR_STAT register accessor: an alias for `Reg<MTL_INTR_STAT_SPEC>`"]
pub type MTL_INTR_STAT = crate::Reg<mtl_intr_stat::MTL_INTR_STAT_SPEC>;
#[doc = "MTL Interrupt Status register"]
pub mod mtl_intr_stat;
#[doc = "MTL_RXQ_DMA_MAP register accessor: an alias for `Reg<MTL_RXQ_DMA_MAP_SPEC>`"]
pub type MTL_RXQ_DMA_MAP = crate::Reg<mtl_rxq_dma_map::MTL_RXQ_DMA_MAP_SPEC>;
#[doc = "MTL Receive Queue and DMA Channel Mapping register"]
pub mod mtl_rxq_dma_map;
#[doc = "DMA_MODE register accessor: an alias for `Reg<DMA_MODE_SPEC>`"]
pub type DMA_MODE = crate::Reg<dma_mode::DMA_MODE_SPEC>;
#[doc = "DMA mode register"]
pub mod dma_mode;
#[doc = "DMA_SYSBUS_MODE register accessor: an alias for `Reg<DMA_SYSBUS_MODE_SPEC>`"]
pub type DMA_SYSBUS_MODE = crate::Reg<dma_sysbus_mode::DMA_SYSBUS_MODE_SPEC>;
#[doc = "DMA System Bus mode"]
pub mod dma_sysbus_mode;
#[doc = "DMA_INTR_STAT register accessor: an alias for `Reg<DMA_INTR_STAT_SPEC>`"]
pub type DMA_INTR_STAT = crate::Reg<dma_intr_stat::DMA_INTR_STAT_SPEC>;
#[doc = "DMA Interrupt status"]
pub mod dma_intr_stat;
#[doc = "DMA_DBG_STAT register accessor: an alias for `Reg<DMA_DBG_STAT_SPEC>`"]
pub type DMA_DBG_STAT = crate::Reg<dma_dbg_stat::DMA_DBG_STAT_SPEC>;
#[doc = "DMA Debug Status"]
pub mod dma_dbg_stat;
