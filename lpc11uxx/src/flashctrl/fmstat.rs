#[doc = "Reader of register FMSTAT"]
pub type R = crate::R<u32, super::FMSTAT>;
#[doc = "Reader of field `SIG_DONE`"]
pub type SIG_DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
    #[inline(always)]
    pub fn sig_done(&self) -> SIG_DONE_R {
        SIG_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
