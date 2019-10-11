#[doc = "Reader of register IENF"]
pub type R = crate::R<u32, super::IENF>;
#[doc = "Writer for register IENF"]
pub type W = crate::W<u32, super::IENF>;
#[doc = "Register IENF `reset()`'s with value 0"]
impl crate::ResetValue for super::IENF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENAF`"]
pub type ENAF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENAF`"]
pub struct ENAF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub fn enaf(&self) -> ENAF_R {
        ENAF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub fn enaf(&mut self) -> ENAF_W {
        ENAF_W { w: self }
    }
}
