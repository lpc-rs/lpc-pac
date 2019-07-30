#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO2__ {
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
pub struct FUNCR {
    bits: u8,
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE_NO_PULL_DO,
    #[doc = "Pull-down resistor enabled."]
    PULL_DOWN_RESISTOR_E,
    #[doc = "Pull-up resistor enabled."]
    PULL_UP_RESISTOR_ENA,
    #[doc = "Repeater mode."]
    REPEATER_MODE,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE_NO_PULL_DO => 0,
            MODER::PULL_DOWN_RESISTOR_E => 1,
            MODER::PULL_UP_RESISTOR_ENA => 2,
            MODER::REPEATER_MODE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::INACTIVE_NO_PULL_DO,
            1 => MODER::PULL_DOWN_RESISTOR_E,
            2 => MODER::PULL_UP_RESISTOR_ENA,
            3 => MODER::REPEATER_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_NO_PULL_DO`"]
    #[inline]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == MODER::INACTIVE_NO_PULL_DO
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_RESISTOR_E`"]
    #[inline]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == MODER::PULL_DOWN_RESISTOR_E
    }
    #[doc = "Checks if the value of the field is `PULL_UP_RESISTOR_ENA`"]
    #[inline]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == MODER::PULL_UP_RESISTOR_ENA
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE`"]
    #[inline]
    pub fn is_repeater_mode(&self) -> bool {
        *self == MODER::REPEATER_MODE
    }
}
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enable."]
    ENABLE,
}
impl HYSR {
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
            HYSR::DISABLE => false,
            HYSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSR {
        match value {
            false => HYSR::DISABLE,
            true => HYSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HYSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HYSR::ENABLE
    }
}
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVR {
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
            INVR::INPUT_NOT_INVERTED => false,
            INVR::INPUT_INVERTED_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::INPUT_NOT_INVERTED,
            true => INVR::INPUT_INVERTED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED`"]
    #[inline]
    pub fn is_input_not_inverted(&self) -> bool {
        *self == INVR::INPUT_NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INVR::INPUT_INVERTED_HIGH
    }
}
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enabled. Open-drain mode enabled.  This is not a true open-drain mode."]
    ENABLED,
}
impl ODR {
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
            ODR::DISABLE => false,
            ODR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODR {
        match value {
            false => ODR::DISABLE,
            true => ODR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ODR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ODR::ENABLED
    }
}
#[doc = "Possible values of the field `S_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_MODER {
    #[doc = "Bypass input filter."]
    BYPASS_INPUT_FILTER,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    _1_CLOCK_CYCLE,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    _2_CLOCK_CYCLES,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    _3_CLOCK_CYCLES,
}
impl S_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S_MODER::BYPASS_INPUT_FILTER => 0,
            S_MODER::_1_CLOCK_CYCLE => 1,
            S_MODER::_2_CLOCK_CYCLES => 2,
            S_MODER::_3_CLOCK_CYCLES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S_MODER {
        match value {
            0 => S_MODER::BYPASS_INPUT_FILTER,
            1 => S_MODER::_1_CLOCK_CYCLE,
            2 => S_MODER::_2_CLOCK_CYCLES,
            3 => S_MODER::_3_CLOCK_CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS_INPUT_FILTER`"]
    #[inline]
    pub fn is_bypass_input_filter(&self) -> bool {
        *self == S_MODER::BYPASS_INPUT_FILTER
    }
    #[doc = "Checks if the value of the field is `_1_CLOCK_CYCLE`"]
    #[inline]
    pub fn is_1_clock_cycle(&self) -> bool {
        *self == S_MODER::_1_CLOCK_CYCLE
    }
    #[doc = "Checks if the value of the field is `_2_CLOCK_CYCLES`"]
    #[inline]
    pub fn is_2_clock_cycles(&self) -> bool {
        *self == S_MODER::_2_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `_3_CLOCK_CYCLES`"]
    #[inline]
    pub fn is_3_clock_cycles(&self) -> bool {
        *self == S_MODER::_3_CLOCK_CYCLES
    }
}
#[doc = "Possible values of the field `CLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKDIVR {
    #[doc = "IOCONCLKDIV0. Use IOCON clock divider 0."]
    IOCONCLKDIV0,
    #[doc = "IOCONCLKDIV1. Use IOCON clock divider 1."]
    IOCONCLKDIV1,
    #[doc = "IOCONCLKDIV2 Use IOCON clock divider 2."]
    IOCONCLKDIV2,
    #[doc = "IOCONCLKDIV3. Use IOCON clock divider 3."]
    IOCONCLKDIV3,
    #[doc = "IOCONCLKDIV4. Use IOCON clock divider 4."]
    IOCONCLKDIV4,
    #[doc = "IOCONCLKDIV5. Use IOCON clock divider 5."]
    IOCONCLKDIV5,
    #[doc = "IOCONCLKDIV6. Use IOCON clock divider 6."]
    IOCONCLKDIV6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKDIVR::IOCONCLKDIV0 => 0,
            CLKDIVR::IOCONCLKDIV1 => 1,
            CLKDIVR::IOCONCLKDIV2 => 2,
            CLKDIVR::IOCONCLKDIV3 => 3,
            CLKDIVR::IOCONCLKDIV4 => 4,
            CLKDIVR::IOCONCLKDIV5 => 5,
            CLKDIVR::IOCONCLKDIV6 => 6,
            CLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKDIVR {
        match value {
            0 => CLKDIVR::IOCONCLKDIV0,
            1 => CLKDIVR::IOCONCLKDIV1,
            2 => CLKDIVR::IOCONCLKDIV2,
            3 => CLKDIVR::IOCONCLKDIV3,
            4 => CLKDIVR::IOCONCLKDIV4,
            5 => CLKDIVR::IOCONCLKDIV5,
            6 => CLKDIVR::IOCONCLKDIV6,
            i => CLKDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV0`"]
    #[inline]
    pub fn is_ioconclkdiv0(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV0
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV1`"]
    #[inline]
    pub fn is_ioconclkdiv1(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV1
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV2`"]
    #[inline]
    pub fn is_ioconclkdiv2(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV2
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV3`"]
    #[inline]
    pub fn is_ioconclkdiv3(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV3
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV4`"]
    #[inline]
    pub fn is_ioconclkdiv4(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV4
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV5`"]
    #[inline]
    pub fn is_ioconclkdiv5(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV5
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV6`"]
    #[inline]
    pub fn is_ioconclkdiv6(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV6
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE_NO_PULL_DO,
    #[doc = "Pull-down resistor enabled."]
    PULL_DOWN_RESISTOR_E,
    #[doc = "Pull-up resistor enabled."]
    PULL_UP_RESISTOR_ENA,
    #[doc = "Repeater mode."]
    REPEATER_MODE,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE_NO_PULL_DO => 0,
            MODEW::PULL_DOWN_RESISTOR_E => 1,
            MODEW::PULL_UP_RESISTOR_ENA => 2,
            MODEW::REPEATER_MODE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline]
    pub fn inactive_no_pull_do(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE_NO_PULL_DO)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline]
    pub fn pull_down_resistor_e(self) -> &'a mut W {
        self.variant(MODEW::PULL_DOWN_RESISTOR_E)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline]
    pub fn pull_up_resistor_ena(self) -> &'a mut W {
        self.variant(MODEW::PULL_UP_RESISTOR_ENA)
    }
    #[doc = "Repeater mode."]
    #[inline]
    pub fn repeater_mode(self) -> &'a mut W {
        self.variant(MODEW::REPEATER_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HYS`"]
pub enum HYSW {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enable."]
    ENABLE,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSW::DISABLE => false,
            HYSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYSW::DISABLE)
    }
    #[doc = "Enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYSW::ENABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INPUT_NOT_INVERTED => false,
            INVW::INPUT_INVERTED_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    #[inline]
    pub fn input_not_inverted(self) -> &'a mut W {
        self.variant(INVW::INPUT_NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline]
    pub fn input_inverted_high(self) -> &'a mut W {
        self.variant(INVW::INPUT_INVERTED_HIGH)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OD`"]
pub enum ODW {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enabled. Open-drain mode enabled.  This is not a true open-drain mode."]
    ENABLED,
}
impl ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODW::DISABLE => false,
            ODW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODW<'a> {
    w: &'a mut W,
}
impl<'a> _ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ODW::DISABLE)
    }
    #[doc = "Enabled. Open-drain mode enabled. This is not a true open-drain mode."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ODW::ENABLED)
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
#[doc = "Values that can be written to the field `S_MODE`"]
pub enum S_MODEW {
    #[doc = "Bypass input filter."]
    BYPASS_INPUT_FILTER,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    _1_CLOCK_CYCLE,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    _2_CLOCK_CYCLES,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    _3_CLOCK_CYCLES,
}
impl S_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S_MODEW::BYPASS_INPUT_FILTER => 0,
            S_MODEW::_1_CLOCK_CYCLE => 1,
            S_MODEW::_2_CLOCK_CYCLES => 2,
            S_MODEW::_3_CLOCK_CYCLES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass input filter."]
    #[inline]
    pub fn bypass_input_filter(self) -> &'a mut W {
        self.variant(S_MODEW::BYPASS_INPUT_FILTER)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline]
    pub fn _1_clock_cycle(self) -> &'a mut W {
        self.variant(S_MODEW::_1_CLOCK_CYCLE)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline]
    pub fn _2_clock_cycles(self) -> &'a mut W {
        self.variant(S_MODEW::_2_CLOCK_CYCLES)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline]
    pub fn _3_clock_cycles(self) -> &'a mut W {
        self.variant(S_MODEW::_3_CLOCK_CYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKDIV`"]
pub enum CLKDIVW {
    #[doc = "IOCONCLKDIV0. Use IOCON clock divider 0."]
    IOCONCLKDIV0,
    #[doc = "IOCONCLKDIV1. Use IOCON clock divider 1."]
    IOCONCLKDIV1,
    #[doc = "IOCONCLKDIV2 Use IOCON clock divider 2."]
    IOCONCLKDIV2,
    #[doc = "IOCONCLKDIV3. Use IOCON clock divider 3."]
    IOCONCLKDIV3,
    #[doc = "IOCONCLKDIV4. Use IOCON clock divider 4."]
    IOCONCLKDIV4,
    #[doc = "IOCONCLKDIV5. Use IOCON clock divider 5."]
    IOCONCLKDIV5,
    #[doc = "IOCONCLKDIV6. Use IOCON clock divider 6."]
    IOCONCLKDIV6,
}
impl CLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKDIVW::IOCONCLKDIV0 => 0,
            CLKDIVW::IOCONCLKDIV1 => 1,
            CLKDIVW::IOCONCLKDIV2 => 2,
            CLKDIVW::IOCONCLKDIV3 => 3,
            CLKDIVW::IOCONCLKDIV4 => 4,
            CLKDIVW::IOCONCLKDIV5 => 5,
            CLKDIVW::IOCONCLKDIV6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IOCONCLKDIV0. Use IOCON clock divider 0."]
    #[inline]
    pub fn ioconclkdiv0(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV0)
    }
    #[doc = "IOCONCLKDIV1. Use IOCON clock divider 1."]
    #[inline]
    pub fn ioconclkdiv1(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV1)
    }
    #[doc = "IOCONCLKDIV2 Use IOCON clock divider 2."]
    #[inline]
    pub fn ioconclkdiv2(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV2)
    }
    #[doc = "IOCONCLKDIV3. Use IOCON clock divider 3."]
    #[inline]
    pub fn ioconclkdiv3(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV3)
    }
    #[doc = "IOCONCLKDIV4. Use IOCON clock divider 4."]
    #[inline]
    pub fn ioconclkdiv4(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV4)
    }
    #[doc = "IOCONCLKDIV5. Use IOCON clock divider 5."]
    #[inline]
    pub fn ioconclkdiv5(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV5)
    }
    #[doc = "IOCONCLKDIV6. Use IOCON clock divider 6."]
    #[inline]
    pub fn ioconclkdiv6(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:2 - Selects pin function."]
    #[inline]
    pub fn func(&self) -> FUNCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FUNCR { bits }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline]
    pub fn hys(&self) -> HYSR {
        HYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline]
    pub fn od(&self) -> ODR {
        ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline]
    pub fn s_mode(&self) -> S_MODER {
        S_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock IOCONCLKDIV. Value 0x7 is reserved."]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        CLKDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 144 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function."]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline]
    pub fn hys(&mut self) -> _HYSW {
        _HYSW { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline]
    pub fn od(&mut self) -> _ODW {
        _ODW { w: self }
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline]
    pub fn s_mode(&mut self) -> _S_MODEW {
        _S_MODEW { w: self }
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock IOCONCLKDIV. Value 0x7 is reserved."]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
}
