#[doc = "Reader of register TRIGGER"]
pub type R = crate::R<u32, super::TRIGGER>;
#[doc = "Writer for register TRIGGER"]
pub type W = crate::W<u32, super::TRIGGER>;
#[doc = "Register TRIGGER `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIGGER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TriggerEvent`"]
pub type TRIGGEREVENT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TriggerEvent`"]
pub struct TRIGGEREVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGEREVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Trigger event"]
    #[inline(always)]
    pub fn trigger_event(&self) -> TRIGGEREVENT_R {
        TRIGGEREVENT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Trigger event"]
    #[inline(always)]
    pub fn trigger_event(&mut self) -> TRIGGEREVENT_W {
        TRIGGEREVENT_W { w: self }
    }
}
