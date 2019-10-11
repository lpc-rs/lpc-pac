#[doc = "Reader of register SYSPLLSTAT"]
pub type R = crate::R<u32, super::SYSPLLSTAT>;
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PLL0 lock indicator"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
}
