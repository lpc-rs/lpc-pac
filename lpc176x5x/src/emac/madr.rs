#[doc = "Reader of register MADR"]
pub type R = crate::R<u32, super::MADR>;
#[doc = "Writer for register MADR"]
pub type W = crate::W<u32, super::MADR>;
#[doc = "Register MADR `reset()`'s with value 0"]
impl crate::ResetValue for super::MADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGADDR`"]
pub type REGADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGADDR`"]
pub struct REGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PHYADDR`"]
pub type PHYADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHYADDR`"]
pub struct PHYADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
    #[inline(always)]
    pub fn phyaddr(&self) -> PHYADDR_R {
        PHYADDR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
    #[inline(always)]
    pub fn regaddr(&mut self) -> REGADDR_W {
        REGADDR_W { w: self }
    }
    #[doc = "Bits 8:12 - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
    #[inline(always)]
    pub fn phyaddr(&mut self) -> PHYADDR_W {
        PHYADDR_W { w: self }
    }
}
