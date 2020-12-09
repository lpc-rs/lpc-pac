#[doc = "Reader of register COUNTER_H"]
pub type R = crate::R<u32, super::COUNTER_H>;
#[doc = "Writer for register COUNTER_H"]
pub type W = crate::W<u32, super::COUNTER_H>;
#[doc = "Register COUNTER_H `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNTER_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RICOUNTER`"]
pub type RICOUNTER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RICOUNTER`"]
pub struct RICOUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 16 LSBs of the up counter."]
    #[inline(always)]
    pub fn ricounter(&self) -> RICOUNTER_R {
        RICOUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 LSBs of the up counter."]
    #[inline(always)]
    pub fn ricounter(&mut self) -> RICOUNTER_W {
        RICOUNTER_W { w: self }
    }
}
