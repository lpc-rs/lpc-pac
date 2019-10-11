#[doc = "Reader of register ADR%s"]
pub type R = crate::R<u32, super::ADR>;
#[doc = "Writer for register ADR%s"]
pub type W = crate::W<u32, super::ADR>;
#[doc = "Register ADR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::ADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GC`"]
pub type GC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GC`"]
pub struct GC_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_W<'a> {
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
#[doc = "Reader of field `Address`"]
pub type ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Address`"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&mut self) -> GC_W {
        GC_W { w: self }
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
}
