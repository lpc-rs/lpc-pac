#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EV_CTRL {
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
pub struct MATCHSELR {
    bits: u8,
}
impl MATCHSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEVENTR {
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    SELECTS_THE_L_STATE,
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    SELECTS_THE_H_STATE,
}
impl HEVENTR {
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
            HEVENTR::SELECTS_THE_L_STATE => false,
            HEVENTR::SELECTS_THE_H_STATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HEVENTR {
        match value {
            false => HEVENTR::SELECTS_THE_L_STATE,
            true => HEVENTR::SELECTS_THE_H_STATE,
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_L_STATE`"]
    #[inline]
    pub fn is_selects_the_l_state(&self) -> bool {
        *self == HEVENTR::SELECTS_THE_L_STATE
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_H_STATE`"]
    #[inline]
    pub fn is_selects_the_h_state(&self) -> bool {
        *self == HEVENTR::SELECTS_THE_H_STATE
    }
}
#[doc = "Possible values of the field `OUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSELR {
    #[doc = "Selects the inputs elected by IOSEL."]
    SELECTS_THE_INPUTS_E,
    #[doc = "Selects the outputs selected by IOSEL."]
    SELECTS_THE_OUTPUTS,
}
impl OUTSELR {
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
            OUTSELR::SELECTS_THE_INPUTS_E => false,
            OUTSELR::SELECTS_THE_OUTPUTS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTSELR {
        match value {
            false => OUTSELR::SELECTS_THE_INPUTS_E,
            true => OUTSELR::SELECTS_THE_OUTPUTS,
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INPUTS_E`"]
    #[inline]
    pub fn is_selects_the_inputs_e(&self) -> bool {
        *self == OUTSELR::SELECTS_THE_INPUTS_E
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_OUTPUTS`"]
    #[inline]
    pub fn is_selects_the_outputs(&self) -> bool {
        *self == OUTSELR::SELECTS_THE_OUTPUTS
    }
}
#[doc = r" Value of the field"]
pub struct IOSELR {
    bits: u8,
}
impl IOSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IOCOND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCONDR {
    #[doc = "LOW"]
    LOW,
    #[doc = "Rise"]
    RISE,
    #[doc = "Fall"]
    FALL,
    #[doc = "HIGH"]
    HIGH,
}
impl IOCONDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IOCONDR::LOW => 0,
            IOCONDR::RISE => 1,
            IOCONDR::FALL => 2,
            IOCONDR::HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IOCONDR {
        match value {
            0 => IOCONDR::LOW,
            1 => IOCONDR::RISE,
            2 => IOCONDR::FALL,
            3 => IOCONDR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IOCONDR::LOW
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == IOCONDR::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == IOCONDR::FALL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IOCONDR::HIGH
    }
}
#[doc = "Possible values of the field `COMBMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBMODER {
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    OR,
    #[doc = "MATCH. Uses the specified match only."]
    MATCH,
    #[doc = "IO. Uses the specified I/O condition only."]
    IO,
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    AND,
}
impl COMBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMBMODER::OR => 0,
            COMBMODER::MATCH => 1,
            COMBMODER::IO => 2,
            COMBMODER::AND => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMBMODER {
        match value {
            0 => COMBMODER::OR,
            1 => COMBMODER::MATCH,
            2 => COMBMODER::IO,
            3 => COMBMODER::AND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline]
    pub fn is_or(&self) -> bool {
        *self == COMBMODER::OR
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == COMBMODER::MATCH
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == COMBMODER::IO
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline]
    pub fn is_and(&self) -> bool {
        *self == COMBMODER::AND
    }
}
#[doc = "Possible values of the field `STATELD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATELDR {
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    STATEV_VALUE_IS_ADDE,
    #[doc = "STATEV value is loaded into STATE."]
    STATEV_VALUE_IS_LOAD,
}
impl STATELDR {
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
            STATELDR::STATEV_VALUE_IS_ADDE => false,
            STATELDR::STATEV_VALUE_IS_LOAD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATELDR {
        match value {
            false => STATELDR::STATEV_VALUE_IS_ADDE,
            true => STATELDR::STATEV_VALUE_IS_LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `STATEV_VALUE_IS_ADDE`"]
    #[inline]
    pub fn is_statev_value_is_adde(&self) -> bool {
        *self == STATELDR::STATEV_VALUE_IS_ADDE
    }
    #[doc = "Checks if the value of the field is `STATEV_VALUE_IS_LOAD`"]
    #[inline]
    pub fn is_statev_value_is_load(&self) -> bool {
        *self == STATELDR::STATEV_VALUE_IS_LOAD
    }
}
#[doc = r" Value of the field"]
pub struct STATEVR {
    bits: u8,
}
impl STATEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MATCHMEMR {
    bits: bool,
}
impl MATCHMEMR {
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
#[doc = "Possible values of the field `DIRECTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTIONR {
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    DIRECTION_INDEPENDEN,
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    COUNTING_UP,
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    COUNTING_DOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIRECTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIRECTIONR::DIRECTION_INDEPENDEN => 0,
            DIRECTIONR::COUNTING_UP => 1,
            DIRECTIONR::COUNTING_DOWN => 2,
            DIRECTIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIRECTIONR {
        match value {
            0 => DIRECTIONR::DIRECTION_INDEPENDEN,
            1 => DIRECTIONR::COUNTING_UP,
            2 => DIRECTIONR::COUNTING_DOWN,
            i => DIRECTIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIRECTION_INDEPENDEN`"]
    #[inline]
    pub fn is_direction_independen(&self) -> bool {
        *self == DIRECTIONR::DIRECTION_INDEPENDEN
    }
    #[doc = "Checks if the value of the field is `COUNTING_UP`"]
    #[inline]
    pub fn is_counting_up(&self) -> bool {
        *self == DIRECTIONR::COUNTING_UP
    }
    #[doc = "Checks if the value of the field is `COUNTING_DOWN`"]
    #[inline]
    pub fn is_counting_down(&self) -> bool {
        *self == DIRECTIONR::COUNTING_DOWN
    }
}
#[doc = r" Proxy"]
pub struct _MATCHSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HEVENT`"]
pub enum HEVENTW {
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    SELECTS_THE_L_STATE,
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    SELECTS_THE_H_STATE,
}
impl HEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HEVENTW::SELECTS_THE_L_STATE => false,
            HEVENTW::SELECTS_THE_H_STATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _HEVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HEVENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    #[inline]
    pub fn selects_the_l_state(self) -> &'a mut W {
        self.variant(HEVENTW::SELECTS_THE_L_STATE)
    }
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    #[inline]
    pub fn selects_the_h_state(self) -> &'a mut W {
        self.variant(HEVENTW::SELECTS_THE_H_STATE)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUTSEL`"]
pub enum OUTSELW {
    #[doc = "Selects the inputs elected by IOSEL."]
    SELECTS_THE_INPUTS_E,
    #[doc = "Selects the outputs selected by IOSEL."]
    SELECTS_THE_OUTPUTS,
}
impl OUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUTSELW::SELECTS_THE_INPUTS_E => false,
            OUTSELW::SELECTS_THE_OUTPUTS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects the inputs elected by IOSEL."]
    #[inline]
    pub fn selects_the_inputs_e(self) -> &'a mut W {
        self.variant(OUTSELW::SELECTS_THE_INPUTS_E)
    }
    #[doc = "Selects the outputs selected by IOSEL."]
    #[inline]
    pub fn selects_the_outputs(self) -> &'a mut W {
        self.variant(OUTSELW::SELECTS_THE_OUTPUTS)
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
#[doc = r" Proxy"]
pub struct _IOSELW<'a> {
    w: &'a mut W,
}
impl<'a> _IOSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOCOND`"]
pub enum IOCONDW {
    #[doc = "LOW"]
    LOW,
    #[doc = "Rise"]
    RISE,
    #[doc = "Fall"]
    FALL,
    #[doc = "HIGH"]
    HIGH,
}
impl IOCONDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOCONDW::LOW => 0,
            IOCONDW::RISE => 1,
            IOCONDW::FALL => 2,
            IOCONDW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCONDW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCONDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCONDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LOW"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(IOCONDW::LOW)
    }
    #[doc = "Rise"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(IOCONDW::RISE)
    }
    #[doc = "Fall"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(IOCONDW::FALL)
    }
    #[doc = "HIGH"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(IOCONDW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMBMODE`"]
pub enum COMBMODEW {
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    OR,
    #[doc = "MATCH. Uses the specified match only."]
    MATCH,
    #[doc = "IO. Uses the specified I/O condition only."]
    IO,
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    AND,
}
impl COMBMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMBMODEW::OR => 0,
            COMBMODEW::MATCH => 1,
            COMBMODEW::IO => 2,
            COMBMODEW::AND => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMBMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMBMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    #[inline]
    pub fn or(self) -> &'a mut W {
        self.variant(COMBMODEW::OR)
    }
    #[doc = "MATCH. Uses the specified match only."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(COMBMODEW::MATCH)
    }
    #[doc = "IO. Uses the specified I/O condition only."]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(COMBMODEW::IO)
    }
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    #[inline]
    pub fn and(self) -> &'a mut W {
        self.variant(COMBMODEW::AND)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATELD`"]
pub enum STATELDW {
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    STATEV_VALUE_IS_ADDE,
    #[doc = "STATEV value is loaded into STATE."]
    STATEV_VALUE_IS_LOAD,
}
impl STATELDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATELDW::STATEV_VALUE_IS_ADDE => false,
            STATELDW::STATEV_VALUE_IS_LOAD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATELDW<'a> {
    w: &'a mut W,
}
impl<'a> _STATELDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATELDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    #[inline]
    pub fn statev_value_is_adde(self) -> &'a mut W {
        self.variant(STATELDW::STATEV_VALUE_IS_ADDE)
    }
    #[doc = "STATEV value is loaded into STATE."]
    #[inline]
    pub fn statev_value_is_load(self) -> &'a mut W {
        self.variant(STATELDW::STATEV_VALUE_IS_LOAD)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STATEVW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MATCHMEMW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHMEMW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIRECTION`"]
pub enum DIRECTIONW {
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    DIRECTION_INDEPENDEN,
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    COUNTING_UP,
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    COUNTING_DOWN,
}
impl DIRECTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIRECTIONW::DIRECTION_INDEPENDEN => 0,
            DIRECTIONW::COUNTING_UP => 1,
            DIRECTIONW::COUNTING_DOWN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRECTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRECTIONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    #[inline]
    pub fn direction_independen(self) -> &'a mut W {
        self.variant(DIRECTIONW::DIRECTION_INDEPENDEN)
    }
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    #[inline]
    pub fn counting_up(self) -> &'a mut W {
        self.variant(DIRECTIONW::COUNTING_UP)
    }
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    #[inline]
    pub fn counting_down(self) -> &'a mut W {
        self.variant(DIRECTIONW::COUNTING_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline]
    pub fn matchsel(&self) -> MATCHSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MATCHSELR { bits }
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline]
    pub fn hevent(&self) -> HEVENTR {
        HEVENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline]
    pub fn outsel(&self) -> OUTSELR {
        OUTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number (0 to 3 for inputs or 0 to 5 for outputs) associated with this event (if any). Do not select an input in this register, if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline]
    pub fn iosel(&self) -> IOSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IOSELR { bits }
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline]
    pub fn iocond(&self) -> IOCONDR {
        IOCONDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline]
    pub fn combmode(&self) -> COMBMODER {
        COMBMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline]
    pub fn stateld(&self) -> STATELDR {
        STATELDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline]
    pub fn statev(&self) -> STATEVR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STATEVR { bits }
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline]
    pub fn matchmem(&self) -> MATCHMEMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MATCHMEMR { bits }
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline]
    pub fn direction(&self) -> DIRECTIONR {
        DIRECTIONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline]
    pub fn matchsel(&mut self) -> _MATCHSELW {
        _MATCHSELW { w: self }
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline]
    pub fn hevent(&mut self) -> _HEVENTW {
        _HEVENTW { w: self }
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline]
    pub fn outsel(&mut self) -> _OUTSELW {
        _OUTSELW { w: self }
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number (0 to 3 for inputs or 0 to 5 for outputs) associated with this event (if any). Do not select an input in this register, if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline]
    pub fn iosel(&mut self) -> _IOSELW {
        _IOSELW { w: self }
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline]
    pub fn iocond(&mut self) -> _IOCONDW {
        _IOCONDW { w: self }
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline]
    pub fn combmode(&mut self) -> _COMBMODEW {
        _COMBMODEW { w: self }
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline]
    pub fn stateld(&mut self) -> _STATELDW {
        _STATELDW { w: self }
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline]
    pub fn statev(&mut self) -> _STATEVW {
        _STATEVW { w: self }
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline]
    pub fn matchmem(&mut self) -> _MATCHMEMW {
        _MATCHMEMW { w: self }
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline]
    pub fn direction(&mut self) -> _DIRECTIONW {
        _DIRECTIONW { w: self }
    }
}
