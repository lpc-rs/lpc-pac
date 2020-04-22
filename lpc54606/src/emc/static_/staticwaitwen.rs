#[doc = "Reader of register STATICWAITWEN"]
pub type R = crate::R<u32, super::STATICWAITWEN>;
#[doc = "Writer for register STATICWAITWEN"]
pub type W = crate::W<u32, super::STATICWAITWEN>;
#[doc = "Register STATICWAITWEN `reset()`'s with value 0"]
impl crate::ResetValue for super::STATICWAITWEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAITWEN`"]
pub type WAITWEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITWEN`"]
pub struct WAITWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITWEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait write enable."]
    #[inline(always)]
    pub fn waitwen(&self) -> WAITWEN_R {
        WAITWEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait write enable."]
    #[inline(always)]
    pub fn waitwen(&mut self) -> WAITWEN_W {
        WAITWEN_W { w: self }
    }
}
