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
#[doc = "Clock source for main clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC Oscillator"]
    IRC_OSCILLATOR = 0,
    #[doc = "1: PLL input"]
    PLL_INPUT = 1,
    #[doc = "2: Watchdog oscillator"]
    WATCHDOG_OSCILLATOR = 2,
    #[doc = "3: PLL output"]
    PLL_OUTPUT = 3,
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
            0 => SEL_A::IRC_OSCILLATOR,
            1 => SEL_A::PLL_INPUT,
            2 => SEL_A::WATCHDOG_OSCILLATOR,
            3 => SEL_A::PLL_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL_INPUT`"]
    #[inline(always)]
    pub fn is_pll_input(&self) -> bool {
        *self == SEL_A::PLL_INPUT
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SEL_A::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_pll_output(&self) -> bool {
        *self == SEL_A::PLL_OUTPUT
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
    #[doc = "IRC Oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSCILLATOR)
    }
    #[doc = "PLL input"]
    #[inline(always)]
    pub fn pll_input(self) -> &'a mut W {
        self.variant(SEL_A::PLL_INPUT)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG_OSCILLATOR)
    }
    #[doc = "PLL output"]
    #[inline(always)]
    pub fn pll_output(self) -> &'a mut W {
        self.variant(SEL_A::PLL_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
