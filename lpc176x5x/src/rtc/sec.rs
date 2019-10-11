#[doc = "Reader of register SEC"]
pub type R = crate::R<u32, super::SEC>;
#[doc = "Writer for register SEC"]
pub type W = crate::W<u32, super::SEC>;
#[doc = "Register SEC `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECONDS`"]
pub type SECONDS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECONDS`"]
pub struct SECONDS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&self) -> SECONDS_R {
        SECONDS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&mut self) -> SECONDS_W {
        SECONDS_W { w: self }
    }
}
