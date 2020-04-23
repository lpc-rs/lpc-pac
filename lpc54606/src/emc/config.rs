#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM`"]
pub type EM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM`"]
pub struct EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_W<'a> {
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
#[doc = "Reader of field `CLKR`"]
pub type CLKR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKR`"]
pub struct CLKR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Endian mode."]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&self) -> CLKR_R {
        CLKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endian mode."]
    #[inline(always)]
    pub fn em(&mut self) -> EM_W {
        EM_W { w: self }
    }
    #[doc = "Bit 8 - This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&mut self) -> CLKR_W {
        CLKR_W { w: self }
    }
}
