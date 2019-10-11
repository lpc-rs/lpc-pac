#[doc = "Reader of register CCLKCFG"]
pub type R = crate::R<u32, super::CCLKCFG>;
#[doc = "Writer for register CCLKCFG"]
pub type W = crate::W<u32, super::CCLKCFG>;
#[doc = "Register CCLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CCLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCLKSEL`"]
pub type CCLKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLKSEL`"]
pub struct CCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclksel(&self) -> CCLKSEL_R {
        CCLKSEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the divide value for creating the CPU clock (CCLK) from the PLL0 output. 0 = pllclk is divided by 1 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 1 = pllclk is divided by 2 to produce the CPU clock. This setting is not allowed when the PLL0 is connected, because the rate would always be greater than the maximum allowed CPU clock. 2 = pllclk is divided by 3 to produce the CPU clock. 3 = pllclk is divided by 4 to produce the CPU clock. ... 255 = pllclk is divided by 256 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclksel(&mut self) -> CCLKSEL_W {
        CCLKSEL_W { w: self }
    }
}
