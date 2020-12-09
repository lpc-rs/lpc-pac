#[doc = "Reader of register EPTOGGLE"]
pub type R = crate::R<u32, super::EPTOGGLE>;
#[doc = "Writer for register EPTOGGLE"]
pub type W = crate::W<u32, super::EPTOGGLE>;
#[doc = "Register EPTOGGLE `reset()`'s with value 0"]
impl crate::ResetValue for super::EPTOGGLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOGGLE`"]
pub type TOGGLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOGGLE`"]
pub struct TOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOGGLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&mut self) -> TOGGLE_W {
        TOGGLE_W { w: self }
    }
}
