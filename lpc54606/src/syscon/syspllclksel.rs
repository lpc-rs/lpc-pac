#[doc = "Reader of register SYSPLLCLKSEL"]
pub type R = crate::R<u32, super::SYSPLLCLKSEL>;
#[doc = "Writer for register SYSPLLCLKSEL"]
pub type W = crate::W<u32, super::SYSPLLCLKSEL>;
#[doc = "Register SYSPLLCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSPLLCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System PLL clock source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ = 0,
    #[doc = "1: CLKIN (clk_in)"]
    CLKIN = 1,
    #[doc = "2: Watchdog oscillator (wdt_clk)"]
    WATCHDOG_OSCILLATOR = 2,
    #[doc = "3: RTC oscillator 32 kHz output (32k_clk)"]
    RTC_OSC_OUTPUT = 3,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    NONE = 7,
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
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::FRO_12_MHZ),
            1 => Val(SEL_A::CLKIN),
            2 => Val(SEL_A::WATCHDOG_OSCILLATOR),
            3 => Val(SEL_A::RTC_OSC_OUTPUT),
            7 => Val(SEL_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        *self == SEL_A::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `CLKIN`"]
    #[inline(always)]
    pub fn is_clkin(&self) -> bool {
        *self == SEL_A::CLKIN
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SEL_A::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `RTC_OSC_OUTPUT`"]
    #[inline(always)]
    pub fn is_rtc_osc_output(&self) -> bool {
        *self == SEL_A::RTC_OSC_OUTPUT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FRO 12 MHz (fro_12m)"]
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12_MHZ)
    }
    #[doc = "CLKIN (clk_in)"]
    #[inline(always)]
    pub fn clkin(self) -> &'a mut W {
        self.variant(SEL_A::CLKIN)
    }
    #[doc = "Watchdog oscillator (wdt_clk)"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG_OSCILLATOR)
    }
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    #[inline(always)]
    pub fn rtc_osc_output(self) -> &'a mut W {
        self.variant(SEL_A::RTC_OSC_OUTPUT)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - System PLL clock source selection."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System PLL clock source selection."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
