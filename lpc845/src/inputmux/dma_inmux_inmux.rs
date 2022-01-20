#[doc = "Register `DMA_INMUX_INMUX[%s]` reader"]
pub struct R(crate::R<DMA_INMUX_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INMUX_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INMUX_INMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INMUX_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INMUX_INMUX[%s]` writer"]
pub struct W(crate::W<DMA_INMUX_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INMUX_INMUX_SPEC>;
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
impl From<crate::W<DMA_INMUX_INMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INMUX_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP` reader - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 24)."]
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
#[doc = "Field `INP` writer - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 24)."]
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 24)."]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 24)."]
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
#[doc = "DMA output trigger selection to become DMA trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_inmux_inmux](index.html) module"]
pub struct DMA_INMUX_INMUX_SPEC;
impl crate::RegisterSpec for DMA_INMUX_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_inmux_inmux::R](R) reader structure"]
impl crate::Readable for DMA_INMUX_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_inmux_inmux::W](W) writer structure"]
impl crate::Writable for DMA_INMUX_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INMUX_INMUX[%s]
to value 0x1f"]
impl crate::Resettable for DMA_INMUX_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
