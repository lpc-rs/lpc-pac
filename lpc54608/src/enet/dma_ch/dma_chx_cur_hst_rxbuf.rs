#[doc = "Register `DMA_CHx_CUR_HST_RXBUF` reader"]
pub struct R(crate::R<DMA_CHX_CUR_HST_RXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_CUR_HST_RXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_CUR_HST_RXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_CUR_HST_RXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HRB` reader - Host Receive Buffer Address Pointer Cleared on Reset."]
pub struct HRB_R(crate::FieldReader<u32, u32>);
impl HRB_R {
    pub(crate) fn new(bits: u32) -> Self {
        HRB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer Cleared on Reset."]
    #[inline(always)]
    pub fn hrb(&self) -> HRB_R {
        HRB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channelx Current Application Receive Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_cur_hst_rxbuf](index.html) module"]
pub struct DMA_CHX_CUR_HST_RXBUF_SPEC;
impl crate::RegisterSpec for DMA_CHX_CUR_HST_RXBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_cur_hst_rxbuf::R](R) reader structure"]
impl crate::Readable for DMA_CHX_CUR_HST_RXBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_CHx_CUR_HST_RXBUF to value 0"]
impl crate::Resettable for DMA_CHX_CUR_HST_RXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
