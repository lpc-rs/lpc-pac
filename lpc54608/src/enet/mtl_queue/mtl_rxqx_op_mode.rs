#[doc = "Reader of register MTL_RXQx_OP_MODE"]
pub type R = crate::R<u32, super::MTL_RXQX_OP_MODE>;
#[doc = "Writer for register MTL_RXQx_OP_MODE"]
pub type W = crate::W<u32, super::MTL_RXQX_OP_MODE>;
#[doc = "Register MTL_RXQx_OP_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_RXQX_OP_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FUP`"]
pub type FUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUP`"]
pub struct FUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FUP_W<'a> {
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
#[doc = "Reader of field `FEP`"]
pub type FEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEP`"]
pub struct FEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RSF`"]
pub type RSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSF`"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DIS_TCP_EF`"]
pub type DIS_TCP_EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_TCP_EF`"]
pub struct DIS_TCP_EF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TCP_EF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RQS`"]
pub type RQS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RQS`"]
pub struct RQS_W<'a> {
    w: &'a mut W,
}
impl<'a> RQS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W {
        FUP_W { w: self }
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&mut self) -> FEP_W {
        FEP_W { w: self }
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W {
        DIS_TCP_EF_W { w: self }
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&mut self) -> RQS_W {
        RQS_W { w: self }
    }
}
