#[doc = "Reader of register ADOM"]
pub type R = crate::R<u32, super::ADOM>;
#[doc = "Writer for register ADOM"]
pub type W = crate::W<u32, super::ADOM>;
#[doc = "Register ADOM `reset()`'s with value 0"]
impl crate::ResetValue for super::ADOM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOM`"]
pub type DOM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOM`"]
pub struct DOM_W<'a> {
    w: &'a mut W,
}
impl<'a> DOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&mut self) -> DOM_W {
        DOM_W { w: self }
    }
}
