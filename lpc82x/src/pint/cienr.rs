#[doc = "Writer for register CIENR"]
pub type W = crate::W<u32, super::CIENR>;
#[doc = "Register CIENR `reset()`'s with value 0"]
impl crate::ResetValue for super::CIENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CENRL`"]
pub struct CENRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CENRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    pub fn cenrl(&mut self) -> CENRL_W {
        CENRL_W { w: self }
    }
}
