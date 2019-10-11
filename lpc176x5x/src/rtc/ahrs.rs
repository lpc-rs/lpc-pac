#[doc = "Reader of register AHRS"]
pub type R = crate::R<u32, super::AHRS>;
#[doc = "Writer for register AHRS"]
pub type W = crate::W<u32, super::AHRS>;
#[doc = "Register AHRS `reset()`'s with value 0"]
impl crate::ResetValue for super::AHRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOURS`"]
pub type HOURS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURS`"]
pub struct HOURS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W {
        HOURS_W { w: self }
    }
}
