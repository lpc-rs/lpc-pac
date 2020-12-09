#[doc = "Reader of register HWVADTHGN"]
pub type R = crate::R<u32, super::HWVADTHGN>;
#[doc = "Writer for register HWVADTHGN"]
pub type W = crate::W<u32, super::HWVADTHGN>;
#[doc = "Register HWVADTHGN `reset()`'s with value 0"]
impl crate::ResetValue for super::HWVADTHGN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THGN`"]
pub type THGN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THGN`"]
pub struct THGN_W<'a> {
    w: &'a mut W,
}
impl<'a> THGN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Gain value for the noise estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
    #[inline(always)]
    pub fn thgn(&self) -> THGN_R {
        THGN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Gain value for the noise estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
    #[inline(always)]
    pub fn thgn(&mut self) -> THGN_W {
        THGN_W { w: self }
    }
}
