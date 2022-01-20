#[doc = "Register `DMA_ITRIG_INMUX[%s]` reader"]
pub struct R(crate::R<DMA_ITRIG_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ITRIG_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ITRIG_INMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ITRIG_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ITRIG_INMUX[%s]` writer"]
pub struct W(crate::W<DMA_ITRIG_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ITRIG_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_ITRIG_INMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ITRIG_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP` reader - Trigger input number (decimal value) for DMA channel n (n = 0 to 12). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4= Comparator out 5 = Pin interrupt 4 6 = Pin interrupt 5 7 = Pin interrupt 6 8 = Pin interrupt 7 9= Timer CTIMER0 Match 0 dma request 10 = Timer CTIMER0 Match 1 dma request 11 = DMA output trigger mux 0 12 = DMA output trigger mux 1"]
pub struct INP_R(crate::FieldReader<u8, u8>);
impl INP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP` writer - Trigger input number (decimal value) for DMA channel n (n = 0 to 12). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4= Comparator out 5 = Pin interrupt 4 6 = Pin interrupt 5 7 = Pin interrupt 6 8 = Pin interrupt 7 9= Timer CTIMER0 Match 0 dma request 10 = Timer CTIMER0 Match 1 dma request 11 = DMA output trigger mux 0 12 = DMA output trigger mux 1"]
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger select register for DMA channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_itrig_inmux](index.html) module"]
pub struct DMA_ITRIG_INMUX_SPEC;
impl crate::RegisterSpec for DMA_ITRIG_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_itrig_inmux::R](R) reader structure"]
impl crate::Readable for DMA_ITRIG_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_itrig_inmux::W](W) writer structure"]
impl crate::Writable for DMA_ITRIG_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ITRIG_INMUX[%s]
to value 0x0f"]
impl crate::Resettable for DMA_ITRIG_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
