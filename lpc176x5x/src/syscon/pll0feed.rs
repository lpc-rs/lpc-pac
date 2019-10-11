#[doc = "Writer for register PLL0FEED"]
pub type W = crate::W<u32, super::PLL0FEED>;
#[doc = "Register PLL0FEED `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL0FEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PLL0FEED`"]
pub struct PLL0FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL0FEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - The PLL0 feed sequence must be written to this register in order for PLL0 configuration and control register changes to take effect."]
    #[inline(always)]
    pub fn pll0feed(&mut self) -> PLL0FEED_W {
        PLL0FEED_W { w: self }
    }
}
