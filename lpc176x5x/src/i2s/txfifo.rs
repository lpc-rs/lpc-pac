#[doc = "Writer for register TXFIFO"]
pub type W = crate::W<u32, super::TXFIFO>;
#[doc = "Register TXFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `I2STXFIFO`"]
pub struct I2STXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2STXFIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    pub fn i2stxfifo(&mut self) -> I2STXFIFO_W {
        I2STXFIFO_W { w: self }
    }
}
