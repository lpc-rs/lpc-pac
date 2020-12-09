#[doc = "Reader of register PIDR3"]
pub type R = crate::R<u32, super::PIDR3>;
#[doc = "Reader of field `CustomerModified`"]
pub type CUSTOMERMODIFIED_R = crate::R<u8, u8>;
#[doc = "Reader of field `RevAnd`"]
pub type REVAND_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Customer Modified."]
    #[inline(always)]
    pub fn customer_modified(&self) -> CUSTOMERMODIFIED_R {
        CUSTOMERMODIFIED_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn rev_and(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
