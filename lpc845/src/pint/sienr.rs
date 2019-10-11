#[doc = "Writer for register SIENR"]
pub type W = crate::W<u32, super::SIENR>;
#[doc = "Register SIENR `reset()`'s with value 0"]
impl crate::ResetValue for super::SIENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SETENRL`"]
pub struct SETENRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl(&mut self) -> SETENRL_W {
        SETENRL_W { w: self }
    }
}
