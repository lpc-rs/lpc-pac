#[doc = "Reader of register CRSR_INTSTAT"]
pub type R = crate::R<u32, super::CRSR_INTSTAT>;
#[doc = "Reader of field `CRSRMIS`"]
pub type CRSRMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Cursor masked interrupt status."]
    #[inline(always)]
    pub fn crsrmis(&self) -> CRSRMIS_R {
        CRSRMIS_R::new((self.bits & 0x01) != 0)
    }
}
