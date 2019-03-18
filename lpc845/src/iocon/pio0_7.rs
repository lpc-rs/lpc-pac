#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO0_7 {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. Repeater mode."]
    REPEATER,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE => 0,
            MODER::PULL_DOWN => 1,
            MODER::PULL_UP => 2,
            MODER::REPEATER => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::INACTIVE,
            1 => MODER::PULL_DOWN,
            2 => MODER::PULL_UP,
            3 => MODER::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == MODER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == MODER::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == MODER::REPEATER
    }
}
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
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
    NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED,
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
            INVR::NOT_INVERTED => false,
            INVR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::NOT_INVERTED,
            true => INVR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline]
    pub fn is_not_inverted(&self) -> bool {
        *self == INVR::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == INVR::INVERTED
    }
}
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Open-drain mode enabled. Remark: This is not a true open-drain mode."]
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
    S_MODE_0,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    S_MODE_1,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    S_MODE_2,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    S_MODE_3,
}
impl S_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S_MODER::S_MODE_0 => 0,
            S_MODER::S_MODE_1 => 1,
            S_MODER::S_MODE_2 => 2,
            S_MODER::S_MODE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S_MODER {
        match value {
            0 => S_MODER::S_MODE_0,
            1 => S_MODER::S_MODE_1,
            2 => S_MODER::S_MODE_2,
            3 => S_MODER::S_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S_MODE_0`"]
    #[inline]
    pub fn is_s_mode_0(&self) -> bool {
        *self == S_MODER::S_MODE_0
    }
    #[doc = "Checks if the value of the field is `S_MODE_1`"]
    #[inline]
    pub fn is_s_mode_1(&self) -> bool {
        *self == S_MODER::S_MODE_1
    }
    #[doc = "Checks if the value of the field is `S_MODE_2`"]
    #[inline]
    pub fn is_s_mode_2(&self) -> bool {
        *self == S_MODER::S_MODE_2
    }
    #[doc = "Checks if the value of the field is `S_MODE_3`"]
    #[inline]
    pub fn is_s_mode_3(&self) -> bool {
        *self == S_MODER::S_MODE_3
    }
}
#[doc = "Possible values of the field `CLK_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DIVR {
    #[doc = "IOCONCLKDIV0"]
    CLK_DIV_0,
    #[doc = "IOCONCLKDIV1"]
    CLK_DIV_1,
    #[doc = "IOCONCLKDIV2"]
    CLK_DIV_2,
    #[doc = "IOCONCLKDIV3"]
    CLK_DIV_3,
    #[doc = "IOCONCLKDIV4"]
    CLK_DIV_4,
    #[doc = "IOCONCLKDIV5"]
    CLK_DIV_5,
    #[doc = "IOCONCLKDIV6"]
    CLK_DIV_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_DIVR::CLK_DIV_0 => 0,
            CLK_DIVR::CLK_DIV_1 => 1,
            CLK_DIVR::CLK_DIV_2 => 2,
            CLK_DIVR::CLK_DIV_3 => 3,
            CLK_DIVR::CLK_DIV_4 => 4,
            CLK_DIVR::CLK_DIV_5 => 5,
            CLK_DIVR::CLK_DIV_6 => 6,
            CLK_DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_DIVR {
        match value {
            0 => CLK_DIVR::CLK_DIV_0,
            1 => CLK_DIVR::CLK_DIV_1,
            2 => CLK_DIVR::CLK_DIV_2,
            3 => CLK_DIVR::CLK_DIV_3,
            4 => CLK_DIVR::CLK_DIV_4,
            5 => CLK_DIVR::CLK_DIV_5,
            6 => CLK_DIVR::CLK_DIV_6,
            i => CLK_DIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_0`"]
    #[inline]
    pub fn is_clk_div_0(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_1`"]
    #[inline]
    pub fn is_clk_div_1(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_2`"]
    #[inline]
    pub fn is_clk_div_2(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_3`"]
    #[inline]
    pub fn is_clk_div_3(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_4`"]
    #[inline]
    pub fn is_clk_div_4(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_5`"]
    #[inline]
    pub fn is_clk_div_5(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_6`"]
    #[inline]
    pub fn is_clk_div_6(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_6
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. Repeater mode."]
    REPEATER,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE => 0,
            MODEW::PULL_DOWN => 1,
            MODEW::PULL_UP => 2,
            MODEW::REPEATER => 3,
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
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODEW::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODEW::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODEW::REPEATER)
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
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
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
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYSW::DISABLE)
    }
    #[doc = "Enable"]
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
    NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::NOT_INVERTED => false,
            INVW::INVERTED => true,
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
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(INVW::NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(INVW::INVERTED)
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
    #[doc = "Open-drain mode enabled. Remark: This is not a true open-drain mode."]
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
    #[doc = "Open-drain mode enabled. Remark: This is not a true open-drain mode."]
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
    S_MODE_0,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    S_MODE_1,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    S_MODE_2,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    S_MODE_3,
}
impl S_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S_MODEW::S_MODE_0 => 0,
            S_MODEW::S_MODE_1 => 1,
            S_MODEW::S_MODE_2 => 2,
            S_MODEW::S_MODE_3 => 3,
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
    pub fn s_mode_0(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_0)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline]
    pub fn s_mode_1(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_1)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline]
    pub fn s_mode_2(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_2)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline]
    pub fn s_mode_3(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_3)
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
#[doc = "Values that can be written to the field `CLK_DIV`"]
pub enum CLK_DIVW {
    #[doc = "IOCONCLKDIV0"]
    CLK_DIV_0,
    #[doc = "IOCONCLKDIV1"]
    CLK_DIV_1,
    #[doc = "IOCONCLKDIV2"]
    CLK_DIV_2,
    #[doc = "IOCONCLKDIV3"]
    CLK_DIV_3,
    #[doc = "IOCONCLKDIV4"]
    CLK_DIV_4,
    #[doc = "IOCONCLKDIV5"]
    CLK_DIV_5,
    #[doc = "IOCONCLKDIV6"]
    CLK_DIV_6,
}
impl CLK_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_DIVW::CLK_DIV_0 => 0,
            CLK_DIVW::CLK_DIV_1 => 1,
            CLK_DIVW::CLK_DIV_2 => 2,
            CLK_DIVW::CLK_DIV_3 => 3,
            CLK_DIVW::CLK_DIV_4 => 4,
            CLK_DIVW::CLK_DIV_5 => 5,
            CLK_DIVW::CLK_DIV_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_DIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IOCONCLKDIV0"]
    #[inline]
    pub fn clk_div_0(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_0)
    }
    #[doc = "IOCONCLKDIV1"]
    #[inline]
    pub fn clk_div_1(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_1)
    }
    #[doc = "IOCONCLKDIV2"]
    #[inline]
    pub fn clk_div_2(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_2)
    }
    #[doc = "IOCONCLKDIV3"]
    #[inline]
    pub fn clk_div_3(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_3)
    }
    #[doc = "IOCONCLKDIV4"]
    #[inline]
    pub fn clk_div_4(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_4)
    }
    #[doc = "IOCONCLKDIV5"]
    #[inline]
    pub fn clk_div_5(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_5)
    }
    #[doc = "IOCONCLKDIV6"]
    #[inline]
    pub fn clk_div_6(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_6)
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
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline]
    pub fn clk_div(&self) -> CLK_DIVR {
        CLK_DIVR::_from({
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
        W { bits: 176 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline]
    pub fn clk_div(&mut self) -> _CLK_DIVW {
        _CLK_DIVW { w: self }
    }
}
