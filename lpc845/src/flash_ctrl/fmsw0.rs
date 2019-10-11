#[doc = "Reader of register FMSW0"]
pub type R = crate::R<u32, super::FMSW0>;
#[doc = "Reader of field `SIG`"]
pub type SIG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit signature."]
    #[inline(always)]
    pub fn sig(&self) -> SIG_R {
        SIG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
