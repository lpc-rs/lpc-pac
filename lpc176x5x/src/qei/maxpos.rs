#[doc = "Reader of register MAXPOS"]
pub type R = crate::R<u32, super::MAXPOS>;
#[doc = "Writer for register MAXPOS"]
pub type W = crate::W<u32, super::MAXPOS>;
#[doc = "Register MAXPOS `reset()`'s with value 0"]
impl crate::ResetValue for super::MAXPOS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXPOS`"]
pub type MAXPOS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MAXPOS`"]
pub struct MAXPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Current maximum position value."]
    #[inline(always)]
    pub fn maxpos(&self) -> MAXPOS_R {
        MAXPOS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current maximum position value."]
    #[inline(always)]
    pub fn maxpos(&mut self) -> MAXPOS_W {
        MAXPOS_W { w: self }
    }
}
