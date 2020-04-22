#[doc = "Reader of register DMA_INMUX_INMUX[%s]"]
pub type R = crate::R<u32, super::DMA_INMUX_INMUX>;
#[doc = "Writer for register DMA_INMUX_INMUX[%s]"]
pub type W = crate::W<u32, super::DMA_INMUX_INMUX>;
#[doc = "Register DMA_INMUX_INMUX[%s]
`reset()`'s with value 0x1f"]
impl crate::ResetValue for super::DMA_INMUX_INMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `INP`"]
pub type INP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INP`"]
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 17)."]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 17)."]
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W {
        INP_W { w: self }
    }
}
