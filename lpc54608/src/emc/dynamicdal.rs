#[doc = "Reader of register DYNAMICDAL"]
pub type R = crate::R<u32, super::DYNAMICDAL>;
#[doc = "Writer for register DYNAMICDAL"]
pub type W = crate::W<u32, super::DYNAMICDAL>;
#[doc = "Register DYNAMICDAL `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICDAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TDAL`"]
pub type TDAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDAL`"]
pub struct TDAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data-in to active command."]
    #[inline(always)]
    pub fn tdal(&self) -> TDAL_R {
        TDAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data-in to active command."]
    #[inline(always)]
    pub fn tdal(&mut self) -> TDAL_W {
        TDAL_W { w: self }
    }
}
