///Register `DMA_CHx_CUR_HST_TXDESC` reader
pub struct R(crate::R<DMA_CHX_CUR_HST_TXDESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_CUR_HST_TXDESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_CUR_HST_TXDESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_CUR_HST_TXDESC_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HTD` reader - Host Transmit descriptor Address Pointer Cleared on Reset.
pub struct HTD_R(crate::FieldReader<u32, u32>);
impl HTD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - Host Transmit descriptor Address Pointer Cleared on Reset.
    #[inline(always)]
    pub fn htd(&self) -> HTD_R {
        HTD_R::new(self.bits)
    }
}
///Channelx Current Host Transmit descriptor
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_chx_cur_hst_txdesc](index.html) module
pub struct DMA_CHX_CUR_HST_TXDESC_SPEC;
impl crate::RegisterSpec for DMA_CHX_CUR_HST_TXDESC_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_chx_cur_hst_txdesc::R](R) reader structure
impl crate::Readable for DMA_CHX_CUR_HST_TXDESC_SPEC {
    type Reader = R;
}
///`reset()` method sets DMA_CHx_CUR_HST_TXDESC to value 0
impl crate::Resettable for DMA_CHX_CUR_HST_TXDESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
