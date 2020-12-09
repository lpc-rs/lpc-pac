#[doc = "Reader of register TV"]
pub type R = crate::R<u32, super::TV>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Counter timer value."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
