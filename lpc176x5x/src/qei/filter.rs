#[doc = "Reader of register FILTER"]
pub type R = crate::R<u32, super::FILTER>;
#[doc = "Writer for register FILTER"]
pub type W = crate::W<u32, super::FILTER>;
#[doc = "Register FILTER `reset()`'s with value 0"]
impl crate::ResetValue for super::FILTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILTA`"]
pub type FILTA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FILTA`"]
pub struct FILTA_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay."]
    #[inline(always)]
    pub fn filta(&self) -> FILTA_R {
        FILTA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay."]
    #[inline(always)]
    pub fn filta(&mut self) -> FILTA_W {
        FILTA_W { w: self }
    }
}
