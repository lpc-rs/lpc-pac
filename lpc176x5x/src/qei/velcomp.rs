#[doc = "Reader of register VELCOMP"]
pub type R = crate::R<u32, super::VELCOMP>;
#[doc = "Writer for register VELCOMP"]
pub type W = crate::W<u32, super::VELCOMP>;
#[doc = "Register VELCOMP `reset()`'s with value 0"]
impl crate::ResetValue for super::VELCOMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VELPC`"]
pub type VELPC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VELPC`"]
pub struct VELPC_W<'a> {
    w: &'a mut W,
}
impl<'a> VELPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&self) -> VELPC_R {
        VELPC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&mut self) -> VELPC_W {
        VELPC_W { w: self }
    }
}
