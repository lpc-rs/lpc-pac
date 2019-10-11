#[doc = "Reader of register LOAD"]
pub type R = crate::R<u32, super::LOAD>;
#[doc = "Writer for register LOAD"]
pub type W = crate::W<u32, super::LOAD>;
#[doc = "Register LOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::LOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VELLOAD`"]
pub type VELLOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VELLOAD`"]
pub struct VELLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> VELLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Current velocity timer load value."]
    #[inline(always)]
    pub fn velload(&self) -> VELLOAD_R {
        VELLOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current velocity timer load value."]
    #[inline(always)]
    pub fn velload(&mut self) -> VELLOAD_W {
        VELLOAD_W { w: self }
    }
}
