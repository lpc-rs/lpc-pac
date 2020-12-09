#[doc = "Reader of register IDATA"]
pub type R = crate::R<u32, super::IDATA>;
#[doc = "Writer for register IDATA"]
pub type W = crate::W<u32, super::IDATA>;
#[doc = "Register IDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::IDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDATA`"]
pub type IDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDATA`"]
pub struct IDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> IDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of intermediate bytes."]
    #[inline(always)]
    pub fn idata(&self) -> IDATA_R {
        IDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of intermediate bytes."]
    #[inline(always)]
    pub fn idata(&mut self) -> IDATA_W {
        IDATA_W { w: self }
    }
}
