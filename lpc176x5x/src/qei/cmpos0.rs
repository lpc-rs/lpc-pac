#[doc = "Reader of register CMPOS0"]
pub type R = crate::R<u32, super::CMPOS0>;
#[doc = "Writer for register CMPOS0"]
pub type W = crate::W<u32, super::CMPOS0>;
#[doc = "Register CMPOS0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CMPOS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PCMP0`"]
pub type PCMP0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCMP0`"]
pub struct PCMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Position compare value 0."]
    #[inline(always)]
    pub fn pcmp0(&self) -> PCMP0_R {
        PCMP0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 0."]
    #[inline(always)]
    pub fn pcmp0(&mut self) -> PCMP0_W {
        PCMP0_W { w: self }
    }
}
