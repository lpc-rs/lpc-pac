#[doc = "Reader of register WARNINT"]
pub type R = crate::R<u32, super::WARNINT>;
#[doc = "Writer for register WARNINT"]
pub type W = crate::W<u32, super::WARNINT>;
#[doc = "Register WARNINT `reset()`'s with value 0"]
impl crate::ResetValue for super::WARNINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WARNINT`"]
pub type WARNINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WARNINT`"]
pub struct WARNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WARNINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub fn warnint(&self) -> WARNINT_R {
        WARNINT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub fn warnint(&mut self) -> WARNINT_W {
        WARNINT_W { w: self }
    }
}
