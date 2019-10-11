#[doc = "Writer for register SIENF"]
pub type W = crate::W<u32, super::SIENF>;
#[doc = "Register SIENF `reset()`'s with value 0"]
impl crate::ResetValue for super::SIENF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SETENAF`"]
pub struct SETENAF_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENAF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    pub fn setenaf(&mut self) -> SETENAF_W {
        SETENAF_W { w: self }
    }
}
