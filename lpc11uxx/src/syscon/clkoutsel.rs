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
#[doc = "CLKOUT clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC_OSCILLATOR = 0,
    #[doc = "1: Crystal oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR = 1,
    #[doc = "2: LF oscillator (watchdog oscillator)"]
    LF_OSCILLATOR_WATCH = 2,
    #[doc = "3: Main clock"]
    MAIN_CLOCK = 3,
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
            1 => SEL_A::CRYSTAL_OSCILLATOR,
            2 => SEL_A::LF_OSCILLATOR_WATCH,
            3 => SEL_A::MAIN_CLOCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `CRYSTAL_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_crystal_oscillator(&self) -> bool {
        *self == SEL_A::CRYSTAL_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `LF_OSCILLATOR_WATCH`"]
    #[inline(always)]
    pub fn is_lf_oscillator_watch(&self) -> bool {
        *self == SEL_A::LF_OSCILLATOR_WATCH
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
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
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSCILLATOR)
    }
    #[doc = "Crystal oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn crystal_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::CRYSTAL_OSCILLATOR)
    }
    #[doc = "LF oscillator (watchdog oscillator)"]
    #[inline(always)]
    pub fn lf_oscillator_watch(self) -> &'a mut W {
        self.variant(SEL_A::LF_OSCILLATOR_WATCH)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
