#[doc = "Reader of register INXCNT"]
pub type R = crate::R<u32, super::INXCNT>;
#[doc = "Reader of field `ENCPOS`"]
pub type ENCPOS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current index counter value."]
    #[inline(always)]
    pub fn encpos(&self) -> ENCPOS_R {
        ENCPOS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
