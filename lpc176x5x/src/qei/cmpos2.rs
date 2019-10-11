#[doc = "Reader of register CMPOS2"]
pub type R = crate::R<u32, super::CMPOS2>;
#[doc = "Writer for register CMPOS2"]
pub type W = crate::W<u32, super::CMPOS2>;
#[doc = "Register CMPOS2 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CMPOS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PCMP2`"]
pub type PCMP2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCMP2`"]
pub struct PCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    pub fn pcmp2(&self) -> PCMP2_R {
        PCMP2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 2."]
    #[inline(always)]
    pub fn pcmp2(&mut self) -> PCMP2_W {
        PCMP2_W { w: self }
    }
}
