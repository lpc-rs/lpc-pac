#[doc = "Reader of register CIDR3"]
pub type R = crate::R<u32, super::CIDR3>;
#[doc = "Reader of field `Preamble`"]
pub type PREAMBLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
