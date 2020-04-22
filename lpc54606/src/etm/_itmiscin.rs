#[doc = "Reader of register _ITMISCIN"]
pub type R = crate::R<u32, super::_ITMISCIN>;
#[doc = "Reader of field `EXTIN`"]
pub type EXTIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `COREHALT`"]
pub type COREHALT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - A read of these bits returns the value of the EXTIN\\[1:0\\]
input pins."]
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - A read of this bit returns the value of the COREHALT input pin."]
    #[inline(always)]
    pub fn corehalt(&self) -> COREHALT_R {
        COREHALT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
