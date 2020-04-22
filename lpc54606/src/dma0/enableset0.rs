#[doc = "Reader of register ENABLESET0"]
pub type R = crate::R<u32, super::ENABLESET0>;
#[doc = "Writer for register ENABLESET0"]
pub type W = crate::W<u32, super::ENABLESET0>;
#[doc = "Register ENABLESET0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ENABLESET0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENA`"]
pub type ENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENA`"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
}
