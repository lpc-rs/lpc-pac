#[doc = "Reader of register MAINCLKSEL"]
pub type R = crate::R<u32, super::MAINCLKSEL>;
#[doc = "Writer for register MAINCLKSEL"]
pub type W = crate::W<u32, super::MAINCLKSEL>;
#[doc = "Register MAINCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::MAINCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source for main clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC Oscillator."]
    IRC_OSC = 0,
    #[doc = "1: PLL input."]
    PLL_IN = 1,
    #[doc = "2: Watchdog oscillator."]
    WDTOSC = 2,
    #[doc = "3: PLL output."]
    PLL_OUT = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::IRC_OSC,
            1 => SEL_A::PLL_IN,
            2 => SEL_A::WDTOSC,
            3 => SEL_A::PLL_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSC`"]
    #[inline(always)]
    pub fn is_irc_osc(&self) -> bool {
        *self == SEL_A::IRC_OSC
    }
    #[doc = "Checks if the value of the field is `PLL_IN`"]
    #[inline(always)]
    pub fn is_pll_in(&self) -> bool {
        *self == SEL_A::PLL_IN
    }
    #[doc = "Checks if the value of the field is `WDTOSC`"]
    #[inline(always)]
    pub fn is_wdtosc(&self) -> bool {
        *self == SEL_A::WDTOSC
    }
    #[doc = "Checks if the value of the field is `PLL_OUT`"]
    #[inline(always)]
    pub fn is_pll_out(&self) -> bool {
        *self == SEL_A::PLL_OUT
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IRC Oscillator."]
    #[inline(always)]
    pub fn irc_osc(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSC)
    }
    #[doc = "PLL input."]
    #[inline(always)]
    pub fn pll_in(self) -> &'a mut W {
        self.variant(SEL_A::PLL_IN)
    }
    #[doc = "Watchdog oscillator."]
    #[inline(always)]
    pub fn wdtosc(self) -> &'a mut W {
        self.variant(SEL_A::WDTOSC)
    }
    #[doc = "PLL output."]
    #[inline(always)]
    pub fn pll_out(self) -> &'a mut W {
        self.variant(SEL_A::PLL_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
