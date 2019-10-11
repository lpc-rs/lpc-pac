#[doc = "Reader of register AMON"]
pub type R = crate::R<u32, super::AMON>;
#[doc = "Writer for register AMON"]
pub type W = crate::W<u32, super::AMON>;
#[doc = "Register AMON `reset()`'s with value 0"]
impl crate::ResetValue for super::AMON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MONTH`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONTH`"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
}
