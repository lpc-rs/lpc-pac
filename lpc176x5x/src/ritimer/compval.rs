#[doc = "Reader of register COMPVAL"]
pub type R = crate::R<u32, super::COMPVAL>;
#[doc = "Writer for register COMPVAL"]
pub type W = crate::W<u32, super::COMPVAL>;
#[doc = "Register COMPVAL `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::COMPVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `RICOMP`"]
pub type RICOMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RICOMP`"]
pub struct RICOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare register. Holds the compare value which is compared to the counter."]
    #[inline(always)]
    pub fn ricomp(&self) -> RICOMP_R {
        RICOMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare register. Holds the compare value which is compared to the counter."]
    #[inline(always)]
    pub fn ricomp(&mut self) -> RICOMP_W {
        RICOMP_W { w: self }
    }
}
