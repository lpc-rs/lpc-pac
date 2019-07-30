#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type CLKDIV_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Possible values of the field `LPWRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWRMODER {
    #[doc = "Disabled. The low-power ADC mode is disabled.  The analog circuitry remains activated even when no conversions are requested."]
    DISABLED,
    #[doc = "Enabled. The low-power ADC mode is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for LPWRMODER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LPWRMODER::DISABLED => false,
            LPWRMODER::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPWRMODE_R = crate::FR<bool, LPWRMODER>;
impl LPWRMODE_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPWRMODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPWRMODER::ENABLED
    }
}
#[doc = "Values that can be written to the field `LPWRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWRMODEW {
    #[doc = "Disabled. The low-power ADC mode is disabled.  The analog circuitry remains activated even when no conversions are requested."]
    DISABLED,
    #[doc = "Enabled. The low-power ADC mode is enabled."]
    ENABLED,
}
impl LPWRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LPWRMODEW::DISABLED => false,
            LPWRMODEW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LPWRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPWRMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPWRMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The low-power ADC mode is disabled. The analog circuitry remains activated even when no conversions are requested."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPWRMODEW::DISABLED)
    }
    #[doc = "Enabled. The low-power ADC mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPWRMODEW::ENABLED)
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
#[doc = r"Reader of the field"]
pub type CAL_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAL_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAL_MODEW<'a> {
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - The system clock is divided by this value plus one to produce the clock for the A/D converter, which should be less than or equal to 50 MHz (up to 100 MHz in 10-bit mode). Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bit 10 - Select low-power ADC mode. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. Using this mode can save an appreciable amount of current (approximately 2.5 mA) when conversions are required relatively infrequently. The penalty for using this mode is an approximately 15 ADC clock delay (30 clocks in 10-bit mode), based on the frequency specified in the CLKDIV field, from the time the trigger event occurs until sampling of the A/D input commences. This mode will NOT power-up the A/D if the ADC_ENA bit is low."]
    #[inline(always)]
    pub fn lpwrmode(&self) -> LPWRMODE_R {
        LPWRMODE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Writing a 1 to this bit initiates a self-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted until the full calibration cycle has ended."]
    #[inline(always)]
    pub fn cal_mode(&self) -> CAL_MODE_R {
        CAL_MODE_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - The system clock is divided by this value plus one to produce the clock for the A/D converter, which should be less than or equal to 50 MHz (up to 100 MHz in 10-bit mode). Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 10 - Select low-power ADC mode. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. Using this mode can save an appreciable amount of current (approximately 2.5 mA) when conversions are required relatively infrequently. The penalty for using this mode is an approximately 15 ADC clock delay (30 clocks in 10-bit mode), based on the frequency specified in the CLKDIV field, from the time the trigger event occurs until sampling of the A/D input commences. This mode will NOT power-up the A/D if the ADC_ENA bit is low."]
    #[inline(always)]
    pub fn lpwrmode(&mut self) -> _LPWRMODEW {
        _LPWRMODEW { w: self }
    }
    #[doc = "Bit 30 - Writing a 1 to this bit initiates a self-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted until the full calibration cycle has ended."]
    #[inline(always)]
    pub fn cal_mode(&mut self) -> _CAL_MODEW {
        _CAL_MODEW { w: self }
    }
}
