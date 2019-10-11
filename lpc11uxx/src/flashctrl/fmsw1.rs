#[doc = "Reader of register FMSW1"]
pub type R = crate::R<u32, super::FMSW1>;
#[doc = "Reader of field `SW1_63_32`"]
pub type SW1_63_32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Word 1 of 128-bit signature (bits 63 to 32)."]
    #[inline(always)]
    pub fn sw1_63_32(&self) -> SW1_63_32_R {
        SW1_63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
