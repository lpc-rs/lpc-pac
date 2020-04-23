#[doc = "Reader of register CARDTHRCTL"]
pub type R = crate::R<u32, super::CARDTHRCTL>;
#[doc = "Writer for register CARDTHRCTL"]
pub type W = crate::W<u32, super::CARDTHRCTL>;
#[doc = "Register CARDTHRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CARDTHRCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARDRDTHREN`"]
pub type CARDRDTHREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDRDTHREN`"]
pub struct CARDRDTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDRDTHREN_W<'a> {
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
#[doc = "Reader of field `BSYCLRINTEN`"]
pub type BSYCLRINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSYCLRINTEN`"]
pub struct BSYCLRINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSYCLRINTEN_W<'a> {
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
#[doc = "Reader of field `CARDTHRESHOLD`"]
pub type CARDTHRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARDTHRESHOLD`"]
pub struct CARDTHRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDTHRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CARDRDTHREN_R {
        CARDRDTHREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub fn bsyclrinten(&self) -> BSYCLRINTEN_R {
        BSYCLRINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    pub fn cardthreshold(&self) -> CARDTHRESHOLD_R {
        CARDTHRESHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&mut self) -> CARDRDTHREN_W {
        CARDRDTHREN_W { w: self }
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub fn bsyclrinten(&mut self) -> BSYCLRINTEN_W {
        BSYCLRINTEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    pub fn cardthreshold(&mut self) -> CARDTHRESHOLD_W {
        CARDTHRESHOLD_W { w: self }
    }
}
