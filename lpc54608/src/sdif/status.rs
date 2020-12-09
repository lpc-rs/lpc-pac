#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0x0406"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0406
    }
}
#[doc = "Reader of field `FIFO_RX_WATERMARK`"]
pub type FIFO_RX_WATERMARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_RX_WATERMARK`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `FIFO_TX_WATERMARK`"]
pub type FIFO_TX_WATERMARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_TX_WATERMARK`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FIFO_EMPTY`"]
pub type FIFO_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_EMPTY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FIFO_FULL`"]
pub type FIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_FULL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CMDFSMSTATES`"]
pub type CMDFSMSTATES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDFSMSTATES`"]
pub struct CMDFSMSTATES_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDFSMSTATES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DATA_3_STATUS`"]
pub type DATA_3_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_3_STATUS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_BUSY`"]
pub type DATA_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_BUSY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DATA_STATE_MC_BUSY`"]
pub type DATA_STATE_MC_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_STATE_MC_BUSY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RESPONSE_INDEX`"]
pub type RESPONSE_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESPONSE_INDEX`"]
pub struct RESPONSE_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | (((value as u32) & 0x3f) << 11);
        self.w
    }
}
#[doc = "Reader of field `FIFO_COUNT`"]
pub type FIFO_COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FIFO_COUNT`"]
pub struct FIFO_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 17)) | (((value as u32) & 0x1fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `DMA_ACK`"]
pub type DMA_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ACK`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DMA_REQ`"]
pub type DMA_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQ`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
