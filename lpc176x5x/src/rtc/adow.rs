#[doc = "Reader of register ADOW"]
pub type R = crate::R<u32, super::ADOW>;
#[doc = "Writer for register ADOW"]
pub type W = crate::W<u32, super::ADOW>;
#[doc = "Register ADOW `reset()`'s with value 0"]
impl crate::ResetValue for super::ADOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOW`"]
pub type DOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOW`"]
pub struct DOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&mut self) -> DOW_W {
        DOW_W { w: self }
    }
}
