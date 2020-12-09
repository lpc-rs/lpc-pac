#[doc = "Reader of register FBWST"]
pub type R = crate::R<u32, super::FBWST>;
#[doc = "Writer for register FBWST"]
pub type W = crate::W<u32, super::FBWST>;
#[doc = "Register FBWST `reset()`'s with value 0xc005"]
impl crate::ResetValue for super::FBWST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc005
    }
}
#[doc = "Reader of field `WAITSTATES`"]
pub type WAITSTATES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITSTATES`"]
pub struct WAITSTATES_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITSTATES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait states for signature generation."]
    #[inline(always)]
    pub fn waitstates(&self) -> WAITSTATES_R {
        WAITSTATES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states for signature generation."]
    #[inline(always)]
    pub fn waitstates(&mut self) -> WAITSTATES_W {
        WAITSTATES_W { w: self }
    }
}
