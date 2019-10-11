#[doc = "Reader of register OSR"]
pub type R = crate::R<u32, super::OSR>;
#[doc = "Writer for register OSR"]
pub type W = crate::W<u32, super::OSR>;
#[doc = "Register OSR `reset()`'s with value 0xf0"]
impl crate::ResetValue for super::OSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf0
    }
}
#[doc = "Reader of field `OSFRAC`"]
pub type OSFRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSFRAC`"]
pub struct OSFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `OSINT`"]
pub type OSINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSINT`"]
pub struct OSINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `FDINT`"]
pub type FDINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDINT`"]
pub struct FDINT_W<'a> {
    w: &'a mut W,
}
impl<'a> FDINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    pub fn osfrac(&self) -> OSFRAC_R {
        OSFRAC_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    pub fn osint(&self) -> OSINT_R {
        OSINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    pub fn fdint(&self) -> FDINT_R {
        FDINT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    pub fn osfrac(&mut self) -> OSFRAC_W {
        OSFRAC_W { w: self }
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    pub fn osint(&mut self) -> OSINT_W {
        OSINT_W { w: self }
    }
    #[doc = "Bits 8:14 - In Smart Card mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In Smart Card mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    pub fn fdint(&mut self) -> FDINT_W {
        FDINT_W { w: self }
    }
}
