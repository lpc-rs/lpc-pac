#[doc = "Reader of register ISEL"]
pub type R = crate::R<u32, super::ISEL>;
#[doc = "Writer for register ISEL"]
pub type W = crate::W<u32, super::ISEL>;
#[doc = "Register ISEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ISEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMODE`"]
pub type PMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMODE`"]
pub struct PMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PMODE_W {
        PMODE_W { w: self }
    }
}
