#[doc = "Reader of register USER0"]
pub type R = crate::R<u32, super::USER0>;
#[doc = "Reader of field `USER0`"]
pub type USER0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - User application specific option."]
    #[inline(always)]
    pub fn user0(&self) -> USER0_R {
        USER0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
