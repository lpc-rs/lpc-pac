#[doc = "Reader of register UPCURR"]
pub type R = crate::R<u32, super::UPCURR>;
#[doc = "Reader of field `LCDUPCURR`"]
pub type LCDUPCURR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - LCD Upper Panel Current Address."]
    #[inline(always)]
    pub fn lcdupcurr(&self) -> LCDUPCURR_R {
        LCDUPCURR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
