#[doc = "Reader of register CRSR_INTRAW"]
pub type R = crate::R<u32, super::CRSR_INTRAW>;
#[doc = "Reader of field `CRSRRIS`"]
pub type CRSRRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Cursor raw interrupt status."]
    #[inline(always)]
    pub fn crsrris(&self) -> CRSRRIS_R {
        CRSRRIS_R::new((self.bits & 0x01) != 0)
    }
}
