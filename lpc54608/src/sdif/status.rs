#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATUS_SPEC>> for R {
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<STATUS_SPEC>> for W {
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_RX_WATERMARK` reader - FIFO reached Receive watermark level; not qualified with data transfer."]
pub struct FIFO_RX_WATERMARK_R(crate::FieldReader<bool, bool>);
impl FIFO_RX_WATERMARK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_RX_WATERMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_RX_WATERMARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_RX_WATERMARK` writer - FIFO reached Receive watermark level; not qualified with data transfer."]
pub struct FIFO_RX_WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RX_WATERMARK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FIFO_TX_WATERMARK` reader - FIFO reached Transmit watermark level; not qualified with data transfer."]
pub struct FIFO_TX_WATERMARK_R(crate::FieldReader<bool, bool>);
impl FIFO_TX_WATERMARK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_TX_WATERMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_TX_WATERMARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_TX_WATERMARK` writer - FIFO reached Transmit watermark level; not qualified with data transfer."]
pub struct FIFO_TX_WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_TX_WATERMARK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FIFO_EMPTY` reader - FIFO is empty status."]
pub struct FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl FIFO_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_EMPTY` writer - FIFO is empty status."]
pub struct FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_EMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FIFO_FULL` reader - FIFO is full status."]
pub struct FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl FIFO_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_FULL` writer - FIFO is full status."]
pub struct FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CMDFSMSTATES` reader - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
pub struct CMDFSMSTATES_R(crate::FieldReader<u8, u8>);
impl CMDFSMSTATES_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMDFSMSTATES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDFSMSTATES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDFSMSTATES` writer - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
pub struct CMDFSMSTATES_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDFSMSTATES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DATA_3_STATUS` reader - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
pub struct DATA_3_STATUS_R(crate::FieldReader<bool, bool>);
impl DATA_3_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_3_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_3_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_3_STATUS` writer - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
pub struct DATA_3_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_3_STATUS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DATA_BUSY` reader - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
pub struct DATA_BUSY_R(crate::FieldReader<bool, bool>);
impl DATA_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BUSY` writer - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
pub struct DATA_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BUSY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DATA_STATE_MC_BUSY` reader - Data transmit or receive state-machine is busy."]
pub struct DATA_STATE_MC_BUSY_R(crate::FieldReader<bool, bool>);
impl DATA_STATE_MC_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_STATE_MC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_STATE_MC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_STATE_MC_BUSY` writer - Data transmit or receive state-machine is busy."]
pub struct DATA_STATE_MC_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_STATE_MC_BUSY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RESPONSE_INDEX` reader - Index of previous response, including any auto-stop sent by core."]
pub struct RESPONSE_INDEX_R(crate::FieldReader<u8, u8>);
impl RESPONSE_INDEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESPONSE_INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE_INDEX` writer - Index of previous response, including any auto-stop sent by core."]
pub struct RESPONSE_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | ((value as u32 & 0x3f) << 11);
        self.w
    }
}
#[doc = "Field `FIFO_COUNT` reader - FIFO count - Number of filled locations in FIFO."]
pub struct FIFO_COUNT_R(crate::FieldReader<u16, u16>);
impl FIFO_COUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        FIFO_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_COUNT` writer - FIFO count - Number of filled locations in FIFO."]
pub struct FIFO_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 17)) | ((value as u32 & 0x1fff) << 17);
        self.w
    }
}
#[doc = "Field `DMA_ACK` reader - DMA acknowledge signal state."]
pub struct DMA_ACK_R(crate::FieldReader<bool, bool>);
impl DMA_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_ACK` writer - DMA acknowledge signal state."]
pub struct DMA_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DMA_REQ` reader - DMA request signal state."]
pub struct DMA_REQ_R(crate::FieldReader<bool, bool>);
impl DMA_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_REQ` writer - DMA request signal state."]
pub struct DMA_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FIFO_RX_WATERMARK_R {
        FIFO_RX_WATERMARK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> FIFO_TX_WATERMARK_R {
        FIFO_TX_WATERMARK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub fn cmdfsmstates(&self) -> CMDFSMSTATES_R {
        CMDFSMSTATES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub fn data_3_status(&self) -> DATA_3_STATUS_R {
        DATA_3_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub fn data_busy(&self) -> DATA_BUSY_R {
        DATA_BUSY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> DATA_STATE_MC_BUSY_R {
        DATA_STATE_MC_BUSY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&self) -> RESPONSE_INDEX_R {
        RESPONSE_INDEX_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&self) -> FIFO_COUNT_R {
        FIFO_COUNT_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    pub fn dma_ack(&self) -> DMA_ACK_R {
        DMA_ACK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&mut self) -> FIFO_RX_WATERMARK_W {
        FIFO_RX_WATERMARK_W { w: self }
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&mut self) -> FIFO_TX_WATERMARK_W {
        FIFO_TX_WATERMARK_W { w: self }
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&mut self) -> FIFO_EMPTY_W {
        FIFO_EMPTY_W { w: self }
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> FIFO_FULL_W {
        FIFO_FULL_W { w: self }
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub fn cmdfsmstates(&mut self) -> CMDFSMSTATES_W {
        CMDFSMSTATES_W { w: self }
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub fn data_3_status(&mut self) -> DATA_3_STATUS_W {
        DATA_3_STATUS_W { w: self }
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub fn data_busy(&mut self) -> DATA_BUSY_W {
        DATA_BUSY_W { w: self }
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&mut self) -> DATA_STATE_MC_BUSY_W {
        DATA_STATE_MC_BUSY_W { w: self }
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&mut self) -> RESPONSE_INDEX_W {
        RESPONSE_INDEX_W { w: self }
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&mut self) -> FIFO_COUNT_W {
        FIFO_COUNT_W { w: self }
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    pub fn dma_ack(&mut self) -> DMA_ACK_W {
        DMA_ACK_W { w: self }
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    pub fn dma_req(&mut self) -> DMA_REQ_W {
        DMA_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0x0406"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0406
    }
}
