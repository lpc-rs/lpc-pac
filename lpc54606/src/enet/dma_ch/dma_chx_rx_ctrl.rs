#[doc = "Reader of register DMA_CHx_RX_CTRL"]
pub type R = crate::R<u32, super::DMA_CHX_RX_CTRL>;
#[doc = "Writer for register DMA_CHx_RX_CTRL"]
pub type W = crate::W<u32, super::DMA_CHX_RX_CTRL>;
#[doc = "Register DMA_CHx_RX_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_RX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
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
#[doc = "Reader of field `RBSZ`"]
pub type RBSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RBSZ`"]
pub struct RBSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RBSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 3)) | (((value as u32) & 0x0fff) << 3);
        self.w
    }
}
#[doc = "Reader of field `RxPBL`"]
pub type RXPBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RxPBL`"]
pub struct RXPBL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RPF`"]
pub type RPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPF`"]
pub struct RPF_W<'a> {
    w: &'a mut W,
}
impl<'a> RPF_W<'a> {
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
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 3) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn rx_pbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bits 3:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W {
        RBSZ_W { w: self }
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn rx_pbl(&mut self) -> RXPBL_W {
        RXPBL_W { w: self }
    }
    #[doc = "Bit 31 - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W {
        RPF_W { w: self }
    }
}
