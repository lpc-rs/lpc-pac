#[doc = "Reader of register DMA_CHx_CUR_HST_TXBUF"]
pub type R = crate::R<u32, super::DMA_CHX_CUR_HST_TXBUF>;
#[doc = "Reader of field `HTB`"]
pub type HTB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer Cleared on Reset."]
    #[inline(always)]
    pub fn htb(&self) -> HTB_R {
        HTB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
