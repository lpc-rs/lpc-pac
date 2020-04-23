#[doc = "Reader of register DYNAMICAPR"]
pub type R = crate::R<u32, super::DYNAMICAPR>;
#[doc = "Writer for register DYNAMICAPR"]
pub type W = crate::W<u32, super::DYNAMICAPR>;
#[doc = "Register DYNAMICAPR `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICAPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TAPR`"]
pub type TAPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAPR`"]
pub struct TAPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Last-data-out to active command time."]
    #[inline(always)]
    pub fn tapr(&self) -> TAPR_R {
        TAPR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Last-data-out to active command time."]
    #[inline(always)]
    pub fn tapr(&mut self) -> TAPR_W {
        TAPR_W { w: self }
    }
}
