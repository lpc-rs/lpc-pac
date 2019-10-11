#[doc = "Reader of register ADCCLKDIV"]
pub type R = crate::R<u32, super::ADCCLKDIV>;
#[doc = "Writer for register ADCCLKDIV"]
pub type W = crate::W<u32, super::ADCCLKDIV>;
#[doc = "Register ADCCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC clock divider values 0: ADC clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC clock divider values 0: ADC clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
