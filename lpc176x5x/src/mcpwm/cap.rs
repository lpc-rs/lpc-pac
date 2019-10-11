#[doc = "Reader of register CAP[%s]"]
pub type R = crate::R<u32, super::CAP>;
#[doc = "Reader of field `CAP`"]
pub type CAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current TC value at a capture event."]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
