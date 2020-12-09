#[doc = "Reader of register DMA_CHx_CUR_HST_RXDESC"]
pub type R = crate::R<u32, super::DMA_CHX_CUR_HST_RXDESC>;
#[doc = "Reader of field `HRD`"]
pub type HRD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive descriptor Address Pointer Cleared on Reset."]
    #[inline(always)]
    pub fn hrd(&self) -> HRD_R {
        HRD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
