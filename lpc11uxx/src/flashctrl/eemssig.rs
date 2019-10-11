#[doc = "Reader of register EEMSSIG"]
pub type R = crate::R<u32, super::EEMSSIG>;
#[doc = "Reader of field `DATA_SIG`"]
pub type DATA_SIG_R = crate::R<u16, u16>;
#[doc = "Reader of field `PARITY_SIG`"]
pub type PARITY_SIG_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - BIST 16-bit signature calculated from only the data bytes"]
    #[inline(always)]
    pub fn data_sig(&self) -> DATA_SIG_R {
        DATA_SIG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BIST 16-bit signature calculated from only the parity bits of the data bytes"]
    #[inline(always)]
    pub fn parity_sig(&self) -> PARITY_SIG_R {
        PARITY_SIG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
