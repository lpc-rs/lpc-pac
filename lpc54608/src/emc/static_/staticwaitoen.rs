#[doc = "Reader of register STATICWAITOEN"]
pub type R = crate::R<u32, super::STATICWAITOEN>;
#[doc = "Writer for register STATICWAITOEN"]
pub type W = crate::W<u32, super::STATICWAITOEN>;
#[doc = "Register STATICWAITOEN `reset()`'s with value 0"]
impl crate::ResetValue for super::STATICWAITOEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAITOEN`"]
pub type WAITOEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITOEN`"]
pub struct WAITOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITOEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait output enable."]
    #[inline(always)]
    pub fn waitoen(&self) -> WAITOEN_R {
        WAITOEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait output enable."]
    #[inline(always)]
    pub fn waitoen(&mut self) -> WAITOEN_W {
        WAITOEN_W { w: self }
    }
}
