#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0600"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
#[doc = "Reader of field `ASYNMODE`"]
pub type ASYNMODE_R = crate::R<bool, ASYNMODE_A>;
impl ASYNMODE_R {
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
        *self == ASYNMODE_A::SYNCHRONOUS_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == ASYNMODE_A::ASYNCHRONOUS_MODE
    }
}
#[doc = "Write proxy for field `ASYNMODE`"]
pub struct ASYNMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASYNMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESOL_A {
    #[doc = "0: 6-bit resolution. An ADC conversion requires 9 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_6_BIT = 0,
    #[doc = "1: 8-bit resolution. An ADC conversion requires 11 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_8_BIT = 1,
    #[doc = "2: 10-bit resolution. An ADC conversion requires 13 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_10_BIT = 2,
    #[doc = "3: 12-bit resolution. An ADC conversion requires 15 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_12_BIT = 3,
}
impl From<RESOL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESOL`"]
pub type RESOL_R = crate::R<u8, RESOL_A>;
impl RESOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESOL_A {
        match self.bits {
            0 => RESOL_A::RESOLUTION_6_BIT,
            1 => RESOL_A::RESOLUTION_8_BIT,
            2 => RESOL_A::RESOLUTION_10_BIT,
            3 => RESOL_A::RESOLUTION_12_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_6_BIT`"]
    #[inline(always)]
    pub fn is_resolution_6_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_6_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_8_BIT`"]
    #[inline(always)]
    pub fn is_resolution_8_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_8_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_10_BIT`"]
    #[inline(always)]
    pub fn is_resolution_10_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_10_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_12_BIT`"]
    #[inline(always)]
    pub fn is_resolution_12_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_12_BIT
    }
}
#[doc = "Write proxy for field `RESOL`"]
pub struct RESOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESOL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "6-bit resolution. An ADC conversion requires 9 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_6_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_6_BIT)
    }
    #[doc = "8-bit resolution. An ADC conversion requires 11 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_8_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_8_BIT)
    }
    #[doc = "10-bit resolution. An ADC conversion requires 13 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_10_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_10_BIT)
    }
    #[doc = "12-bit resolution. An ADC conversion requires 15 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_12_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_12_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSCAL_A {
    #[doc = "0: Calibrate. The stored calibration value will be applied to the ADC during conversions to compensated for offset error. A calibration cycle must be performed each time the chip is powered-up. Re-calibration may be warranted periodically - especially if operating conditions have changed."]
    CALIBRATE = 0,
    #[doc = "1: Bypass calibration. Calibration is not utilized. Less time is required when enabling the ADC - particularly following chip power-up. Attempts to launch a calibration cycle are blocked when this bit is set."]
    BYPASS_CALIBRATION = 1,
}
impl From<BYPASSCAL_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYPASSCAL`"]
pub type BYPASSCAL_R = crate::R<bool, BYPASSCAL_A>;
impl BYPASSCAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSCAL_A {
        match self.bits {
            false => BYPASSCAL_A::CALIBRATE,
            true => BYPASSCAL_A::BYPASS_CALIBRATION,
        }
    }
    #[doc = "Checks if the value of the field is `CALIBRATE`"]
    #[inline(always)]
    pub fn is_calibrate(&self) -> bool {
        *self == BYPASSCAL_A::CALIBRATE
    }
    #[doc = "Checks if the value of the field is `BYPASS_CALIBRATION`"]
    #[inline(always)]
    pub fn is_bypass_calibration(&self) -> bool {
        *self == BYPASSCAL_A::BYPASS_CALIBRATION
    }
}
#[doc = "Write proxy for field `BYPASSCAL`"]
pub struct BYPASSCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSCAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calibrate. The stored calibration value will be applied to the ADC during conversions to compensated for offset error. A calibration cycle must be performed each time the chip is powered-up. Re-calibration may be warranted periodically - especially if operating conditions have changed."]
    #[inline(always)]
    pub fn calibrate(self) -> &'a mut W {
        self.variant(BYPASSCAL_A::CALIBRATE)
    }
    #[doc = "Bypass calibration. Calibration is not utilized. Less time is required when enabling the ADC - particularly following chip power-up. Attempts to launch a calibration cycle are blocked when this bit is set."]
    #[inline(always)]
    pub fn bypass_calibration(self) -> &'a mut W {
        self.variant(BYPASSCAL_A::BYPASS_CALIBRATION)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TSAMP`"]
pub type TSAMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSAMP`"]
pub struct TSAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
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
    #[doc = "Bits 9:10 - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
    #[inline(always)]
    pub fn resol(&self) -> RESOL_R {
        RESOL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
    #[inline(always)]
    pub fn bypasscal(&self) -> BYPASSCAL_R {
        BYPASSCAL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
    #[inline(always)]
    pub fn tsamp(&self) -> TSAMP_R {
        TSAMP_R::new(((self.bits >> 12) & 0x07) as u8)
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
    #[doc = "Bits 9:10 - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
    #[inline(always)]
    pub fn resol(&mut self) -> RESOL_W {
        RESOL_W { w: self }
    }
    #[doc = "Bit 11 - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
    #[inline(always)]
    pub fn bypasscal(&mut self) -> BYPASSCAL_W {
        BYPASSCAL_W { w: self }
    }
    #[doc = "Bits 12:14 - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
    #[inline(always)]
    pub fn tsamp(&mut self) -> TSAMP_W {
        TSAMP_W { w: self }
    }
}
