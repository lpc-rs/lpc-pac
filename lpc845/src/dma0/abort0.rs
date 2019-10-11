#[doc = "Writer for register ABORT0"]
pub type W = crate::W<u32, super::ABORT0>;
#[doc = "Register ABORT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ABORT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ABORTCTRL`"]
pub struct ABORTCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORTCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[inline(always)]
    pub fn abortctrl(&mut self) -> ABORTCTRL_W {
        ABORTCTRL_W { w: self }
    }
}
