#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `ADINTEN`"]
pub type ADINTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADINTEN`"]
pub struct ADINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ADGINTEN`"]
pub type ADGINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADGINTEN`"]
pub struct ADGINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADGINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits allow control over which A/D channels generate interrupts for conversion completion. When bit 0 is one, completion of a conversion on A/D channel 0 will generate an interrupt, when bit 1 is one, completion of a conversion on A/D channel 1 will generate an interrupt, etc."]
    #[inline(always)]
    pub fn adinten(&self) -> ADINTEN_R {
        ADINTEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - When 1, enables the global DONE flag in ADDR to generate an interrupt. When 0, only the individual A/D channels enabled by ADINTEN 7:0 will generate interrupts. This bit must be set to 0 in burst mode (BURST = 1 in the CR register)."]
    #[inline(always)]
    pub fn adginten(&self) -> ADGINTEN_R {
        ADGINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits allow control over which A/D channels generate interrupts for conversion completion. When bit 0 is one, completion of a conversion on A/D channel 0 will generate an interrupt, when bit 1 is one, completion of a conversion on A/D channel 1 will generate an interrupt, etc."]
    #[inline(always)]
    pub fn adinten(&mut self) -> ADINTEN_W {
        ADINTEN_W { w: self }
    }
    #[doc = "Bit 8 - When 1, enables the global DONE flag in ADDR to generate an interrupt. When 0, only the individual A/D channels enabled by ADINTEN 7:0 will generate interrupts. This bit must be set to 0 in burst mode (BURST = 1 in the CR register)."]
    #[inline(always)]
    pub fn adginten(&mut self) -> ADGINTEN_W {
        ADGINTEN_W { w: self }
    }
}
