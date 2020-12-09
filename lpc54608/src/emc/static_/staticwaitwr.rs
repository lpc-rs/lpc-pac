#[doc = "Reader of register STATICWAITWR"]
pub type R = crate::R<u32, super::STATICWAITWR>;
#[doc = "Writer for register STATICWAITWR"]
pub type W = crate::W<u32, super::STATICWAITWR>;
#[doc = "Register STATICWAITWR `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::STATICWAITWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `WAITWR`"]
pub type WAITWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITWR`"]
pub struct WAITWR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Write wait states."]
    #[inline(always)]
    pub fn waitwr(&self) -> WAITWR_R {
        WAITWR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write wait states."]
    #[inline(always)]
    pub fn waitwr(&mut self) -> WAITWR_W {
        WAITWR_W { w: self }
    }
}
