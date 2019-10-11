#[doc = "Reader of register RISE"]
pub type R = crate::R<u32, super::RISE>;
#[doc = "Writer for register RISE"]
pub type W = crate::W<u32, super::RISE>;
#[doc = "Register RISE `reset()`'s with value 0"]
impl crate::ResetValue for super::RISE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDET`"]
pub type RDET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDET`"]
pub struct RDET_W<'a> {
    w: &'a mut W,
}
impl<'a> RDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&self) -> RDET_R {
        RDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&mut self) -> RDET_W {
        RDET_W { w: self }
    }
}
