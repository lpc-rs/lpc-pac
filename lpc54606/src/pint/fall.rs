#[doc = "Reader of register FALL"]
pub type R = crate::R<u32, super::FALL>;
#[doc = "Writer for register FALL"]
pub type W = crate::W<u32, super::FALL>;
#[doc = "Register FALL `reset()`'s with value 0"]
impl crate::ResetValue for super::FALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FDET`"]
pub type FDET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDET`"]
pub struct FDET_W<'a> {
    w: &'a mut W,
}
impl<'a> FDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub fn fdet(&self) -> FDET_R {
        FDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub fn fdet(&mut self) -> FDET_W {
        FDET_W { w: self }
    }
}
