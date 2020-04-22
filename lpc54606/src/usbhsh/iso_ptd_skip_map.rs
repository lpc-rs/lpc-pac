#[doc = "Reader of register ISO_PTD_SKIP_MAP"]
pub type R = crate::R<u32, super::ISO_PTD_SKIP_MAP>;
#[doc = "Writer for register ISO_PTD_SKIP_MAP"]
pub type W = crate::W<u32, super::ISO_PTD_SKIP_MAP>;
#[doc = "Register ISO_PTD_SKIP_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::ISO_PTD_SKIP_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISO_SKIP`"]
pub type ISO_SKIP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISO_SKIP`"]
pub struct ISO_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_SKIP_W<'a> {
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
    pub fn iso_skip(&self) -> ISO_SKIP_R {
        ISO_SKIP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn iso_skip(&mut self) -> ISO_SKIP_W {
        ISO_SKIP_W { w: self }
    }
}
