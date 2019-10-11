#[doc = "Reader of register CAP"]
pub type R = crate::R<u32, super::CAP>;
#[doc = "Reader of field `VELCAP`"]
pub type VELCAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last velocity capture."]
    #[inline(always)]
    pub fn velcap(&self) -> VELCAP_R {
        VELCAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
