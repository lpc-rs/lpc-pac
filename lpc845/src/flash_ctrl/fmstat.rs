#[doc = "Reader of register FMSTAT"]
pub type R = crate::R<u32, super::FMSTAT>;
#[doc = "Reader of field `SIG_DONE`"]
pub type SIG_DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - This status bit is set at the end of signature computation"]
    #[inline(always)]
    pub fn sig_done(&self) -> SIG_DONE_R {
        SIG_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
