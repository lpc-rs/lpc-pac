#[doc = "Reader of register FMSW2"]
pub type R = crate::R<u32, super::FMSW2>;
#[doc = "Reader of field `SW2_95_64`"]
pub type SW2_95_64_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Word 2 of 128-bit signature (bits 95 to 64)."]
    #[inline(always)]
    pub fn sw2_95_64(&self) -> SW2_95_64_R {
        SW2_95_64_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
