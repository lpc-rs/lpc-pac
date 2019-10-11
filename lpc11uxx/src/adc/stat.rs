#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADINT`"]
pub type ADINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - These bits mirror the DONE status flags that appear in the result register for each A/D channel n."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits mirror the OVERRRUN status flags that appear in the result register for each A/D channel n. Reading ADSTAT allows checking the status of all A/D channels simultaneously."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
    #[inline(always)]
    pub fn adint(&self) -> ADINT_R {
        ADINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
