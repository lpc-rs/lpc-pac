#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Reader of field `CAP`"]
pub type CAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer counter capture value."]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
