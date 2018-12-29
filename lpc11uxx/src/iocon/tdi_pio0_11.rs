#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TDI_PIO0_11 {
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
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "TDI."]
    TDI_,
    #[doc = "PIO0_11."]
    PIO0_11_,
    #[doc = "AD0."]
    AD0_,
    #[doc = "CT32B0_MAT3."]
    CT32B0_MAT3_,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::TDI_ => 0,
            FUNCR::PIO0_11_ => 1,
            FUNCR::AD0_ => 2,
            FUNCR::CT32B0_MAT3_ => 3,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::TDI_,
            1 => FUNCR::PIO0_11_,
            2 => FUNCR::AD0_,
            3 => FUNCR::CT32B0_MAT3_,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TDI_`"]
    #[inline]
    pub fn is_tdi_(&self) -> bool {
        *self == FUNCR::TDI_
    }
    #[doc = "Checks if the value of the field is `PIO0_11_`"]
    #[inline]
    pub fn is_pio0_11_(&self) -> bool {
        *self == FUNCR::PIO0_11_
    }
    #[doc = "Checks if the value of the field is `AD0_`"]
    #[inline]
    pub fn is_ad0_(&self) -> bool {
        *self == FUNCR::AD0_
    }
    #[doc = "Checks if the value of the field is `CT32B0_MAT3_`"]
    #[inline]
    pub fn is_ct32b0_mat3_(&self) -> bool {
        *self == FUNCR::CT32B0_MAT3_
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
    REPEATER_MODE_,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE_NO_PULL_DO => 0,
            MODER::PULL_DOWN_RESISTOR_E => 1,
            MODER::PULL_UP_RESISTOR_ENA => 2,
            MODER::REPEATER_MODE_ => 3,
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
            3 => MODER::REPEATER_MODE_,
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
    #[doc = "Checks if the value of the field is `REPEATER_MODE_`"]
    #[inline]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == MODER::REPEATER_MODE_
    }
}
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Enable."]
    ENABLE_,
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
            HYSR::DISABLE_ => false,
            HYSR::ENABLE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSR {
        match value {
            false => HYSR::DISABLE_,
            true => HYSR::ENABLE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_`"]
    #[inline]
    pub fn is_disable_(&self) -> bool {
        *self == HYSR::DISABLE_
    }
    #[doc = "Checks if the value of the field is `ENABLE_`"]
    #[inline]
    pub fn is_enable_(&self) -> bool {
        *self == HYSR::ENABLE_
    }
}
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED_,
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
            INVR::INPUT_NOT_INVERTED_ => false,
            INVR::INPUT_INVERTED_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::INPUT_NOT_INVERTED_,
            true => INVR::INPUT_INVERTED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED_`"]
    #[inline]
    pub fn is_input_not_inverted_(&self) -> bool {
        *self == INVR::INPUT_NOT_INVERTED_
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INVR::INPUT_INVERTED_HIGH
    }
}
#[doc = "Possible values of the field `ADMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMODER {
    #[doc = "Analog input mode."]
    ANALOG_INPUT_MODE_,
    #[doc = "Digital functional mode."]
    DIGITAL_FUNCTIONAL_M,
}
impl ADMODER {
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
            ADMODER::ANALOG_INPUT_MODE_ => false,
            ADMODER::DIGITAL_FUNCTIONAL_M => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMODER {
        match value {
            false => ADMODER::ANALOG_INPUT_MODE_,
            true => ADMODER::DIGITAL_FUNCTIONAL_M,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT_MODE_`"]
    #[inline]
    pub fn is_analog_input_mode_(&self) -> bool {
        *self == ADMODER::ANALOG_INPUT_MODE_
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTIONAL_M`"]
    #[inline]
    pub fn is_digital_functional_m(&self) -> bool {
        *self == ADMODER::DIGITAL_FUNCTIONAL_M
    }
}
#[doc = "Possible values of the field `FILTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTRR {
    #[doc = "Filter enabled."]
    FILTER_ENABLED_,
    #[doc = "Filter disabled."]
    FILTER_DISABLED_,
}
impl FILTRR {
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
            FILTRR::FILTER_ENABLED_ => false,
            FILTRR::FILTER_DISABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTRR {
        match value {
            false => FILTRR::FILTER_ENABLED_,
            true => FILTRR::FILTER_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_ENABLED_`"]
    #[inline]
    pub fn is_filter_enabled_(&self) -> bool {
        *self == FILTRR::FILTER_ENABLED_
    }
    #[doc = "Checks if the value of the field is `FILTER_DISABLED_`"]
    #[inline]
    pub fn is_filter_disabled_(&self) -> bool {
        *self == FILTRR::FILTER_DISABLED_
    }
}
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Open-drain mode enabled.  This is not a true open-drain mode."]
    OPEN_DRAIN_MODE_ENAB,
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
            ODR::DISABLE_ => false,
            ODR::OPEN_DRAIN_MODE_ENAB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODR {
        match value {
            false => ODR::DISABLE_,
            true => ODR::OPEN_DRAIN_MODE_ENAB,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_`"]
    #[inline]
    pub fn is_disable_(&self) -> bool {
        *self == ODR::DISABLE_
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_MODE_ENAB`"]
    #[inline]
    pub fn is_open_drain_mode_enab(&self) -> bool {
        *self == ODR::OPEN_DRAIN_MODE_ENAB
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "TDI."]
    TDI_,
    #[doc = "PIO0_11."]
    PIO0_11_,
    #[doc = "AD0."]
    AD0_,
    #[doc = "CT32B0_MAT3."]
    CT32B0_MAT3_,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::TDI_ => 0,
            FUNCW::PIO0_11_ => 1,
            FUNCW::AD0_ => 2,
            FUNCW::CT32B0_MAT3_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TDI."]
    #[inline]
    pub fn tdi_(self) -> &'a mut W {
        self.variant(FUNCW::TDI_)
    }
    #[doc = "PIO0_11."]
    #[inline]
    pub fn pio0_11_(self) -> &'a mut W {
        self.variant(FUNCW::PIO0_11_)
    }
    #[doc = "AD0."]
    #[inline]
    pub fn ad0_(self) -> &'a mut W {
        self.variant(FUNCW::AD0_)
    }
    #[doc = "CT32B0_MAT3."]
    #[inline]
    pub fn ct32b0_mat3_(self) -> &'a mut W {
        self.variant(FUNCW::CT32B0_MAT3_)
    }
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
    REPEATER_MODE_,
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
            MODEW::REPEATER_MODE_ => 3,
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
    pub fn repeater_mode_(self) -> &'a mut W {
        self.variant(MODEW::REPEATER_MODE_)
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
    DISABLE_,
    #[doc = "Enable."]
    ENABLE_,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSW::DISABLE_ => false,
            HYSW::ENABLE_ => true,
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
    pub fn disable_(self) -> &'a mut W {
        self.variant(HYSW::DISABLE_)
    }
    #[doc = "Enable."]
    #[inline]
    pub fn enable_(self) -> &'a mut W {
        self.variant(HYSW::ENABLE_)
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
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED_,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INPUT_NOT_INVERTED_ => false,
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
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline]
    pub fn input_not_inverted_(self) -> &'a mut W {
        self.variant(INVW::INPUT_NOT_INVERTED_)
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
#[doc = "Values that can be written to the field `ADMODE`"]
pub enum ADMODEW {
    #[doc = "Analog input mode."]
    ANALOG_INPUT_MODE_,
    #[doc = "Digital functional mode."]
    DIGITAL_FUNCTIONAL_M,
}
impl ADMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADMODEW::ANALOG_INPUT_MODE_ => false,
            ADMODEW::DIGITAL_FUNCTIONAL_M => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog input mode."]
    #[inline]
    pub fn analog_input_mode_(self) -> &'a mut W {
        self.variant(ADMODEW::ANALOG_INPUT_MODE_)
    }
    #[doc = "Digital functional mode."]
    #[inline]
    pub fn digital_functional_m(self) -> &'a mut W {
        self.variant(ADMODEW::DIGITAL_FUNCTIONAL_M)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTR`"]
pub enum FILTRW {
    #[doc = "Filter enabled."]
    FILTER_ENABLED_,
    #[doc = "Filter disabled."]
    FILTER_DISABLED_,
}
impl FILTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTRW::FILTER_ENABLED_ => false,
            FILTRW::FILTER_DISABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTRW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filter enabled."]
    #[inline]
    pub fn filter_enabled_(self) -> &'a mut W {
        self.variant(FILTRW::FILTER_ENABLED_)
    }
    #[doc = "Filter disabled."]
    #[inline]
    pub fn filter_disabled_(self) -> &'a mut W {
        self.variant(FILTRW::FILTER_DISABLED_)
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
#[doc = "Values that can be written to the field `OD`"]
pub enum ODW {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Open-drain mode enabled.  This is not a true open-drain mode."]
    OPEN_DRAIN_MODE_ENAB,
}
impl ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODW::DISABLE_ => false,
            ODW::OPEN_DRAIN_MODE_ENAB => true,
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
    pub fn disable_(self) -> &'a mut W {
        self.variant(ODW::DISABLE_)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode."]
    #[inline]
    pub fn open_drain_mode_enab(self) -> &'a mut W {
        self.variant(ODW::OPEN_DRAIN_MODE_ENAB)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selects pin function. Values 0x4 to 0x7 are reserved."]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline]
    pub fn admode(&self) -> ADMODER {
        ADMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline]
    pub fn filtr(&self) -> FILTRR {
        FILTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Selects pin function. Values 0x4 to 0x7 are reserved."]
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
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline]
    pub fn admode(&mut self) -> _ADMODEW {
        _ADMODEW { w: self }
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline]
    pub fn filtr(&mut self) -> _FILTRW {
        _FILTRW { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline]
    pub fn od(&mut self) -> _ODW {
        _ODW { w: self }
    }
}
