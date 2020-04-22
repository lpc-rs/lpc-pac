#[doc = "Writer for register FIFOWR"]
pub type W = crate::W<u32, super::FIFOWR>;
#[doc = "Register FIFOWR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXDATA`"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data to the FIFO. The number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
}
