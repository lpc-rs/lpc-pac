#[doc = "Writer for register CIENF"]
pub type W = crate::W<u32, super::CIENF>;
#[doc = "Register CIENF `reset()`'s with value 0"]
impl crate::ResetValue for super::CIENF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CENAF`"]
pub struct CENAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CENAF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    pub fn cenaf(&mut self) -> CENAF_W {
        CENAF_W { w: self }
    }
}
