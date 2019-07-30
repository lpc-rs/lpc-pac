#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO1_ {
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
        0x90
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type FUNC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
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
impl crate::ToBits<u8> for MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE_NO_PULL_DO => 0,
            MODER::PULL_DOWN_RESISTOR_E => 1,
            MODER::PULL_UP_RESISTOR_ENA => 2,
            MODER::REPEATER_MODE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MODE_R = crate::FR<u8, MODER>;
impl MODE_R {
    #[doc = "Checks if the value of the field is `INACTIVE_NO_PULL_DO`"]
    #[inline(always)]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == MODER::INACTIVE_NO_PULL_DO
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_RESISTOR_E`"]
    #[inline(always)]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == MODER::PULL_DOWN_RESISTOR_E
    }
    #[doc = "Checks if the value of the field is `PULL_UP_RESISTOR_ENA`"]
    #[inline(always)]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == MODER::PULL_UP_RESISTOR_ENA
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE`"]
    #[inline(always)]
    pub fn is_repeater_mode(&self) -> bool {
        *self == MODER::REPEATER_MODE
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE_NO_PULL_DO => 0,
            MODEW::PULL_DOWN_RESISTOR_E => 1,
            MODEW::PULL_UP_RESISTOR_ENA => 2,
            MODEW::REPEATER_MODE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive_no_pull_do(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE_NO_PULL_DO)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down_resistor_e(self) -> &'a mut W {
        self.variant(MODEW::PULL_DOWN_RESISTOR_E)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up_resistor_ena(self) -> &'a mut W {
        self.variant(MODEW::PULL_UP_RESISTOR_ENA)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater_mode(self) -> &'a mut W {
        self.variant(MODEW::REPEATER_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
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
impl crate::ToBits<bool> for HYSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HYSR::DISABLE => false,
            HYSR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HYS_R = crate::FR<bool, HYSR>;
impl HYS_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYSR::ENABLE
    }
}
#[doc = "Values that can be written to the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSW {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enable."]
    ENABLE,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSW::DISABLE => false,
            HYSW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYSW::DISABLE)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYSW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
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
impl crate::ToBits<bool> for INVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INVR::INPUT_NOT_INVERTED => false,
            INVR::INPUT_INVERTED_HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INV_R = crate::FR<bool, INVR>;
impl INV_R {
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_input_not_inverted(&self) -> bool {
        *self == INVR::INPUT_NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline(always)]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INVR::INPUT_INVERTED_HIGH
    }
}
#[doc = "Values that can be written to the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INPUT_NOT_INVERTED => false,
            INVW::INPUT_INVERTED_HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn input_not_inverted(self) -> &'a mut W {
        self.variant(INVW::INPUT_NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn input_inverted_high(self) -> &'a mut W {
        self.variant(INVW::INPUT_INVERTED_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
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
impl crate::ToBits<bool> for ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ODR::DISABLE => false,
            ODR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OD_R = crate::FR<bool, ODR>;
impl OD_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ODR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ODR::ENABLED
    }
}
#[doc = "Values that can be written to the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODW {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enabled. Open-drain mode enabled.  This is not a true open-drain mode."]
    ENABLED,
}
impl ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ODW::DISABLE => false,
            ODW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ODW<'a> {
    w: &'a mut W,
}
impl<'a> _ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ODW::DISABLE)
    }
    #[doc = "Enabled. Open-drain mode enabled. This is not a true open-drain mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ODW::ENABLED)
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
impl crate::ToBits<u8> for S_MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            S_MODER::BYPASS_INPUT_FILTER => 0,
            S_MODER::_1_CLOCK_CYCLE => 1,
            S_MODER::_2_CLOCK_CYCLES => 2,
            S_MODER::_3_CLOCK_CYCLES => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type S_MODE_R = crate::FR<u8, S_MODER>;
impl S_MODE_R {
    #[doc = "Checks if the value of the field is `BYPASS_INPUT_FILTER`"]
    #[inline(always)]
    pub fn is_bypass_input_filter(&self) -> bool {
        *self == S_MODER::BYPASS_INPUT_FILTER
    }
    #[doc = "Checks if the value of the field is `_1_CLOCK_CYCLE`"]
    #[inline(always)]
    pub fn is_1_clock_cycle(&self) -> bool {
        *self == S_MODER::_1_CLOCK_CYCLE
    }
    #[doc = "Checks if the value of the field is `_2_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_2_clock_cycles(&self) -> bool {
        *self == S_MODER::_2_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `_3_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_3_clock_cycles(&self) -> bool {
        *self == S_MODER::_3_CLOCK_CYCLES
    }
}
#[doc = "Values that can be written to the field `S_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            S_MODEW::BYPASS_INPUT_FILTER => 0,
            S_MODEW::_1_CLOCK_CYCLE => 1,
            S_MODEW::_2_CLOCK_CYCLES => 2,
            S_MODEW::_3_CLOCK_CYCLES => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _S_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass input filter."]
    #[inline(always)]
    pub fn bypass_input_filter(self) -> &'a mut W {
        self.variant(S_MODEW::BYPASS_INPUT_FILTER)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline(always)]
    pub fn _1_clock_cycle(self) -> &'a mut W {
        self.variant(S_MODEW::_1_CLOCK_CYCLE)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline(always)]
    pub fn _2_clock_cycles(self) -> &'a mut W {
        self.variant(S_MODEW::_2_CLOCK_CYCLES)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline(always)]
    pub fn _3_clock_cycles(self) -> &'a mut W {
        self.variant(S_MODEW::_3_CLOCK_CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
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
}
impl crate::ToBits<u8> for CLKDIVR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKDIVR::IOCONCLKDIV0 => 0,
            CLKDIVR::IOCONCLKDIV1 => 1,
            CLKDIVR::IOCONCLKDIV2 => 2,
            CLKDIVR::IOCONCLKDIV3 => 3,
            CLKDIVR::IOCONCLKDIV4 => 4,
            CLKDIVR::IOCONCLKDIV5 => 5,
            CLKDIVR::IOCONCLKDIV6 => 6,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLKDIV_R = crate::FR<u8, CLKDIVR>;
impl CLKDIV_R {
    #[doc = "Checks if the value of the field is `IOCONCLKDIV0`"]
    #[inline(always)]
    pub fn is_ioconclkdiv0(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV0
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV1`"]
    #[inline(always)]
    pub fn is_ioconclkdiv1(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV1
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV2`"]
    #[inline(always)]
    pub fn is_ioconclkdiv2(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV2
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV3`"]
    #[inline(always)]
    pub fn is_ioconclkdiv3(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV3
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV4`"]
    #[inline(always)]
    pub fn is_ioconclkdiv4(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV4
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV5`"]
    #[inline(always)]
    pub fn is_ioconclkdiv5(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV5
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV6`"]
    #[inline(always)]
    pub fn is_ioconclkdiv6(&self) -> bool {
        *self == CLKDIVR::IOCONCLKDIV6
    }
}
#[doc = "Values that can be written to the field `CLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IOCONCLKDIV0. Use IOCON clock divider 0."]
    #[inline(always)]
    pub fn ioconclkdiv0(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV0)
    }
    #[doc = "IOCONCLKDIV1. Use IOCON clock divider 1."]
    #[inline(always)]
    pub fn ioconclkdiv1(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV1)
    }
    #[doc = "IOCONCLKDIV2 Use IOCON clock divider 2."]
    #[inline(always)]
    pub fn ioconclkdiv2(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV2)
    }
    #[doc = "IOCONCLKDIV3. Use IOCON clock divider 3."]
    #[inline(always)]
    pub fn ioconclkdiv3(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV3)
    }
    #[doc = "IOCONCLKDIV4. Use IOCON clock divider 4."]
    #[inline(always)]
    pub fn ioconclkdiv4(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV4)
    }
    #[doc = "IOCONCLKDIV5. Use IOCON clock divider 5."]
    #[inline(always)]
    pub fn ioconclkdiv5(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV5)
    }
    #[doc = "IOCONCLKDIV6. Use IOCON clock divider 6."]
    #[inline(always)]
    pub fn ioconclkdiv6(self) -> &'a mut W {
        self.variant(CLKDIVW::IOCONCLKDIV6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selects pin function."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits() >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline(always)]
    pub fn s_mode(&self) -> S_MODE_R {
        S_MODE_R::new(((self.bits() >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock IOCONCLKDIV. Value 0x7 is reserved."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits() >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function."]
    #[inline(always)]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> _HYSW {
        _HYSW { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> _ODW {
        _ODW { w: self }
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline(always)]
    pub fn s_mode(&mut self) -> _S_MODEW {
        _S_MODEW { w: self }
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock IOCONCLKDIV. Value 0x7 is reserved."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
}
