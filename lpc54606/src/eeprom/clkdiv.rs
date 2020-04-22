#[doc = "Reader of register CLKDIV"]
pub type R = crate::R<u32, super::CLKDIV>;
#[doc = "Writer for register CLKDIV"]
pub type W = crate::W<u32, super::CLKDIV>;
#[doc = "Register CLKDIV `reset()`'s with value 0x63"]
impl crate::ResetValue for super::CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x63
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Division factor (minus 1 encoded)."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Division factor (minus 1 encoded)."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
}
