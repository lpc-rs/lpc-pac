#[doc = "Reader of register INT_PTD_SKIP_MAP"]
pub type R = crate::R<u32, super::INT_PTD_SKIP_MAP>;
#[doc = "Writer for register INT_PTD_SKIP_MAP"]
pub type W = crate::W<u32, super::INT_PTD_SKIP_MAP>;
#[doc = "Register INT_PTD_SKIP_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_PTD_SKIP_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT_SKIP`"]
pub type INT_SKIP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INT_SKIP`"]
pub struct INT_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&self) -> INT_SKIP_R {
        INT_SKIP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&mut self) -> INT_SKIP_W {
        INT_SKIP_W { w: self }
    }
}
