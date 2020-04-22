#[doc = "Reader of register CLKSRCSEL"]
pub type R = crate::R<u32, super::CLKSRCSEL>;
#[doc = "Writer for register CLKSRCSEL"]
pub type W = crate::W<u32, super::CLKSRCSEL>;
#[doc = "Register CLKSRCSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKSRCSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSRC_A {
    #[doc = "0: Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    SELECTS_THE_INTERNAL = 0,
    #[doc = "1: Selects the main oscillator as the PLL0 clock source.  Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    SELECTS_THE_MAIN_OSC = 1,
    #[doc = "2: Selects the RTC oscillator as the PLL0 clock source."]
    SELECTS_THE_RTC_OSCI = 2,
}
impl From<CLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSRC`"]
pub type CLKSRC_R = crate::R<u8, CLKSRC_A>;
impl CLKSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            0 => CLKSRC_A::SELECTS_THE_INTERNAL,
            1 => CLKSRC_A::SELECTS_THE_MAIN_OSC,
            2 => CLKSRC_A::SELECTS_THE_RTC_OSCI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == CLKSRC_A::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == CLKSRC_A::SELECTS_THE_MAIN_OSC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`"]
    #[inline(always)]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == CLKSRC_A::SELECTS_THE_RTC_OSCI
    }
}
#[doc = "Write proxy for field `CLKSRC`"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKSRC_A::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKSRC_A::SELECTS_THE_MAIN_OSC)
    }
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    #[inline(always)]
    pub fn selects_the_rtc_osci(self) -> &'a mut W {
        self.variant(CLKSRC_A::SELECTS_THE_RTC_OSCI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
}
