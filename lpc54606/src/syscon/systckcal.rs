#[doc = "Reader of register SYSTCKCAL"]
pub type R = crate::R<u32, super::SYSTCKCAL>;
#[doc = "Writer for register SYSTCKCAL"]
pub type W = crate::W<u32, super::SYSTCKCAL>;
#[doc = "Register SYSTCKCAL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTCKCAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CAL`"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SKEW`"]
pub struct SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOREF`"]
pub struct NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - System tick timer calibration value."]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - System tick timer calibration value."]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
}
