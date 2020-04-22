#[doc = "Reader of register FMSW[%s]"]
pub type R = crate::R<u32, super::FMSW>;
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Words of 128-bit signature (bits)."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
