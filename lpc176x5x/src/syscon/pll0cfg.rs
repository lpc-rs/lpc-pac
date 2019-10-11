#[doc = "Reader of register PLL0CFG"]
pub type R = crate::R<u32, super::PLL0CFG>;
#[doc = "Writer for register PLL0CFG"]
pub type W = crate::W<u32, super::PLL0CFG>;
#[doc = "Register PLL0CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL0CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSEL0`"]
pub type MSEL0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MSEL0`"]
pub struct MSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
#[doc = "Reader of field `NSEL0`"]
pub type NSEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NSEL0`"]
pub struct NSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
    #[inline(always)]
    pub fn msel0(&self) -> MSEL0_R {
        MSEL0_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
    #[inline(always)]
    pub fn nsel0(&self) -> NSEL0_R {
        NSEL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
    #[inline(always)]
    pub fn msel0(&mut self) -> MSEL0_W {
        MSEL0_W { w: self }
    }
    #[doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
    #[inline(always)]
    pub fn nsel0(&mut self) -> NSEL0_W {
        NSEL0_W { w: self }
    }
}
