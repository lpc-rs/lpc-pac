#[doc = "Reader of register COUNTER"]
pub type R = crate::R<u32, super::COUNTER>;
#[doc = "Writer for register COUNTER"]
pub type W = crate::W<u32, super::COUNTER>;
#[doc = "Register COUNTER `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RICOUNTER`"]
pub type RICOUNTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RICOUNTER`"]
pub struct RICOUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32-bit up counter. Counts continuously unless RITEN bit in RICTRL register is cleared or debug mode is entered (if enabled by the RITNEBR bit in RICTRL). Can be loaded to any value in software."]
    #[inline(always)]
    pub fn ricounter(&self) -> RICOUNTER_R {
        RICOUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit up counter. Counts continuously unless RITEN bit in RICTRL register is cleared or debug mode is entered (if enabled by the RITNEBR bit in RICTRL). Can be loaded to any value in software."]
    #[inline(always)]
    pub fn ricounter(&mut self) -> RICOUNTER_W {
        RICOUNTER_W { w: self }
    }
}
