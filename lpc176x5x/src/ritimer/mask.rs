#[doc = "Reader of register MASK"]
pub type R = crate::R<u32, super::MASK>;
#[doc = "Writer for register MASK"]
pub type W = crate::W<u32, super::MASK>;
#[doc = "Register MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RIMASK`"]
pub type RIMASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RIMASK`"]
pub struct RIMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask register. This register holds the 32-bit mask value. A one written to any bit overrides the result of the comparison for the corresponding bit of the counter and compare register (causes the comparison of the register bits to be always true)."]
    #[inline(always)]
    pub fn rimask(&self) -> RIMASK_R {
        RIMASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask register. This register holds the 32-bit mask value. A one written to any bit overrides the result of the comparison for the corresponding bit of the counter and compare register (causes the comparison of the register bits to be always true)."]
    #[inline(always)]
    pub fn rimask(&mut self) -> RIMASK_W {
        RIMASK_W { w: self }
    }
}
