#[doc = "Reader of register LUTERRAD"]
pub type R = crate::R<u32, super::LUTERRAD>;
#[doc = "Reader of field `LUTERRAD`"]
pub type LUTERRAD_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 2:10 - It the LUT Error bit (below) is 1, this read-only field contains the address in AF Lookup Table RAM, at which the Acceptance Filter encountered an error in the content of the tables."]
    #[inline(always)]
    pub fn luterrad(&self) -> LUTERRAD_R {
        LUTERRAD_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
