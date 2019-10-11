#[doc = "Reader of register TC[%s]"]
pub type R = crate::R<u32, super::TC>;
#[doc = "Writer for register TC[%s]"]
pub type W = crate::W<u32, super::TC>;
#[doc = "Register TC[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::TC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCTC`"]
pub type MCTC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCTC`"]
pub struct MCTC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    pub fn mctc(&self) -> MCTC_R {
        MCTC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    pub fn mctc(&mut self) -> MCTC_W {
        MCTC_W { w: self }
    }
}
