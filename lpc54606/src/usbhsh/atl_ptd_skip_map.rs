#[doc = "Reader of register ATL_PTD_SKIP_MAP"]
pub type R = crate::R<u32, super::ATL_PTD_SKIP_MAP>;
#[doc = "Writer for register ATL_PTD_SKIP_MAP"]
pub type W = crate::W<u32, super::ATL_PTD_SKIP_MAP>;
#[doc = "Register ATL_PTD_SKIP_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::ATL_PTD_SKIP_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATL_SKIP`"]
pub type ATL_SKIP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ATL_SKIP`"]
pub struct ATL_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_SKIP_W<'a> {
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
    pub fn atl_skip(&self) -> ATL_SKIP_R {
        ATL_SKIP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn atl_skip(&mut self) -> ATL_SKIP_W {
        ATL_SKIP_W { w: self }
    }
}
