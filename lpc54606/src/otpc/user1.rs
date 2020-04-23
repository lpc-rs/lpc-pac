#[doc = "Reader of register USER1"]
pub type R = crate::R<u32, super::USER1>;
#[doc = "Reader of field `USER1`"]
pub type USER1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - User application specific option."]
    #[inline(always)]
    pub fn user1(&self) -> USER1_R {
        USER1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
