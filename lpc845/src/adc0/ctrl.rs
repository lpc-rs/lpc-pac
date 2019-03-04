#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CLKDIVR {
    bits: u8,
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ASYNMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNMODER {
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    SYNCHRONOUS_MODE,
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    ASYNCHRONOUS_MODE,
}
impl ASYNMODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ASYNMODER::SYNCHRONOUS_MODE => false,
            ASYNMODER::ASYNCHRONOUS_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASYNMODER {
        match value {
            false => ASYNMODER::SYNCHRONOUS_MODE,
            true => ASYNMODER::ASYNCHRONOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE`"]
    #[inline]
    pub fn is_synchronous_mode(&self) -> bool {
        *self == ASYNMODER::SYNCHRONOUS_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == ASYNMODER::ASYNCHRONOUS_MODE
    }
}
#[doc = "Possible values of the field `LPWRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWRMODER {
    #[doc = "The low-power ADC mode is disabled. The analog circuitry remains activated even when no conversions are requested."]
    LPWRMODE_0,
    #[doc = "The low-power ADC mode is enabled. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. Using this mode can save an appreciable amount of current (approximately 2.5 mA) when conversions are required relatively infrequently. The penalty for using this mode is an approximately FIFTEEN ADC CLOCK delay (30 clocks in 10-bit mode), based on the frequency specified in the CLKDIV field, from the time the trigger event occurs until sampling of the A/D input commences. Note: This mode will NOT power-up the A/D if the ADC_ENA bit is low."]
    LPWRMODE_1,
}
impl LPWRMODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPWRMODER::LPWRMODE_0 => false,
            LPWRMODER::LPWRMODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPWRMODER {
        match value {
            false => LPWRMODER::LPWRMODE_0,
            true => LPWRMODER::LPWRMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPWRMODE_0`"]
    #[inline]
    pub fn is_lpwrmode_0(&self) -> bool {
        *self == LPWRMODER::LPWRMODE_0
    }
    #[doc = "Checks if the value of the field is `LPWRMODE_1`"]
    #[inline]
    pub fn is_lpwrmode_1(&self) -> bool {
        *self == LPWRMODER::LPWRMODE_1
    }
}
#[doc = r" Value of the field"]
pub struct CALMODER {
    bits: bool,
}
impl CALMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASYNMODE`"]
pub enum ASYNMODEW {
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    SYNCHRONOUS_MODE,
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    ASYNCHRONOUS_MODE,
}
impl ASYNMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASYNMODEW::SYNCHRONOUS_MODE => false,
            ASYNMODEW::ASYNCHRONOUS_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASYNMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASYNMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    #[inline]
    pub fn synchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODEW::SYNCHRONOUS_MODE)
    }
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    #[inline]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODEW::ASYNCHRONOUS_MODE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPWRMODE`"]
pub enum LPWRMODEW {
    #[doc = "The low-power ADC mode is disabled. The analog circuitry remains activated even when no conversions are requested."]
    LPWRMODE_0,
    #[doc = "The low-power ADC mode is enabled. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. Using this mode can save an appreciable amount of current (approximately 2.5 mA) when conversions are required relatively infrequently. The penalty for using this mode is an approximately FIFTEEN ADC CLOCK delay (30 clocks in 10-bit mode), based on the frequency specified in the CLKDIV field, from the time the trigger event occurs until sampling of the A/D input commences. Note: This mode will NOT power-up the A/D if the ADC_ENA bit is low."]
    LPWRMODE_1,
}
impl LPWRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPWRMODEW::LPWRMODE_0 => false,
            LPWRMODEW::LPWRMODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPWRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPWRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPWRMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The low-power ADC mode is disabled. The analog circuitry remains activated even when no conversions are requested."]
    #[inline]
    pub fn lpwrmode_0(self) -> &'a mut W {
        self.variant(LPWRMODEW::LPWRMODE_0)
    }
    #[doc = "The low-power ADC mode is enabled. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. Using this mode can save an appreciable amount of current (approximately 2.5 mA) when conversions are required relatively infrequently. The penalty for using this mode is an approximately FIFTEEN ADC CLOCK delay (30 clocks in 10-bit mode), based on the frequency specified in the CLKDIV field, from the time the trigger event occurs until sampling of the A/D input commences. Note: This mode will NOT power-up the A/D if the ADC_ENA bit is low."]
    #[inline]
    pub fn lpwrmode_1(self) -> &'a mut W {
        self.variant(LPWRMODEW::LPWRMODE_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CALMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CALMODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKDIVR { bits }
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline]
    pub fn asynmode(&self) -> ASYNMODER {
        ASYNMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - The low-power ADC mode"]
    #[inline]
    pub fn lpwrmode(&self) -> LPWRMODER {
        LPWRMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Writing a '1' to this bit will initiate a sef-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Note: Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted unitl the full calibration cycle has ended."]
    #[inline]
    pub fn calmode(&self) -> CALMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CALMODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1536 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline]
    pub fn asynmode(&mut self) -> _ASYNMODEW {
        _ASYNMODEW { w: self }
    }
    #[doc = "Bit 10 - The low-power ADC mode"]
    #[inline]
    pub fn lpwrmode(&mut self) -> _LPWRMODEW {
        _LPWRMODEW { w: self }
    }
    #[doc = "Bit 30 - Writing a '1' to this bit will initiate a sef-calibration cycle. This bit will be automatically cleared by hardware after the calibration cycle is complete. Note: Other bits of this register may be written to concurrently with setting this bit, however once this bit has been set no further writes to this register are permitted unitl the full calibration cycle has ended."]
    #[inline]
    pub fn calmode(&mut self) -> _CALMODEW {
        _CALMODEW { w: self }
    }
}
