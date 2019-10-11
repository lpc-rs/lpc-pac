#[doc = "Reader of register AYRS"]
pub type R = crate::R<u32, super::AYRS>;
#[doc = "Writer for register AYRS"]
pub type W = crate::W<u32, super::AYRS>;
#[doc = "Register AYRS `reset()`'s with value 0"]
impl crate::ResetValue for super::AYRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `YEAR`"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
}
