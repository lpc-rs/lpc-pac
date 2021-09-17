#[doc = "Register `DMA_CHx_CUR_HST_RXDESC` reader"]
pub struct R(crate::R<DMA_CHX_CUR_HST_RXDESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_CUR_HST_RXDESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_CUR_HST_RXDESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_CUR_HST_RXDESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HRD` reader - Host Receive descriptor Address Pointer Cleared on Reset."]
pub struct HRD_R(crate::FieldReader<u32, u32>);
impl HRD_R {
    pub(crate) fn new(bits: u32) -> Self {
        HRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Host Receive descriptor Address Pointer Cleared on Reset."]
    #[inline(always)]
    pub fn hrd(&self) -> HRD_R {
        HRD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_cur_hst_rxdesc](index.html) module"]
pub struct DMA_CHX_CUR_HST_RXDESC_SPEC;
impl crate::RegisterSpec for DMA_CHX_CUR_HST_RXDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_cur_hst_rxdesc::R](R) reader structure"]
impl crate::Readable for DMA_CHX_CUR_HST_RXDESC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_CHx_CUR_HST_RXDESC to value 0"]
impl crate::Resettable for DMA_CHX_CUR_HST_RXDESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
