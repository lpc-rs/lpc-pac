#[doc = "Reader of register _ITATBCTR2"]
pub type R = crate::R<u32, super::_ITATBCTR2>;
#[doc = "Reader of field `ATREADY`"]
pub type ATREADY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - A read of this bit returns the value of the ETM ATREADY input."]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 0x01) != 0)
    }
}
