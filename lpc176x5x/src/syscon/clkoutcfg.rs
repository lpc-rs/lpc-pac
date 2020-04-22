#[doc = "Reader of register CLKOUTCFG"]
pub type R = crate::R<u32, super::CLKOUTCFG>;
#[doc = "Writer for register CLKOUTCFG"]
pub type W = crate::W<u32, super::CLKOUTCFG>;
#[doc = "Register CLKOUTCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKOUTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the clock source for the CLKOUT function. Other values are reserved. Do not use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: Selects the CPU clock as the CLKOUT source."]
    SELECTS_THE_CPU_CLOC = 0,
    #[doc = "1: Selects the main oscillator as the CLKOUT source."]
    SELECTS_THE_MAIN_OSC = 1,
    #[doc = "2: Selects the Internal RC oscillator as the CLKOUT source."]
    SELECTS_THE_INTERNAL = 2,
    #[doc = "3: Selects the USB clock as the CLKOUT source."]
    SELECTS_THE_USB_CLOC = 3,
    #[doc = "4: Selects the RTC oscillator as the CLKOUT source."]
    SELECTS_THE_RTC_OSCI = 4,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL_A::SELECTS_THE_CPU_CLOC),
            1 => Val(CLKOUTSEL_A::SELECTS_THE_MAIN_OSC),
            2 => Val(CLKOUTSEL_A::SELECTS_THE_INTERNAL),
            3 => Val(CLKOUTSEL_A::SELECTS_THE_USB_CLOC),
            4 => Val(CLKOUTSEL_A::SELECTS_THE_RTC_OSCI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_CPU_CLOC`"]
    #[inline(always)]
    pub fn is_selects_the_cpu_cloc(&self) -> bool {
        *self == CLKOUTSEL_A::SELECTS_THE_CPU_CLOC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == CLKOUTSEL_A::SELECTS_THE_MAIN_OSC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == CLKOUTSEL_A::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_USB_CLOC`"]
    #[inline(always)]
    pub fn is_selects_the_usb_cloc(&self) -> bool {
        *self == CLKOUTSEL_A::SELECTS_THE_USB_CLOC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`"]
    #[inline(always)]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == CLKOUTSEL_A::SELECTS_THE_RTC_OSCI
    }
}
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_cpu_cloc(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::SELECTS_THE_CPU_CLOC)
    }
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::SELECTS_THE_MAIN_OSC)
    }
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the USB clock as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_usb_cloc(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::SELECTS_THE_USB_CLOC)
    }
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn selects_the_rtc_osci(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::SELECTS_THE_RTC_OSCI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CLKOUTDIV`"]
pub type CLKOUTDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKOUTDIV`"]
pub struct CLKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLKOUT_EN`"]
pub type CLKOUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOUT_EN`"]
pub struct CLKOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKOUT_ACT`"]
pub type CLKOUT_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOUT_ACT`"]
pub struct CLKOUT_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_ACT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> CLKOUTDIV_R {
        CLKOUTDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&self) -> CLKOUT_EN_R {
        CLKOUT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&self) -> CLKOUT_ACT_R {
        CLKOUT_ACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&mut self) -> CLKOUTDIV_W {
        CLKOUTDIV_W { w: self }
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&mut self) -> CLKOUT_EN_W {
        CLKOUT_EN_W { w: self }
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&mut self) -> CLKOUT_ACT_W {
        CLKOUT_ACT_W { w: self }
    }
}
