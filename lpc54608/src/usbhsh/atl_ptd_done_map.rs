#[doc = "Reader of register ATL_PTD_DONE_MAP"]
pub type R = crate::R<u32, super::ATL_PTD_DONE_MAP>;
#[doc = "Writer for register ATL_PTD_DONE_MAP"]
pub type W = crate::W<u32, super::ATL_PTD_DONE_MAP>;
#[doc = "Register ATL_PTD_DONE_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::ATL_PTD_DONE_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATL_DONE`"]
pub type ATL_DONE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ATL_DONE`"]
pub struct ATL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_DONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn atl_done(&self) -> ATL_DONE_R {
        ATL_DONE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn atl_done(&mut self) -> ATL_DONE_W {
        ATL_DONE_W { w: self }
    }
}
