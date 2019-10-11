#[doc = "Reader of register DR[%s]"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\] pin, as it falls within the range of VREFP to V SS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits.This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
