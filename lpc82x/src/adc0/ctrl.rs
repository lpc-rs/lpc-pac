#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `LPWRMODE`"]
pub type LPWRMODE_R = crate::R<bool, LPWRMODE_A>;
impl LPWRMODE_R {
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
        *self == LPWRMODE_A::LPWRMODE_0
    }
    #[doc = "Checks if the value of the field is `LPWRMODE_1`"]
    #[inline(always)]
    pub fn is_lpwrmode_1(&self) -> bool {
        *self == LPWRMODE_A::LPWRMODE_1
    }
}
#[doc = "Write proxy for field `LPWRMODE`"]
pub struct LPWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPWRMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CALMODE`"]
pub type CALMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALMODE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
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
}
