#[doc = "Reader of register LIM[%s]"]
pub type R = crate::R<u32, super::LIM>;
#[doc = "Writer for register LIM[%s]"]
pub type W = crate::W<u32, super::LIM>;
#[doc = "Register LIM[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCLIM`"]
pub type MCLIM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCLIM`"]
pub struct MCLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    pub fn mclim(&self) -> MCLIM_R {
        MCLIM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    pub fn mclim(&mut self) -> MCLIM_W {
        MCLIM_W { w: self }
    }
}
