#[doc = "Reader of register DYNAMICXSR"]
pub type R = crate::R<u32, super::DYNAMICXSR>;
#[doc = "Writer for register DYNAMICXSR"]
pub type W = crate::W<u32, super::DYNAMICXSR>;
#[doc = "Register DYNAMICXSR `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::DYNAMICXSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `TXSR`"]
pub type TXSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXSR`"]
pub struct TXSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Exit self-refresh to active command time."]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Exit self-refresh to active command time."]
    #[inline(always)]
    pub fn txsr(&mut self) -> TXSR_W {
        TXSR_W { w: self }
    }
}
