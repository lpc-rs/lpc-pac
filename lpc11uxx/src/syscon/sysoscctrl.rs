#[doc = "Register `SYSOSCCTRL` reader"]
pub struct R(crate::R<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSOSCCTRL` writer"]
pub struct W(crate::W<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSOSCCTRL_SPEC>;
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
impl From<crate::W<SYSOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bypass system oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Oscillator is not bypassed."]
    DISABLED = 0,
    #[doc = "1: Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Bypass system oscillator"]
pub struct BYPASS_R(crate::FieldReader<bool, BYPASS_A>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BYPASS_A::ENABLED
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Bypass system oscillator"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Oscillator is not bypassed."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Determines frequency range for Low-power oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGE_A {
    #[doc = "0: 1 - 20 MHz frequency range."]
    _1_20_MHZ_FREQUENCY = 0,
    #[doc = "1: 15 - 25 MHz frequency range"]
    _15_25_MHZ_FREQUENC = 1,
}
impl From<FREQRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQRANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQRANGE` reader - Determines frequency range for Low-power oscillator."]
pub struct FREQRANGE_R(crate::FieldReader<bool, FREQRANGE_A>);
impl FREQRANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQRANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQRANGE_A {
        match self.bits {
            false => FREQRANGE_A::_1_20_MHZ_FREQUENCY,
            true => FREQRANGE_A::_15_25_MHZ_FREQUENC,
        }
    }
    #[doc = "Checks if the value of the field is `_1_20_MHZ_FREQUENCY`"]
    #[inline(always)]
    pub fn is_1_20_mhz_frequency(&self) -> bool {
        **self == FREQRANGE_A::_1_20_MHZ_FREQUENCY
    }
    #[doc = "Checks if the value of the field is `_15_25_MHZ_FREQUENC`"]
    #[inline(always)]
    pub fn is_15_25_mhz_frequenc(&self) -> bool {
        **self == FREQRANGE_A::_15_25_MHZ_FREQUENC
    }
}
impl core::ops::Deref for FREQRANGE_R {
    type Target = crate::FieldReader<bool, FREQRANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQRANGE` writer - Determines frequency range for Low-power oscillator."]
pub struct FREQRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQRANGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1 - 20 MHz frequency range."]
    #[inline(always)]
    pub fn _1_20_mhz_frequency(self) -> &'a mut W {
        self.variant(FREQRANGE_A::_1_20_MHZ_FREQUENCY)
    }
    #[doc = "15 - 25 MHz frequency range"]
    #[inline(always)]
    pub fn _15_25_mhz_frequenc(self) -> &'a mut W {
        self.variant(FREQRANGE_A::_15_25_MHZ_FREQUENC)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FREQRANGE_W {
        FREQRANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscctrl](index.html) module"]
pub struct SYSOSCCTRL_SPEC;
impl crate::RegisterSpec for SYSOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysoscctrl::R](R) reader structure"]
impl crate::Readable for SYSOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysoscctrl::W](W) writer structure"]
impl crate::Writable for SYSOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSOSCCTRL to value 0"]
impl crate::Resettable for SYSOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
