#[doc = "Reader of register MAC_RXQ_CTRL1"]
pub type R = crate::R<u32, super::MAC_RXQ_CTRL1>;
#[doc = "Writer for register MAC_RXQ_CTRL1"]
pub type W = crate::W<u32, super::MAC_RXQ_CTRL1>;
#[doc = "Register MAC_RXQ_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_RXQ_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AVCPQ`"]
pub type AVCPQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AVCPQ`"]
pub struct AVCPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AVCPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `AVPTPQ`"]
pub type AVPTPQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AVPTPQ`"]
pub struct AVPTPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AVPTPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `UPQ`"]
pub type UPQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UPQ`"]
pub struct UPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> UPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `MCBCQ`"]
pub type MCBCQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCBCQ`"]
pub struct MCBCQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCBCQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `MCBCQEN`"]
pub type MCBCQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCBCQEN`"]
pub struct MCBCQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCBCQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&mut self) -> AVCPQ_W {
        AVCPQ_W { w: self }
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&mut self) -> AVPTPQ_W {
        AVPTPQ_W { w: self }
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&mut self) -> UPQ_W {
        UPQ_W { w: self }
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&mut self) -> MCBCQ_W {
        MCBCQ_W { w: self }
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W {
        MCBCQEN_W { w: self }
    }
}
