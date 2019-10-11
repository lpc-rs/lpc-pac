#[doc = "Reader of register FRGMULT"]
pub type R = crate::R<u32, super::FRGMULT>;
#[doc = "Writer for register FRGMULT"]
pub type W = crate::W<u32, super::FRGMULT>;
#[doc = "Register FRGMULT `reset()`'s with value 0"]
impl crate::ResetValue for super::FRGMULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MULT`"]
pub type MULT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MULT`"]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
}
