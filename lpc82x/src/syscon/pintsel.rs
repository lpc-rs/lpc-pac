#[doc = "Reader of register PINTSEL[%s]"]
pub type R = crate::R<u32, super::PINTSEL>;
#[doc = "Writer for register PINTSEL[%s]"]
pub type W = crate::W<u32, super::PINTSEL>;
#[doc = "Register PINTSEL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PINTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTPIN`"]
pub type INTPIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTPIN`"]
pub struct INTPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt or pattern match engine input. (PIO0_0 to PIO0_28 correspond to numbers 0 to 28)."]
    #[inline(always)]
    pub fn intpin(&self) -> INTPIN_R {
        INTPIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt or pattern match engine input. (PIO0_0 to PIO0_28 correspond to numbers 0 to 28)."]
    #[inline(always)]
    pub fn intpin(&mut self) -> INTPIN_W {
        INTPIN_W { w: self }
    }
}
