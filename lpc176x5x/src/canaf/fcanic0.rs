#[doc = "Reader of register FCANIC0"]
pub type R = crate::R<u32, super::FCANIC0>;
#[doc = "Writer for register FCANIC0"]
pub type W = crate::W<u32, super::FCANIC0>;
#[doc = "Register FCANIC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCANIC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTPND`"]
pub type INTPND_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INTPND`"]
pub struct INTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
    #[inline(always)]
    pub fn intpnd(&self) -> INTPND_R {
        INTPND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending 0 = FullCan Interrupt Pending bit 0. 1 = FullCan Interrupt Pending bit 1. ... 31 = FullCan Interrupt Pending bit 31."]
    #[inline(always)]
    pub fn intpnd(&mut self) -> INTPND_W {
        INTPND_W { w: self }
    }
}
