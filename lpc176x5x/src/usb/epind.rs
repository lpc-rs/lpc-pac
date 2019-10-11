#[doc = "Writer for register EPIND"]
pub type W = crate::W<u32, super::EPIND>;
#[doc = "Register EPIND `reset()`'s with value 0"]
impl crate::ResetValue for super::EPIND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PHY_EP`"]
pub struct PHY_EP_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:4 - Physical endpoint number (0-31)"]
    #[inline(always)]
    pub fn phy_ep(&mut self) -> PHY_EP_W {
        PHY_EP_W { w: self }
    }
}
