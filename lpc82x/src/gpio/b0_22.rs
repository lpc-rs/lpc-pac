#[doc = "Reader of register B0_22"]
pub type R = crate::R<u8, super::B0_22>;
#[doc = "Writer for register B0_22"]
pub type W = crate::W<u8, super::B0_22>;
#[doc = "Register B0_22 `reset()`'s with value 0"]
impl crate::ResetValue for super::B0_22 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PBYTE`"]
pub type PBYTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBYTE`"]
pub struct PBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBYTE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&self) -> PBYTE_R {
        PBYTE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&mut self) -> PBYTE_W {
        PBYTE_W { w: self }
    }
}
