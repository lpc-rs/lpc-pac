#[doc = "Reader of register MTL_RXQ_DMA_MAP"]
pub type R = crate::R<u32, super::MTL_RXQ_DMA_MAP>;
#[doc = "Writer for register MTL_RXQ_DMA_MAP"]
pub type W = crate::W<u32, super::MTL_RXQ_DMA_MAP>;
#[doc = "Register MTL_RXQ_DMA_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_RXQ_DMA_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Q0MDMACH`"]
pub type Q0MDMACH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Q0MDMACH`"]
pub struct Q0MDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q0MDMACH_W<'a> {
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
#[doc = "Reader of field `Q0DDMACH`"]
pub type Q0DDMACH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Q0DDMACH`"]
pub struct Q0DDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q0DDMACH_W<'a> {
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
#[doc = "Reader of field `Q1MDMACH`"]
pub type Q1MDMACH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Q1MDMACH`"]
pub struct Q1MDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q1MDMACH_W<'a> {
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
#[doc = "Reader of field `Q1DDMACH`"]
pub type Q1DDMACH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Q1DDMACH`"]
pub struct Q1DDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q1DDMACH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
    #[inline(always)]
    pub fn q0mdmach(&self) -> Q0MDMACH_R {
        Q0MDMACH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q0ddmach(&self) -> Q0DDMACH_R {
        Q0DDMACH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
    #[inline(always)]
    pub fn q1mdmach(&self) -> Q1MDMACH_R {
        Q1MDMACH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q1ddmach(&self) -> Q1DDMACH_R {
        Q1DDMACH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
    #[inline(always)]
    pub fn q0mdmach(&mut self) -> Q0MDMACH_W {
        Q0MDMACH_W { w: self }
    }
    #[doc = "Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q0ddmach(&mut self) -> Q0DDMACH_W {
        Q0DDMACH_W { w: self }
    }
    #[doc = "Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
    #[inline(always)]
    pub fn q1mdmach(&mut self) -> Q1MDMACH_W {
        Q1MDMACH_W { w: self }
    }
    #[doc = "Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q1ddmach(&mut self) -> Q1DDMACH_W {
        Q1DDMACH_W { w: self }
    }
}
