#[doc = "Reader of register CMPOS1"]
pub type R = crate::R<u32, super::CMPOS1>;
#[doc = "Writer for register CMPOS1"]
pub type W = crate::W<u32, super::CMPOS1>;
#[doc = "Register CMPOS1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CMPOS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PCMP1`"]
pub type PCMP1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCMP1`"]
pub struct PCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Position compare value 1."]
    #[inline(always)]
    pub fn pcmp1(&self) -> PCMP1_R {
        PCMP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 1."]
    #[inline(always)]
    pub fn pcmp1(&mut self) -> PCMP1_W {
        PCMP1_W { w: self }
    }
}
