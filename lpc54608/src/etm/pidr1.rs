#[doc = "Reader of register PIDR1"]
pub type R = crate::R<u32, super::PIDR1>;
#[doc = "Reader of field `PartNumber`"]
pub type PARTNUMBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `JEP106_identity_code`"]
pub type JEP106_IDENTITY_CODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Part Number \\[11:8\\]"]
    #[inline(always)]
    pub fn part_number(&self) -> PARTNUMBER_R {
        PARTNUMBER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity code \\[3:0\\]"]
    #[inline(always)]
    pub fn jep106_identity_code(&self) -> JEP106_IDENTITY_CODE_R {
        JEP106_IDENTITY_CODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
