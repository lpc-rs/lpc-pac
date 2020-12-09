#[doc = "Reader of register CLIMIT"]
pub type R = crate::R<u32, super::CLIMIT>;
#[doc = "Writer for register CLIMIT"]
pub type W = crate::W<u32, super::CLIMIT>;
#[doc = "Register CLIMIT `reset()`'s with value 0x0800_0000"]
impl crate::ResetValue for super::CLIMIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800_0000
    }
}
#[doc = "Reader of field `CLIMIT`"]
pub type CLIMIT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLIMIT`"]
pub struct CLIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Zero-based upper limit of cacheable memory"]
    #[inline(always)]
    pub fn climit(&self) -> CLIMIT_R {
        CLIMIT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Zero-based upper limit of cacheable memory"]
    #[inline(always)]
    pub fn climit(&mut self) -> CLIMIT_W {
        CLIMIT_W { w: self }
    }
}
