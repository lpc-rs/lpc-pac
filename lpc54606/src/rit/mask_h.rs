#[doc = "Reader of register MASK_H"]
pub type R = crate::R<u32, super::MASK_H>;
#[doc = "Writer for register MASK_H"]
pub type W = crate::W<u32, super::MASK_H>;
#[doc = "Register MASK_H `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RIMASK`"]
pub type RIMASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RIMASK`"]
pub struct RIMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Mask register."]
    #[inline(always)]
    pub fn rimask(&self) -> RIMASK_R {
        RIMASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask register."]
    #[inline(always)]
    pub fn rimask(&mut self) -> RIMASK_W {
        RIMASK_W { w: self }
    }
}
