#[doc = "Register `CLKOUTSELA` reader"]
pub struct R(crate::R<CLKOUTSELA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTSELA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTSELA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTSELA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTSELA` writer"]
pub struct W(crate::W<CLKOUTSELA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTSELA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLKOUTSELA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTSELA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CLKOUT clock source selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock (main_clk)"]
    MAIN_CLOCK = 0,
    #[doc = "1: CLKIN (clk_in)"]
    CLKIN = 1,
    #[doc = "2: Watchdog oscillator (wdt_clk)"]
    WATCHDOG_OSCILLATOR = 2,
    #[doc = "3: FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF = 3,
    #[doc = "4: PLL output (pll_clk)"]
    SYSTEM_PLL_OUTPUT = 4,
    #[doc = "5: USB PLL clock (usb_pll_clk)"]
    USB_PLL_CLOCK = 5,
    #[doc = "6: Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT = 6,
    #[doc = "7: RTC oscillator 32 kHz output (32k_clk)"]
    RTC_OSC_OUTPUT = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - CLKOUT clock source selection"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::MAIN_CLOCK,
            1 => SEL_A::CLKIN,
            2 => SEL_A::WATCHDOG_OSCILLATOR,
            3 => SEL_A::FRO_HF,
            4 => SEL_A::SYSTEM_PLL_OUTPUT,
            5 => SEL_A::USB_PLL_CLOCK,
            6 => SEL_A::AUDIO_PLL_OUTPUT,
            7 => SEL_A::RTC_OSC_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        **self == SEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `CLKIN`"]
    #[inline(always)]
    pub fn is_clkin(&self) -> bool {
        **self == SEL_A::CLKIN
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        **self == SEL_A::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        **self == SEL_A::FRO_HF
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_system_pll_output(&self) -> bool {
        **self == SEL_A::SYSTEM_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `USB_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_usb_pll_clock(&self) -> bool {
        **self == SEL_A::USB_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_audio_pll_output(&self) -> bool {
        **self == SEL_A::AUDIO_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `RTC_OSC_OUTPUT`"]
    #[inline(always)]
    pub fn is_rtc_osc_output(&self) -> bool {
        **self == SEL_A::RTC_OSC_OUTPUT
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - CLKOUT clock source selection"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Main clock (main_clk)"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
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
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
    }
    #[doc = "PLL output (pll_clk)"]
    #[inline(always)]
    pub fn system_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_OUTPUT)
    }
    #[doc = "USB PLL clock (usb_pll_clk)"]
    #[inline(always)]
    pub fn usb_pll_clock(self) -> &'a mut W {
        self.variant(SEL_A::USB_PLL_CLOCK)
    }
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    #[inline(always)]
    pub fn audio_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL_OUTPUT)
    }
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    #[inline(always)]
    pub fn rtc_osc_output(self) -> &'a mut W {
        self.variant(SEL_A::RTC_OSC_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CLKOUT clock source selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CLKOUT clock source selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKOUT clock source select A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutsela](index.html) module"]
pub struct CLKOUTSELA_SPEC;
impl crate::RegisterSpec for CLKOUTSELA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutsela::R](R) reader structure"]
impl crate::Readable for CLKOUTSELA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutsela::W](W) writer structure"]
impl crate::Writable for CLKOUTSELA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKOUTSELA to value 0x07"]
impl crate::Resettable for CLKOUTSELA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
