#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Select clock mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNMODE_A {
    #[doc = "0: Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    SYNCHRONOUS_MODE = 0,
    #[doc = "1: Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    ASYNCHRONOUS_MODE = 1,
}
impl From<ASYNMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ASYNMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNMODE` reader - Select clock mode."]
pub struct ASYNMODE_R(crate::FieldReader<bool, ASYNMODE_A>);
impl ASYNMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASYNMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASYNMODE_A {
        match self.bits {
            false => ASYNMODE_A::SYNCHRONOUS_MODE,
            true => ASYNMODE_A::ASYNCHRONOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_synchronous_mode(&self) -> bool {
        **self == ASYNMODE_A::SYNCHRONOUS_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        **self == ASYNMODE_A::ASYNCHRONOUS_MODE
    }
}
impl core::ops::Deref for ASYNMODE_R {
    type Target = crate::FieldReader<bool, ASYNMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASYNMODE` writer - Select clock mode."]
pub struct ASYNMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASYNMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    #[inline(always)]
    pub fn synchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODE_A::SYNCHRONOUS_MODE)
    }
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODE_A::ASYNCHRONOUS_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "The low-power ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWRMODE_A {
    #[doc = "0: The low-power ADC mode is disabled. The analog circuitry remains activated even when no conversions are requested."]
    LPWRMODE_0 = 0,
    #[doc = "1: The low-power ADC mode is enabled. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. Using this mode can save an appreciable amount of current (approximately 2.5 mA) when conversions are required relatively infrequently. The penalty for using this mode is an approximately FIFTEEN ADC CLOCK delay (30 clocks in 10-bit mode), based on the frequency specified in the CLKDIV field, from the time the trigger event occurs until sampling of the A/D input commences. Note: This mode will NOT power-up the A/D if the ADC_ENA bit is low."]
    LPWRMODE_1 = 1,
}
impl From<LPWRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPWRMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPWRMODE` reader - The low-power ADC mode"]
pub struct LPWRMODE_R(crate::FieldReader<bool, LPWRMODE_A>);
impl LPWRMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPWRMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPWRMODE_A {
        match self.bits {
            false => LPWRMODE_A::LPWRMODE_0,
            true => LPWRMODE_A::LPWRMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPWRMODE_0`"]
    #[inline(always)]
    pub fn is_lpwrmode_0(&self) -> bool {
        **self == LPWRMODE_A::LPWRMODE_0
    }
    #[doc = "Checks if the value of the field is `LPWRMODE_1`"]
    #[inline(always)]
    pub fn is_lpwrmode_1(&self) -> bool {
        **self == LPWRMODE_A::LPWRMODE_1
    }
}
impl core::ops::Deref for LPWRMODE_R {
    type Target = crate::FieldReader<bool, LPWRMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPWRMODE` writer - The low-power ADC mode"]
pub struct LPWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPWRMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The low-power ADC mode is disabled. The analog circuitry remains activated even when no conversions are requested."]
    #[inline(always)]
    pub fn lpwrmode_0(self) -> &'a mut W {
        self.variant(LPWRMODE_A::LPWRMODE_0)
    }
    #[doc = "The low-power ADC mode is enabled. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. Using this mode can save an appreciable amount of current (approximately 2.5 mA) when conversions are required relatively infrequently. The penalty for using this mode is an approximately FIFTEEN ADC CLOCK delay (30 clocks in 10-bit mode), based on the frequency specified in the CLKDIV field, from the time the trigger event occurs until sampling of the A/D input commences. Note: This mode will NOT power-up the A/D if the ADC_ENA bit is low."]
    #[inline(always)]
    pub fn lpwrmode_1(self) -> &'a mut W {
        self.variant(LPWRMODE_A::LPWRMODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CALMODE` reader - Writing a '1' to this bit will initiate a sef-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Note: Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted unitl the full calibration cycle has ended."]
pub struct CALMODE_R(crate::FieldReader<bool, bool>);
impl CALMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALMODE` writer - Writing a '1' to this bit will initiate a sef-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Note: Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted unitl the full calibration cycle has ended."]
pub struct CALMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline(always)]
    pub fn asynmode(&self) -> ASYNMODE_R {
        ASYNMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The low-power ADC mode"]
    #[inline(always)]
    pub fn lpwrmode(&self) -> LPWRMODE_R {
        LPWRMODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Writing a '1' to this bit will initiate a sef-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Note: Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted unitl the full calibration cycle has ended."]
    #[inline(always)]
    pub fn calmode(&self) -> CALMODE_R {
        CALMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline(always)]
    pub fn asynmode(&mut self) -> ASYNMODE_W {
        ASYNMODE_W { w: self }
    }
    #[doc = "Bit 10 - The low-power ADC mode"]
    #[inline(always)]
    pub fn lpwrmode(&mut self) -> LPWRMODE_W {
        LPWRMODE_W { w: self }
    }
    #[doc = "Bit 30 - Writing a '1' to this bit will initiate a sef-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Note: Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted unitl the full calibration cycle has ended."]
    #[inline(always)]
    pub fn calmode(&mut self) -> CALMODE_W {
        CALMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
