#[doc = "Reader of register AMIN"]
pub type R = crate::R<u32, super::AMIN>;
#[doc = "Writer for register AMIN"]
pub type W = crate::W<u32, super::AMIN>;
#[doc = "Register AMIN `reset()`'s with value 0"]
impl crate::ResetValue for super::AMIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MINUTES`"]
pub type MINUTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINUTES`"]
pub struct MINUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MINUTES_W {
        MINUTES_W { w: self }
    }
}
