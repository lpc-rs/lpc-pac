#[doc = "Register `DMA_CHx_CUR_HST_TXBUF` reader"]
pub struct R(crate::R<DMA_CHX_CUR_HST_TXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_CUR_HST_TXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_CUR_HST_TXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_CUR_HST_TXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HTB` reader - Host Transmit Buffer Address Pointer Cleared on Reset."]
pub struct HTB_R(crate::FieldReader<u32, u32>);
impl HTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer Cleared on Reset."]
    #[inline(always)]
    pub fn htb(&self) -> HTB_R {
        HTB_R::new(self.bits)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_cur_hst_txbuf](index.html) module"]
pub struct DMA_CHX_CUR_HST_TXBUF_SPEC;
impl crate::RegisterSpec for DMA_CHX_CUR_HST_TXBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_cur_hst_txbuf::R](R) reader structure"]
impl crate::Readable for DMA_CHX_CUR_HST_TXBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_CHx_CUR_HST_TXBUF to value 0"]
impl crate::Resettable for DMA_CHX_CUR_HST_TXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
