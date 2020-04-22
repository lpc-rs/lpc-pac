#[doc = "Reader of register DMA_ITRIG_INMUX[%s]"]
pub type R = crate::R<u32, super::DMA_ITRIG_INMUX>;
#[doc = "Writer for register DMA_ITRIG_INMUX[%s]"]
pub type W = crate::W<u32, super::DMA_ITRIG_INMUX>;
#[doc = "Register DMA_ITRIG_INMUX[%s]
`reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DMA_ITRIG_INMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 12). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4= Comparator out 5 = Pin interrupt 4 6 = Pin interrupt 5 7 = Pin interrupt 6 8 = Pin interrupt 7 9= Timer CTIMER0 Match 0 dma request 10 = Timer CTIMER0 Match 1 dma request 11 = DMA output trigger mux 0 12 = DMA output trigger mux 1"]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 12). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4= Comparator out 5 = Pin interrupt 4 6 = Pin interrupt 5 7 = Pin interrupt 6 8 = Pin interrupt 7 9= Timer CTIMER0 Match 0 dma request 10 = Timer CTIMER0 Match 1 dma request 11 = DMA output trigger mux 0 12 = DMA output trigger mux 1"]
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W {
        INP_W { w: self }
    }
}
