#[doc = "Reader of register ETSCV"]
pub type R = crate::R<u32, super::ETSCV>;
#[doc = "Writer for register ETSCV"]
pub type W = crate::W<u32, super::ETSCV>;
#[doc = "Register ETSCV `reset()`'s with value 0"]
impl crate::ResetValue for super::ETSCV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETSC`"]
pub type ETSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ETSC`"]
pub struct ETSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&self) -> ETSC_R {
        ETSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&mut self) -> ETSC_W {
        ETSC_W { w: self }
    }
}
