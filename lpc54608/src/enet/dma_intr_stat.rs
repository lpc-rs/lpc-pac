#[doc = "Reader of register DMA_INTR_STAT"]
pub type R = crate::R<u32, super::DMA_INTR_STAT>;
#[doc = "Writer for register DMA_INTR_STAT"]
pub type W = crate::W<u32, super::DMA_INTR_STAT>;
#[doc = "Register DMA_INTR_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INTR_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DC0IS`"]
pub type DC0IS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DC0IS`"]
pub struct DC0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DC0IS_W<'a> {
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
#[doc = "Reader of field `DC1IS`"]
pub type DC1IS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DC1IS`"]
pub struct DC1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DC1IS_W<'a> {
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
#[doc = "Reader of field `MTLIS`"]
pub type MTLIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MACIS`"]
pub type MACIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
    #[inline(always)]
    pub fn dc1is(&self) -> DC1IS_R {
        DC1IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status This bit indicates an interrupt event in the MTL."]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status This bit indicates an interrupt event in the MAC."]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
    #[inline(always)]
    pub fn dc0is(&mut self) -> DC0IS_W {
        DC0IS_W { w: self }
    }
    #[doc = "Bit 1 - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
    #[inline(always)]
    pub fn dc1is(&mut self) -> DC1IS_W {
        DC1IS_W { w: self }
    }
}
