#[doc = "Reader of register DMA_CHx_CUR_HST_RXBUF"]
pub type R = crate::R<u32, super::DMA_CHX_CUR_HST_RXBUF>;
#[doc = "Reader of field `HRB`"]
pub type HRB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer Cleared on Reset."]
    #[inline(always)]
    pub fn hrb(&self) -> HRB_R {
        HRB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
