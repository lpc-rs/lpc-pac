#[doc = "Register `DMA_CHx_TXDESC_RING_LENGTH` reader"]
pub struct R(crate::R<DMA_CHX_TXDESC_RING_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_TXDESC_RING_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_TXDESC_RING_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_TXDESC_RING_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_TXDESC_RING_LENGTH` writer"]
pub struct W(crate::W<DMA_CHX_TXDESC_RING_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_TXDESC_RING_LENGTH_SPEC>;
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
impl From<crate::W<DMA_CHX_TXDESC_RING_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_TXDESC_RING_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDRL` reader - Transmit Ring Length This field sets the maximum number of Tx descriptors in the circular ring."]
pub struct TDRL_R(crate::FieldReader<u16, u16>);
impl TDRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TDRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDRL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDRL` writer - Transmit Ring Length This field sets the maximum number of Tx descriptors in the circular ring."]
pub struct TDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Transmit Ring Length This field sets the maximum number of Tx descriptors in the circular ring."]
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit Ring Length This field sets the maximum number of Tx descriptors in the circular ring."]
    #[inline(always)]
    pub fn tdrl(&mut self) -> TDRL_W {
        TDRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_txdesc_ring_length](index.html) module"]
pub struct DMA_CHX_TXDESC_RING_LENGTH_SPEC;
impl crate::RegisterSpec for DMA_CHX_TXDESC_RING_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_txdesc_ring_length::R](R) reader structure"]
impl crate::Readable for DMA_CHX_TXDESC_RING_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_txdesc_ring_length::W](W) writer structure"]
impl crate::Writable for DMA_CHX_TXDESC_RING_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_TXDESC_RING_LENGTH to value 0"]
impl crate::Resettable for DMA_CHX_TXDESC_RING_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
