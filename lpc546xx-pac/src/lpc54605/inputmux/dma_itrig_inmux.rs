///Register `DMA_ITRIG_INMUX[%s]` reader
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
///Register `DMA_ITRIG_INMUX[%s]` writer
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
///Field `INP` reader - Trigger input number (decimal value) for DMA channel n (n = 0 to 21). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4 = Timer CTIMER0 Match 0 5 = Timer CTIMER0 Match 1 6 = Timer CTIMER1 Match 0 7 = Timer CTIMER2 Match 0 8 = Timer CTIMER2 Match 1 9 = Timer CTIMER3 Match 0 10 = Timer CTIMER4 Match 0 11 = Timer CTIMER4 Match 1 12 = Pin interrupt 0 13 = Pin interrupt 1 14 = Pin interrupt 2 15 = Pin interrupt 3 16 = DMA output trigger mux 0 17 = DMA output trigger mux 1 18 = DMA output trigger mux 2 19 = DMA output trigger mux 3
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
///Field `INP` writer - Trigger input number (decimal value) for DMA channel n (n = 0 to 21). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4 = Timer CTIMER0 Match 0 5 = Timer CTIMER0 Match 1 6 = Timer CTIMER1 Match 0 7 = Timer CTIMER2 Match 0 8 = Timer CTIMER2 Match 1 9 = Timer CTIMER3 Match 0 10 = Timer CTIMER4 Match 0 11 = Timer CTIMER4 Match 1 12 = Pin interrupt 0 13 = Pin interrupt 1 14 = Pin interrupt 2 15 = Pin interrupt 3 16 = DMA output trigger mux 0 17 = DMA output trigger mux 1 18 = DMA output trigger mux 2 19 = DMA output trigger mux 3
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    ///Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 21). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4 = Timer CTIMER0 Match 0 5 = Timer CTIMER0 Match 1 6 = Timer CTIMER1 Match 0 7 = Timer CTIMER2 Match 0 8 = Timer CTIMER2 Match 1 9 = Timer CTIMER3 Match 0 10 = Timer CTIMER4 Match 0 11 = Timer CTIMER4 Match 1 12 = Pin interrupt 0 13 = Pin interrupt 1 14 = Pin interrupt 2 15 = Pin interrupt 3 16 = DMA output trigger mux 0 17 = DMA output trigger mux 1 18 = DMA output trigger mux 2 19 = DMA output trigger mux 3
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 21). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4 = Timer CTIMER0 Match 0 5 = Timer CTIMER0 Match 1 6 = Timer CTIMER1 Match 0 7 = Timer CTIMER2 Match 0 8 = Timer CTIMER2 Match 1 9 = Timer CTIMER3 Match 0 10 = Timer CTIMER4 Match 0 11 = Timer CTIMER4 Match 1 12 = Pin interrupt 0 13 = Pin interrupt 1 14 = Pin interrupt 2 15 = Pin interrupt 3 16 = DMA output trigger mux 0 17 = DMA output trigger mux 1 18 = DMA output trigger mux 2 19 = DMA output trigger mux 3
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W {
        INP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Trigger select register for DMA channel
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_itrig_inmux](index.html) module
pub struct DMA_ITRIG_INMUX_SPEC;
impl crate::RegisterSpec for DMA_ITRIG_INMUX_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_itrig_inmux::R](R) reader structure
impl crate::Readable for DMA_ITRIG_INMUX_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dma_itrig_inmux::W](W) writer structure
impl crate::Writable for DMA_ITRIG_INMUX_SPEC {
    type Writer = W;
}
///`reset()` method sets DMA_ITRIG_INMUX[%s]
///to value 0x1f
impl crate::Resettable for DMA_ITRIG_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
