#[doc = "Reader of register EVFLAG"]
pub type R = crate::R<u32, super::EVFLAG>;
#[doc = "Writer for register EVFLAG"]
pub type W = crate::W<u32, super::EVFLAG>;
#[doc = "Register EVFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::EVFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLAG`"]
pub type FLAG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLAG`"]
pub struct FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W {
        FLAG_W { w: self }
    }
}
