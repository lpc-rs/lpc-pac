#[doc = "Reader of register PIDR2"]
pub type R = crate::R<u32, super::PIDR2>;
#[doc = "Reader of field `JEP106_identity_code`"]
pub type JEP106_IDENTITY_CODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `Revision`"]
pub type REVISION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP106 identity code \\[6:4\\]"]
    #[inline(always)]
    pub fn jep106_identity_code(&self) -> JEP106_IDENTITY_CODE_R {
        JEP106_IDENTITY_CODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
