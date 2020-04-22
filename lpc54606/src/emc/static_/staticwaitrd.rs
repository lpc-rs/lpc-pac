#[doc = "Reader of register STATICWAITRD"]
pub type R = crate::R<u32, super::STATICWAITRD>;
#[doc = "Writer for register STATICWAITRD"]
pub type W = crate::W<u32, super::STATICWAITRD>;
#[doc = "Register STATICWAITRD `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::STATICWAITRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `WAITRD`"]
pub type WAITRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITRD`"]
pub struct WAITRD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn waitrd(&self) -> WAITRD_R {
        WAITRD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn waitrd(&mut self) -> WAITRD_W {
        WAITRD_W { w: self }
    }
}
