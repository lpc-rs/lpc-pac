#[doc = "Reader of register FMSW0"]
pub type R = crate::R<u32, super::FMSW0>;
#[doc = "Reader of field `SW0_31_0`"]
pub type SW0_31_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Word 0 of 128-bit signature (bits 31 to 0)."]
    #[inline(always)]
    pub fn sw0_31_0(&self) -> SW0_31_0_R {
        SW0_31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
