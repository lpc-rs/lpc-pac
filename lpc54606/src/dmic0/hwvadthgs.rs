#[doc = "Reader of register HWVADTHGS"]
pub type R = crate::R<u32, super::HWVADTHGS>;
#[doc = "Writer for register HWVADTHGS"]
pub type W = crate::W<u32, super::HWVADTHGS>;
#[doc = "Register HWVADTHGS `reset()`'s with value 0x04"]
impl crate::ResetValue for super::HWVADTHGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `THGS`"]
pub type THGS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THGS`"]
pub struct THGS_W<'a> {
    w: &'a mut W,
}
impl<'a> THGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Gain value for the signal estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
    #[inline(always)]
    pub fn thgs(&self) -> THGS_R {
        THGS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Gain value for the signal estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
    #[inline(always)]
    pub fn thgs(&mut self) -> THGS_W {
        THGS_W { w: self }
    }
}
