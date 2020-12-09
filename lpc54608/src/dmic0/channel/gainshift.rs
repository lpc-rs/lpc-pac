#[doc = "Reader of register GAINSHIFT"]
pub type R = crate::R<u32, super::GAINSHIFT>;
#[doc = "Writer for register GAINSHIFT"]
pub type W = crate::W<u32, super::GAINSHIFT>;
#[doc = "Register GAINSHIFT `reset()`'s with value 0"]
impl crate::ResetValue for super::GAINSHIFT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Gain control, as a positive or negative (two's complement) number of bits to shift."]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Gain control, as a positive or negative (two's complement) number of bits to shift."]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
}
