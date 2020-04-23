#[doc = "Reader of register DMA_DBG_STAT"]
pub type R = crate::R<u32, super::DMA_DBG_STAT>;
#[doc = "Writer for register DMA_DBG_STAT"]
pub type W = crate::W<u32, super::DMA_DBG_STAT>;
#[doc = "Register DMA_DBG_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_DBG_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AHSTS`"]
pub type AHSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHSTS`"]
pub struct AHSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> AHSTS_W<'a> {
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
#[doc = "Reader of field `RPS0`"]
pub type RPS0_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPS0`"]
pub type TPS0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RPS1`"]
pub type RPS1_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPS1`"]
pub type TPS1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&self) -> AHSTS_R {
        AHSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0: 0x0: Stopped (Reset or Stop Receive Command issued) 0x1: Running (Fetching Rx Transfer ) 0x2: Reserved 0x3: Running (Waiting for Rx packet) 0x4: Suspended (Rx Unavailable) 0x5: Running (Closing the Rx) 0x6: Timestamp write state 0x7: Running (Transferring the received packet data from the Rx buffer to the system memory) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0: 000: Stopped (Reset or Stop Transmit Command issued) 0x1: Running (Fetching Tx Transfer) 0x2: Running (Waiting for status) 0x3: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO)) 0x4: Timestamp write state 0x5: Reserved for future use 0x6: Suspended (Tx Unavailable or Tx Buffer Underflow) 0x7: Running (Closing Tx ) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&mut self) -> AHSTS_W {
        AHSTS_W { w: self }
    }
}
