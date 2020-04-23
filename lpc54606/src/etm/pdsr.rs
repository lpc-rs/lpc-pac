#[doc = "Reader of register PDSR"]
pub type R = crate::R<u32, super::PDSR>;
#[doc = "Reader of field `ETMpoweredup`"]
pub type ETMPOWEREDUP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - The value of this bit indicates whether you can access the ETM Trace Registers. The value of this bit is always 1, indicating that the ETM Trace Registers can be accessed."]
    #[inline(always)]
    pub fn etmpoweredup(&self) -> ETMPOWEREDUP_R {
        ETMPOWEREDUP_R::new((self.bits & 0x01) != 0)
    }
}
