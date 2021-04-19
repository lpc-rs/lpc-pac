#[doc = "DMA_CHx_CTRL register accessor: an alias for `Reg<DMA_CHX_CTRL_SPEC>`"]
pub type DMA_CHX_CTRL = crate::Reg<dma_chx_ctrl::DMA_CHX_CTRL_SPEC>;
#[doc = "DMA Channelx Control"]
pub mod dma_chx_ctrl;
#[doc = "DMA_CHx_TX_CTRL register accessor: an alias for `Reg<DMA_CHX_TX_CTRL_SPEC>`"]
pub type DMA_CHX_TX_CTRL = crate::Reg<dma_chx_tx_ctrl::DMA_CHX_TX_CTRL_SPEC>;
#[doc = "DMA Channelx Transmit Control"]
pub mod dma_chx_tx_ctrl;
#[doc = "DMA_CHx_RX_CTRL register accessor: an alias for `Reg<DMA_CHX_RX_CTRL_SPEC>`"]
pub type DMA_CHX_RX_CTRL = crate::Reg<dma_chx_rx_ctrl::DMA_CHX_RX_CTRL_SPEC>;
#[doc = "DMA Channelx Receive Control"]
pub mod dma_chx_rx_ctrl;
#[doc = "DMA_CHx_TXDESC_LIST_ADDR register accessor: an alias for `Reg<DMA_CHX_TXDESC_LIST_ADDR_SPEC>`"]
pub type DMA_CHX_TXDESC_LIST_ADDR =
    crate::Reg<dma_chx_txdesc_list_addr::DMA_CHX_TXDESC_LIST_ADDR_SPEC>;
#[doc = "no description available"]
pub mod dma_chx_txdesc_list_addr;
#[doc = "DMA_CHx_RXDESC_LIST_ADDR register accessor: an alias for `Reg<DMA_CHX_RXDESC_LIST_ADDR_SPEC>`"]
pub type DMA_CHX_RXDESC_LIST_ADDR =
    crate::Reg<dma_chx_rxdesc_list_addr::DMA_CHX_RXDESC_LIST_ADDR_SPEC>;
#[doc = "no description available"]
pub mod dma_chx_rxdesc_list_addr;
#[doc = "DMA_CHx_TXDESC_TAIL_PTR register accessor: an alias for `Reg<DMA_CHX_TXDESC_TAIL_PTR_SPEC>`"]
pub type DMA_CHX_TXDESC_TAIL_PTR =
    crate::Reg<dma_chx_txdesc_tail_ptr::DMA_CHX_TXDESC_TAIL_PTR_SPEC>;
#[doc = "no description available"]
pub mod dma_chx_txdesc_tail_ptr;
#[doc = "DMA_CHx_RXDESC_TAIL_PTR register accessor: an alias for `Reg<DMA_CHX_RXDESC_TAIL_PTR_SPEC>`"]
pub type DMA_CHX_RXDESC_TAIL_PTR =
    crate::Reg<dma_chx_rxdesc_tail_ptr::DMA_CHX_RXDESC_TAIL_PTR_SPEC>;
#[doc = "no description available"]
pub mod dma_chx_rxdesc_tail_ptr;
#[doc = "DMA_CHx_TXDESC_RING_LENGTH register accessor: an alias for `Reg<DMA_CHX_TXDESC_RING_LENGTH_SPEC>`"]
pub type DMA_CHX_TXDESC_RING_LENGTH =
    crate::Reg<dma_chx_txdesc_ring_length::DMA_CHX_TXDESC_RING_LENGTH_SPEC>;
#[doc = "no description available"]
pub mod dma_chx_txdesc_ring_length;
#[doc = "DMA_CHx_RXDESC_RING_LENGTH register accessor: an alias for `Reg<DMA_CHX_RXDESC_RING_LENGTH_SPEC>`"]
pub type DMA_CHX_RXDESC_RING_LENGTH =
    crate::Reg<dma_chx_rxdesc_ring_length::DMA_CHX_RXDESC_RING_LENGTH_SPEC>;
#[doc = "Channelx Rx descriptor Ring Length"]
pub mod dma_chx_rxdesc_ring_length;
#[doc = "DMA_CHx_INT_EN register accessor: an alias for `Reg<DMA_CHX_INT_EN_SPEC>`"]
pub type DMA_CHX_INT_EN = crate::Reg<dma_chx_int_en::DMA_CHX_INT_EN_SPEC>;
#[doc = "Channelx Interrupt Enable"]
pub mod dma_chx_int_en;
#[doc = "DMA_CHx_RX_INT_WDTIMER register accessor: an alias for `Reg<DMA_CHX_RX_INT_WDTIMER_SPEC>`"]
pub type DMA_CHX_RX_INT_WDTIMER = crate::Reg<dma_chx_rx_int_wdtimer::DMA_CHX_RX_INT_WDTIMER_SPEC>;
#[doc = "Receive Interrupt Watchdog Timer"]
pub mod dma_chx_rx_int_wdtimer;
#[doc = "DMA_CHx_SLOT_FUNC_CTRL_STAT register accessor: an alias for `Reg<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>`"]
pub type DMA_CHX_SLOT_FUNC_CTRL_STAT =
    crate::Reg<dma_chx_slot_func_ctrl_stat::DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>;
#[doc = "Slot Function Control and Status"]
pub mod dma_chx_slot_func_ctrl_stat;
#[doc = "DMA_CHx_CUR_HST_TXDESC register accessor: an alias for `Reg<DMA_CHX_CUR_HST_TXDESC_SPEC>`"]
pub type DMA_CHX_CUR_HST_TXDESC = crate::Reg<dma_chx_cur_hst_txdesc::DMA_CHX_CUR_HST_TXDESC_SPEC>;
#[doc = "Channelx Current Host Transmit descriptor"]
pub mod dma_chx_cur_hst_txdesc;
#[doc = "DMA_CHx_CUR_HST_RXDESC register accessor: an alias for `Reg<DMA_CHX_CUR_HST_RXDESC_SPEC>`"]
pub type DMA_CHX_CUR_HST_RXDESC = crate::Reg<dma_chx_cur_hst_rxdesc::DMA_CHX_CUR_HST_RXDESC_SPEC>;
#[doc = "no description available"]
pub mod dma_chx_cur_hst_rxdesc;
#[doc = "DMA_CHx_CUR_HST_TXBUF register accessor: an alias for `Reg<DMA_CHX_CUR_HST_TXBUF_SPEC>`"]
pub type DMA_CHX_CUR_HST_TXBUF = crate::Reg<dma_chx_cur_hst_txbuf::DMA_CHX_CUR_HST_TXBUF_SPEC>;
#[doc = "no description available"]
pub mod dma_chx_cur_hst_txbuf;
#[doc = "DMA_CHx_CUR_HST_RXBUF register accessor: an alias for `Reg<DMA_CHX_CUR_HST_RXBUF_SPEC>`"]
pub type DMA_CHX_CUR_HST_RXBUF = crate::Reg<dma_chx_cur_hst_rxbuf::DMA_CHX_CUR_HST_RXBUF_SPEC>;
#[doc = "Channelx Current Application Receive Buffer Address"]
pub mod dma_chx_cur_hst_rxbuf;
#[doc = "DMA_CHx_STAT register accessor: an alias for `Reg<DMA_CHX_STAT_SPEC>`"]
pub type DMA_CHX_STAT = crate::Reg<dma_chx_stat::DMA_CHX_STAT_SPEC>;
#[doc = "Channelx DMA status register"]
pub mod dma_chx_stat;
