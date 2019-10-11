#[doc = "Reader of register ADOY"]
pub type R = crate::R<u32, super::ADOY>;
#[doc = "Writer for register ADOY"]
pub type W = crate::W<u32, super::ADOY>;
#[doc = "Register ADOY `reset()`'s with value 0"]
impl crate::ResetValue for super::ADOY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOY`"]
pub type DOY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOY`"]
pub struct DOY_W<'a> {
    w: &'a mut W,
}
impl<'a> DOY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&mut self) -> DOY_W {
        DOY_W { w: self }
    }
}
