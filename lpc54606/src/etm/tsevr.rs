#[doc = "Reader of register TSEVR"]
pub type R = crate::R<u32, super::TSEVR>;
#[doc = "Writer for register TSEVR"]
pub type W = crate::W<u32, super::TSEVR>;
#[doc = "Register TSEVR `reset()`'s with value 0"]
impl crate::ResetValue for super::TSEVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TimestampEvent`"]
pub type TIMESTAMPEVENT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TimestampEvent`"]
pub struct TIMESTAMPEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESTAMPEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Timestamp event."]
    #[inline(always)]
    pub fn timestamp_event(&self) -> TIMESTAMPEVENT_R {
        TIMESTAMPEVENT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Timestamp event."]
    #[inline(always)]
    pub fn timestamp_event(&mut self) -> TIMESTAMPEVENT_W {
        TIMESTAMPEVENT_W { w: self }
    }
}
