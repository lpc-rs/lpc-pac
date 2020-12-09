#[doc = "Reader of register DMA_CHx_CUR_HST_TXDESC"]
pub type R = crate::R<u32, super::DMA_CHX_CUR_HST_TXDESC>;
#[doc = "Reader of field `HTD`"]
pub type HTD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit descriptor Address Pointer Cleared on Reset."]
    #[inline(always)]
    pub fn htd(&self) -> HTD_R {
        HTD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
