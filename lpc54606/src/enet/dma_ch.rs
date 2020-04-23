#[doc = "DMA Channelx Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_ctrl](dma_chx_ctrl) module"]
pub type DMA_CHX_CTRL = crate::Reg<u32, _DMA_CHX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_CTRL;
#[doc = "`read()` method returns [dma_chx_ctrl::R](dma_chx_ctrl::R) reader structure"]
impl crate::Readable for DMA_CHX_CTRL {}
#[doc = "`write(|w| ..)` method takes [dma_chx_ctrl::W](dma_chx_ctrl::W) writer structure"]
impl crate::Writable for DMA_CHX_CTRL {}
#[doc = "DMA Channelx Control"]
pub mod dma_chx_ctrl;
#[doc = "DMA Channelx Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_tx_ctrl](dma_chx_tx_ctrl) module"]
pub type DMA_CHX_TX_CTRL = crate::Reg<u32, _DMA_CHX_TX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_TX_CTRL;
#[doc = "`read()` method returns [dma_chx_tx_ctrl::R](dma_chx_tx_ctrl::R) reader structure"]
impl crate::Readable for DMA_CHX_TX_CTRL {}
#[doc = "`write(|w| ..)` method takes [dma_chx_tx_ctrl::W](dma_chx_tx_ctrl::W) writer structure"]
impl crate::Writable for DMA_CHX_TX_CTRL {}
#[doc = "DMA Channelx Transmit Control"]
pub mod dma_chx_tx_ctrl;
#[doc = "DMA Channelx Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_rx_ctrl](dma_chx_rx_ctrl) module"]
pub type DMA_CHX_RX_CTRL = crate::Reg<u32, _DMA_CHX_RX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_RX_CTRL;
#[doc = "`read()` method returns [dma_chx_rx_ctrl::R](dma_chx_rx_ctrl::R) reader structure"]
impl crate::Readable for DMA_CHX_RX_CTRL {}
#[doc = "`write(|w| ..)` method takes [dma_chx_rx_ctrl::W](dma_chx_rx_ctrl::W) writer structure"]
impl crate::Writable for DMA_CHX_RX_CTRL {}
#[doc = "DMA Channelx Receive Control"]
pub mod dma_chx_rx_ctrl;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_txdesc_list_addr](dma_chx_txdesc_list_addr) module"]
pub type DMA_CHX_TXDESC_LIST_ADDR = crate::Reg<u32, _DMA_CHX_TXDESC_LIST_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_TXDESC_LIST_ADDR;
#[doc = "`read()` method returns [dma_chx_txdesc_list_addr::R](dma_chx_txdesc_list_addr::R) reader structure"]
impl crate::Readable for DMA_CHX_TXDESC_LIST_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_chx_txdesc_list_addr::W](dma_chx_txdesc_list_addr::W) writer structure"]
impl crate::Writable for DMA_CHX_TXDESC_LIST_ADDR {}
#[doc = "no description available"]
pub mod dma_chx_txdesc_list_addr;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_rxdesc_list_addr](dma_chx_rxdesc_list_addr) module"]
pub type DMA_CHX_RXDESC_LIST_ADDR = crate::Reg<u32, _DMA_CHX_RXDESC_LIST_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_RXDESC_LIST_ADDR;
#[doc = "`read()` method returns [dma_chx_rxdesc_list_addr::R](dma_chx_rxdesc_list_addr::R) reader structure"]
impl crate::Readable for DMA_CHX_RXDESC_LIST_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_chx_rxdesc_list_addr::W](dma_chx_rxdesc_list_addr::W) writer structure"]
impl crate::Writable for DMA_CHX_RXDESC_LIST_ADDR {}
#[doc = "no description available"]
pub mod dma_chx_rxdesc_list_addr;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_txdesc_tail_ptr](dma_chx_txdesc_tail_ptr) module"]
pub type DMA_CHX_TXDESC_TAIL_PTR = crate::Reg<u32, _DMA_CHX_TXDESC_TAIL_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_TXDESC_TAIL_PTR;
#[doc = "`read()` method returns [dma_chx_txdesc_tail_ptr::R](dma_chx_txdesc_tail_ptr::R) reader structure"]
impl crate::Readable for DMA_CHX_TXDESC_TAIL_PTR {}
#[doc = "`write(|w| ..)` method takes [dma_chx_txdesc_tail_ptr::W](dma_chx_txdesc_tail_ptr::W) writer structure"]
impl crate::Writable for DMA_CHX_TXDESC_TAIL_PTR {}
#[doc = "no description available"]
pub mod dma_chx_txdesc_tail_ptr;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_rxdesc_tail_ptr](dma_chx_rxdesc_tail_ptr) module"]
pub type DMA_CHX_RXDESC_TAIL_PTR = crate::Reg<u32, _DMA_CHX_RXDESC_TAIL_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_RXDESC_TAIL_PTR;
#[doc = "`read()` method returns [dma_chx_rxdesc_tail_ptr::R](dma_chx_rxdesc_tail_ptr::R) reader structure"]
impl crate::Readable for DMA_CHX_RXDESC_TAIL_PTR {}
#[doc = "`write(|w| ..)` method takes [dma_chx_rxdesc_tail_ptr::W](dma_chx_rxdesc_tail_ptr::W) writer structure"]
impl crate::Writable for DMA_CHX_RXDESC_TAIL_PTR {}
#[doc = "no description available"]
pub mod dma_chx_rxdesc_tail_ptr;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_txdesc_ring_length](dma_chx_txdesc_ring_length) module"]
pub type DMA_CHX_TXDESC_RING_LENGTH = crate::Reg<u32, _DMA_CHX_TXDESC_RING_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_TXDESC_RING_LENGTH;
#[doc = "`read()` method returns [dma_chx_txdesc_ring_length::R](dma_chx_txdesc_ring_length::R) reader structure"]
impl crate::Readable for DMA_CHX_TXDESC_RING_LENGTH {}
#[doc = "`write(|w| ..)` method takes [dma_chx_txdesc_ring_length::W](dma_chx_txdesc_ring_length::W) writer structure"]
impl crate::Writable for DMA_CHX_TXDESC_RING_LENGTH {}
#[doc = "no description available"]
pub mod dma_chx_txdesc_ring_length;
#[doc = "Channelx Rx descriptor Ring Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_rxdesc_ring_length](dma_chx_rxdesc_ring_length) module"]
pub type DMA_CHX_RXDESC_RING_LENGTH = crate::Reg<u32, _DMA_CHX_RXDESC_RING_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_RXDESC_RING_LENGTH;
#[doc = "`read()` method returns [dma_chx_rxdesc_ring_length::R](dma_chx_rxdesc_ring_length::R) reader structure"]
impl crate::Readable for DMA_CHX_RXDESC_RING_LENGTH {}
#[doc = "`write(|w| ..)` method takes [dma_chx_rxdesc_ring_length::W](dma_chx_rxdesc_ring_length::W) writer structure"]
impl crate::Writable for DMA_CHX_RXDESC_RING_LENGTH {}
#[doc = "Channelx Rx descriptor Ring Length"]
pub mod dma_chx_rxdesc_ring_length;
#[doc = "Channelx Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_int_en](dma_chx_int_en) module"]
pub type DMA_CHX_INT_EN = crate::Reg<u32, _DMA_CHX_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_INT_EN;
#[doc = "`read()` method returns [dma_chx_int_en::R](dma_chx_int_en::R) reader structure"]
impl crate::Readable for DMA_CHX_INT_EN {}
#[doc = "`write(|w| ..)` method takes [dma_chx_int_en::W](dma_chx_int_en::W) writer structure"]
impl crate::Writable for DMA_CHX_INT_EN {}
#[doc = "Channelx Interrupt Enable"]
pub mod dma_chx_int_en;
#[doc = "Receive Interrupt Watchdog Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_rx_int_wdtimer](dma_chx_rx_int_wdtimer) module"]
pub type DMA_CHX_RX_INT_WDTIMER = crate::Reg<u32, _DMA_CHX_RX_INT_WDTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_RX_INT_WDTIMER;
#[doc = "`read()` method returns [dma_chx_rx_int_wdtimer::R](dma_chx_rx_int_wdtimer::R) reader structure"]
impl crate::Readable for DMA_CHX_RX_INT_WDTIMER {}
#[doc = "`write(|w| ..)` method takes [dma_chx_rx_int_wdtimer::W](dma_chx_rx_int_wdtimer::W) writer structure"]
impl crate::Writable for DMA_CHX_RX_INT_WDTIMER {}
#[doc = "Receive Interrupt Watchdog Timer"]
pub mod dma_chx_rx_int_wdtimer;
#[doc = "Slot Function Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_slot_func_ctrl_stat](dma_chx_slot_func_ctrl_stat) module"]
pub type DMA_CHX_SLOT_FUNC_CTRL_STAT = crate::Reg<u32, _DMA_CHX_SLOT_FUNC_CTRL_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_SLOT_FUNC_CTRL_STAT;
#[doc = "`read()` method returns [dma_chx_slot_func_ctrl_stat::R](dma_chx_slot_func_ctrl_stat::R) reader structure"]
impl crate::Readable for DMA_CHX_SLOT_FUNC_CTRL_STAT {}
#[doc = "`write(|w| ..)` method takes [dma_chx_slot_func_ctrl_stat::W](dma_chx_slot_func_ctrl_stat::W) writer structure"]
impl crate::Writable for DMA_CHX_SLOT_FUNC_CTRL_STAT {}
#[doc = "Slot Function Control and Status"]
pub mod dma_chx_slot_func_ctrl_stat;
#[doc = "Channelx Current Host Transmit descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_cur_hst_txdesc](dma_chx_cur_hst_txdesc) module"]
pub type DMA_CHX_CUR_HST_TXDESC = crate::Reg<u32, _DMA_CHX_CUR_HST_TXDESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_CUR_HST_TXDESC;
#[doc = "`read()` method returns [dma_chx_cur_hst_txdesc::R](dma_chx_cur_hst_txdesc::R) reader structure"]
impl crate::Readable for DMA_CHX_CUR_HST_TXDESC {}
#[doc = "Channelx Current Host Transmit descriptor"]
pub mod dma_chx_cur_hst_txdesc;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_cur_hst_rxdesc](dma_chx_cur_hst_rxdesc) module"]
pub type DMA_CHX_CUR_HST_RXDESC = crate::Reg<u32, _DMA_CHX_CUR_HST_RXDESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_CUR_HST_RXDESC;
#[doc = "`read()` method returns [dma_chx_cur_hst_rxdesc::R](dma_chx_cur_hst_rxdesc::R) reader structure"]
impl crate::Readable for DMA_CHX_CUR_HST_RXDESC {}
#[doc = "no description available"]
pub mod dma_chx_cur_hst_rxdesc;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_cur_hst_txbuf](dma_chx_cur_hst_txbuf) module"]
pub type DMA_CHX_CUR_HST_TXBUF = crate::Reg<u32, _DMA_CHX_CUR_HST_TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_CUR_HST_TXBUF;
#[doc = "`read()` method returns [dma_chx_cur_hst_txbuf::R](dma_chx_cur_hst_txbuf::R) reader structure"]
impl crate::Readable for DMA_CHX_CUR_HST_TXBUF {}
#[doc = "no description available"]
pub mod dma_chx_cur_hst_txbuf;
#[doc = "Channelx Current Application Receive Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_cur_hst_rxbuf](dma_chx_cur_hst_rxbuf) module"]
pub type DMA_CHX_CUR_HST_RXBUF = crate::Reg<u32, _DMA_CHX_CUR_HST_RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_CUR_HST_RXBUF;
#[doc = "`read()` method returns [dma_chx_cur_hst_rxbuf::R](dma_chx_cur_hst_rxbuf::R) reader structure"]
impl crate::Readable for DMA_CHX_CUR_HST_RXBUF {}
#[doc = "Channelx Current Application Receive Buffer Address"]
pub mod dma_chx_cur_hst_rxbuf;
#[doc = "Channelx DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_stat](dma_chx_stat) module"]
pub type DMA_CHX_STAT = crate::Reg<u32, _DMA_CHX_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CHX_STAT;
#[doc = "`read()` method returns [dma_chx_stat::R](dma_chx_stat::R) reader structure"]
impl crate::Readable for DMA_CHX_STAT {}
#[doc = "`write(|w| ..)` method takes [dma_chx_stat::W](dma_chx_stat::W) writer structure"]
impl crate::Writable for DMA_CHX_STAT {}
#[doc = "Channelx DMA status register"]
pub mod dma_chx_stat;
