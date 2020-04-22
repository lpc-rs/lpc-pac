#[doc = "Reader of register MAC_ADDR_HIGH"]
pub type R = crate::R<u32, super::MAC_ADDR_HIGH>;
#[doc = "Writer for register MAC_ADDR_HIGH"]
pub type W = crate::W<u32, super::MAC_ADDR_HIGH>;
#[doc = "Register MAC_ADDR_HIGH `reset()`'s with value 0x8000_ffff"]
impl crate::ResetValue for super::MAC_ADDR_HIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_ffff
    }
}
#[doc = "Reader of field `A47_32`"]
pub type A47_32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `A47_32`"]
pub struct A47_32_W<'a> {
    w: &'a mut W,
}
impl<'a> A47_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DCS`"]
pub type DCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCS`"]
pub struct DCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&self) -> A47_32_R {
        A47_32_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Address Enable."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&mut self) -> A47_32_W {
        A47_32_W { w: self }
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&mut self) -> DCS_W {
        DCS_W { w: self }
    }
}
