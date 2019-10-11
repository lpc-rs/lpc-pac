#[doc = "Reader of register TRM"]
pub type R = crate::R<u32, super::TRM>;
#[doc = "Writer for register TRM"]
pub type W = crate::W<u32, super::TRM>;
#[doc = "Register TRM `reset()`'s with value 0"]
impl crate::ResetValue for super::TRM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCOFFS`"]
pub type ADCOFFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCOFFS`"]
pub struct ADCOFFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCOFFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM`"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    pub fn adcoffs(&self) -> ADCOFFS_R {
        ADCOFFS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Offset trim bits for ADC operation. Initialized by the boot code. Can be overwritten by the user."]
    #[inline(always)]
    pub fn adcoffs(&mut self) -> ADCOFFS_W {
        ADCOFFS_W { w: self }
    }
    #[doc = "Bits 8:11 - written-to by boot code. Can not be overwritten by the user. These bits are locked after boot code write."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
}
