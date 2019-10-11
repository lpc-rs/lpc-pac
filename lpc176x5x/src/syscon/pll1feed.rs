#[doc = "Writer for register PLL1FEED"]
pub type W = crate::W<u32, super::PLL1FEED>;
#[doc = "Register PLL1FEED `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1FEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PLL1FEED`"]
pub struct PLL1FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1FEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - The PLL1 feed sequence must be written to this register in order for PLL1 configuration and control register changes to take effect."]
    #[inline(always)]
    pub fn pll1feed(&mut self) -> PLL1FEED_W {
        PLL1FEED_W { w: self }
    }
}
