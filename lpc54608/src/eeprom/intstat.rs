#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Reader of field `END_OF_PROG`"]
pub type END_OF_PROG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - EEPROM program operation finished interrupt status bit."]
    #[inline(always)]
    pub fn end_of_prog(&self) -> END_OF_PROG_R {
        END_OF_PROG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
