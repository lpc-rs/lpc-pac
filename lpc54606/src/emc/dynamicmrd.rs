#[doc = "Reader of register DYNAMICMRD"]
pub type R = crate::R<u32, super::DYNAMICMRD>;
#[doc = "Writer for register DYNAMICMRD"]
pub type W = crate::W<u32, super::DYNAMICMRD>;
#[doc = "Register DYNAMICMRD `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICMRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TMRD`"]
pub type TMRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRD`"]
pub struct TMRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Load mode register to active command time."]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load mode register to active command time."]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W {
        TMRD_W { w: self }
    }
}
