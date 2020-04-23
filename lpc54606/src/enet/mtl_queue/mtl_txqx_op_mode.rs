#[doc = "Reader of register MTL_TXQx_OP_MODE"]
pub type R = crate::R<u32, super::MTL_TXQX_OP_MODE>;
#[doc = "Writer for register MTL_TXQx_OP_MODE"]
pub type W = crate::W<u32, super::MTL_TXQX_OP_MODE>;
#[doc = "Register MTL_TXQx_OP_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_TXQX_OP_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FTQ`"]
pub type FTQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTQ`"]
pub struct FTQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FTQ_W<'a> {
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
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSF`"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
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
#[doc = "Reader of field `TXQEN`"]
pub type TXQEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXQEN`"]
pub struct TXQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXQEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TTC`"]
pub type TTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TTC`"]
pub struct TTC_W<'a> {
    w: &'a mut W,
}
impl<'a> TTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `TQS`"]
pub type TQS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TQS`"]
pub struct TQS_W<'a> {
    w: &'a mut W,
}
impl<'a> TQS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W {
        FTQ_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&mut self) -> TXQEN_W {
        TXQEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W {
        TTC_W { w: self }
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W {
        TQS_W { w: self }
    }
}
