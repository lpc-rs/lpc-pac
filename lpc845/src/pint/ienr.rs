#[doc = "Reader of register IENR"]
pub type R = crate::R<u32, super::IENR>;
#[doc = "Writer for register IENR"]
pub type W = crate::W<u32, super::IENR>;
#[doc = "Register IENR `reset()`'s with value 0"]
impl crate::ResetValue for super::IENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENRL`"]
pub type ENRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENRL`"]
pub struct ENRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl(&self) -> ENRL_R {
        ENRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl(&mut self) -> ENRL_W {
        ENRL_W { w: self }
    }
}
