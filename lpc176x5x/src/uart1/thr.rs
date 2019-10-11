#[doc = "Writer for register THR"]
pub type W = crate::W<u32, super::THR>;
#[doc = "Register THR `reset()`'s with value 0"]
impl crate::ResetValue for super::THR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `THR`"]
pub struct THR_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing to the UART1 Transmit Holding Register causes the data to be stored in the UART1 transmit FIFO. The byte will be sent when it reaches the bottom of the FIFO and the transmitter is available."]
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W {
        THR_W { w: self }
    }
}
