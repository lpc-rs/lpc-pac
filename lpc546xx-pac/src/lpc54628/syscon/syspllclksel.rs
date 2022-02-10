///Register `SYSPLLCLKSEL` reader
pub struct R(crate::R<SYSPLLCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SYSPLLCLKSEL` writer
pub struct W(crate::W<SYSPLLCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLCLKSEL_SPEC>;
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
impl From<crate::W<SYSPLLCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
///System PLL clock source selection.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    ///0: FRO 12 MHz (fro_12m)
    FRO_12_MHZ = 0,
    ///1: CLKIN (clk_in)
    CLKIN = 1,
    ///2: Watchdog oscillator (wdt_clk)
    WATCHDOG_OSCILLATOR = 2,
    ///3: RTC oscillator 32 kHz output (32k_clk)
    RTC_OSC_OUTPUT = 3,
    ///7: None, this may be selected in order to reduce power when no output is needed.
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
///Field `SEL` reader - System PLL clock source selection.
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::FRO_12_MHZ),
            1 => Some(SEL_A::CLKIN),
            2 => Some(SEL_A::WATCHDOG_OSCILLATOR),
            3 => Some(SEL_A::RTC_OSC_OUTPUT),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `FRO_12_MHZ`
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        **self == SEL_A::FRO_12_MHZ
    }
    ///Checks if the value of the field is `CLKIN`
    #[inline(always)]
    pub fn is_clkin(&self) -> bool {
        **self == SEL_A::CLKIN
    }
    ///Checks if the value of the field is `WATCHDOG_OSCILLATOR`
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        **self == SEL_A::WATCHDOG_OSCILLATOR
    }
    ///Checks if the value of the field is `RTC_OSC_OUTPUT`
    #[inline(always)]
    pub fn is_rtc_osc_output(&self) -> bool {
        **self == SEL_A::RTC_OSC_OUTPUT
    }
    ///Checks if the value of the field is `NONE`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SEL_A::NONE
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SEL` writer - System PLL clock source selection.
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///FRO 12 MHz (fro_12m)
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12_MHZ)
    }
    ///CLKIN (clk_in)
    #[inline(always)]
    pub fn clkin(self) -> &'a mut W {
        self.variant(SEL_A::CLKIN)
    }
    ///Watchdog oscillator (wdt_clk)
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG_OSCILLATOR)
    }
    ///RTC oscillator 32 kHz output (32k_clk)
    #[inline(always)]
    pub fn rtc_osc_output(self) -> &'a mut W {
        self.variant(SEL_A::RTC_OSC_OUTPUT)
    }
    ///None, this may be selected in order to reduce power when no output is needed.
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - System PLL clock source selection.
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - System PLL clock source selection.
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PLL clock source select
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syspllclksel](index.html) module
pub struct SYSPLLCLKSEL_SPEC;
impl crate::RegisterSpec for SYSPLLCLKSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [syspllclksel::R](R) reader structure
impl crate::Readable for SYSPLLCLKSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [syspllclksel::W](W) writer structure
impl crate::Writable for SYSPLLCLKSEL_SPEC {
    type Writer = W;
}
///`reset()` method sets SYSPLLCLKSEL to value 0
impl crate::Resettable for SYSPLLCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
