#[doc = "Reader of register CLKOUTSEL"]
pub type R = crate::R<u32, super::CLKOUTSEL>;
#[doc = "Writer for register CLKOUTSEL"]
pub type W = crate::W<u32, super::CLKOUTSEL>;
#[doc = "Register CLKOUTSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKOUTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CLKOUT clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC_OSC = 0,
    #[doc = "1: Crystal oscillator (SYSOSC)"]
    SYSOSC = 1,
    #[doc = "2: Watchdog oscillator"]
    WDTOSC = 2,
    #[doc = "3: Main clock"]
    MAIN_CLK = 3,
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
            1 => SEL_A::SYSOSC,
            2 => SEL_A::WDTOSC,
            3 => SEL_A::MAIN_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSC`"]
    #[inline(always)]
    pub fn is_irc_osc(&self) -> bool {
        *self == SEL_A::IRC_OSC
    }
    #[doc = "Checks if the value of the field is `SYSOSC`"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == SEL_A::SYSOSC
    }
    #[doc = "Checks if the value of the field is `WDTOSC`"]
    #[inline(always)]
    pub fn is_wdtosc(&self) -> bool {
        *self == SEL_A::WDTOSC
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == SEL_A::MAIN_CLK
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
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn irc_osc(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSC)
    }
    #[doc = "Crystal oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut W {
        self.variant(SEL_A::SYSOSC)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn wdtosc(self) -> &'a mut W {
        self.variant(SEL_A::WDTOSC)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CLKOUT clock source."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT clock source."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
