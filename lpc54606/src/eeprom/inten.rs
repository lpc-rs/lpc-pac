#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Reader of field `EE_PROG_DONE`"]
pub type EE_PROG_DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - EEPROM program operation finished interrupt enable bit."]
    #[inline(always)]
    pub fn ee_prog_done(&self) -> EE_PROG_DONE_R {
        EE_PROG_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
