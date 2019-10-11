#[doc = "Writer for register SETVALID0"]
pub type W = crate::W<u32, super::SETVALID0>;
#[doc = "Register SETVALID0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SETVALID0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SV`"]
pub struct SV_W<'a> {
    w: &'a mut W,
}
impl<'a> SV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n"]
    #[inline(always)]
    pub fn sv(&mut self) -> SV_W {
        SV_W { w: self }
    }
}
