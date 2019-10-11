#[doc = "Reader of register INT"]
pub type R = crate::R<u32, super::INT>;
#[doc = "Writer for register INT"]
pub type W = crate::W<u32, super::INT>;
#[doc = "Register INT `reset()`'s with value 0"]
impl crate::ResetValue for super::INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPIF`"]
pub type SPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIF`"]
pub struct SPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software."]
    #[inline(always)]
    pub fn spif(&self) -> SPIF_R {
        SPIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI interrupt flag. Set by the SPI interface to generate an interrupt. Cleared by writing a 1 to this bit. Note: this bit will be set once when SPIE = 1 and at least one of SPIF and WCOL bits is 1. However, only when the SPI Interrupt bit is set and SPI0 Interrupt is enabled in the NVIC, SPI based interrupt can be processed by interrupt handling software."]
    #[inline(always)]
    pub fn spif(&mut self) -> SPIF_W {
        SPIF_W { w: self }
    }
}
