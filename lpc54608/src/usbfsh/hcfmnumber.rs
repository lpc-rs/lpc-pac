#[doc = "Reader of register HCFMNUMBER"]
pub type R = crate::R<u32, super::HCFMNUMBER>;
#[doc = "Writer for register HCFMNUMBER"]
pub type W = crate::W<u32, super::HCFMNUMBER>;
#[doc = "Register HCFMNUMBER `reset()`'s with value 0"]
impl crate::ResetValue for super::HCFMNUMBER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FN`"]
pub type FN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FN`"]
pub struct FN_W<'a> {
    w: &'a mut W,
}
impl<'a> FN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub fn fn_(&mut self) -> FN_W {
        FN_W { w: self }
    }
}
