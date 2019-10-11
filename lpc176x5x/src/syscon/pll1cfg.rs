#[doc = "Reader of register PLL1CFG"]
pub type R = crate::R<u32, super::PLL1CFG>;
#[doc = "Writer for register PLL1CFG"]
pub type W = crate::W<u32, super::PLL1CFG>;
#[doc = "Register PLL1CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSEL1`"]
pub type MSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSEL1`"]
pub struct MSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PSEL1`"]
pub type PSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSEL1`"]
pub struct PSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn msel1(&self) -> MSEL1_R {
        MSEL1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn psel1(&self) -> PSEL1_R {
        PSEL1_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn msel1(&mut self) -> MSEL1_W {
        MSEL1_W { w: self }
    }
    #[doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn psel1(&mut self) -> PSEL1_W {
        PSEL1_W { w: self }
    }
}
