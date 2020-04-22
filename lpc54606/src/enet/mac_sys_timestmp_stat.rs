#[doc = "Reader of register MAC_SYS_TIMESTMP_STAT"]
pub type R = crate::R<u32, super::MAC_SYS_TIMESTMP_STAT>;
#[doc = "Reader of field `TSSOVF`"]
pub type TSSOVF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Time stamp seconds overflow When set, indicates that the seconds value of the Time stamp has overflowed beyond 0xFFFF_FFFF."]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 0x01) != 0)
    }
}
