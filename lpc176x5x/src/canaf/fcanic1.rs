#[doc = "Reader of register FCANIC1"]
pub type R = crate::R<u32, super::FCANIC1>;
#[doc = "Writer for register FCANIC1"]
pub type W = crate::W<u32, super::FCANIC1>;
#[doc = "Register FCANIC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCANIC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IntPnd32`"]
pub type INTPND32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IntPnd32`"]
pub struct INTPND32_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPND32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending bit 32. 0 = FullCan Interrupt Pending bit 32. 1 = FullCan Interrupt Pending bit 33. ... 31 = FullCan Interrupt Pending bit 63."]
    #[inline(always)]
    pub fn int_pnd32(&self) -> INTPND32_R {
        INTPND32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending bit 32. 0 = FullCan Interrupt Pending bit 32. 1 = FullCan Interrupt Pending bit 33. ... 31 = FullCan Interrupt Pending bit 63."]
    #[inline(always)]
    pub fn int_pnd32(&mut self) -> INTPND32_W {
        INTPND32_W { w: self }
    }
}
