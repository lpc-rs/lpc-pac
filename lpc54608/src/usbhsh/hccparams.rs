#[doc = "Reader of register HCCPARAMS"]
pub type R = crate::R<u32, super::HCCPARAMS>;
#[doc = "Reader of field `LPMC`"]
pub type LPMC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17 - Link Power Management Capability."]
    #[inline(always)]
    pub fn lpmc(&self) -> LPMC_R {
        LPMC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
