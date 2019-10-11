#[doc = "Reader of register FMSW3"]
pub type R = crate::R<u32, super::FMSW3>;
#[doc = "Reader of field `SW3_127_96`"]
pub type SW3_127_96_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Word 3 of 128-bit signature (bits 127 to 96)."]
    #[inline(always)]
    pub fn sw3_127_96(&self) -> SW3_127_96_R {
        SW3_127_96_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
