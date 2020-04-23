#[doc = "Reader of register CLKDIV"]
pub type R = crate::R<u32, super::CLKDIV>;
#[doc = "Writer for register CLKDIV"]
pub type W = crate::W<u32, super::CLKDIV>;
#[doc = "Register CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_DIVIDER0`"]
pub type CLK_DIVIDER0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_DIVIDER0`"]
pub struct CLK_DIVIDER0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divider-0 value."]
    #[inline(always)]
    pub fn clk_divider0(&self) -> CLK_DIVIDER0_R {
        CLK_DIVIDER0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider-0 value."]
    #[inline(always)]
    pub fn clk_divider0(&mut self) -> CLK_DIVIDER0_W {
        CLK_DIVIDER0_W { w: self }
    }
}
