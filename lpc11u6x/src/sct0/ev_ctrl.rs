#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EV_CTRL {
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
pub type MATCHSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MATCHSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
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
impl crate::ToBits<bool> for HEVENTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HEVENTR::SELECTS_THE_L_STATE => false,
            HEVENTR::SELECTS_THE_H_STATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HEVENT_R = crate::FR<bool, HEVENTR>;
impl HEVENT_R {
    #[doc = "Checks if the value of the field is `SELECTS_THE_L_STATE`"]
    #[inline(always)]
    pub fn is_selects_the_l_state(&self) -> bool {
        *self == HEVENTR::SELECTS_THE_L_STATE
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_H_STATE`"]
    #[inline(always)]
    pub fn is_selects_the_h_state(&self) -> bool {
        *self == HEVENTR::SELECTS_THE_H_STATE
    }
}
#[doc = "Values that can be written to the field `HEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEVENTW {
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    SELECTS_THE_L_STATE,
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    SELECTS_THE_H_STATE,
}
impl HEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HEVENTW::SELECTS_THE_L_STATE => false,
            HEVENTW::SELECTS_THE_H_STATE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _HEVENTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HEVENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn selects_the_l_state(self) -> &'a mut W {
        self.variant(HEVENTW::SELECTS_THE_L_STATE)
    }
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn selects_the_h_state(self) -> &'a mut W {
        self.variant(HEVENTW::SELECTS_THE_H_STATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
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
impl crate::ToBits<bool> for OUTSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OUTSELR::SELECTS_THE_INPUTS_E => false,
            OUTSELR::SELECTS_THE_OUTPUTS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OUTSEL_R = crate::FR<bool, OUTSELR>;
impl OUTSEL_R {
    #[doc = "Checks if the value of the field is `SELECTS_THE_INPUTS_E`"]
    #[inline(always)]
    pub fn is_selects_the_inputs_e(&self) -> bool {
        *self == OUTSELR::SELECTS_THE_INPUTS_E
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_OUTPUTS`"]
    #[inline(always)]
    pub fn is_selects_the_outputs(&self) -> bool {
        *self == OUTSELR::SELECTS_THE_OUTPUTS
    }
}
#[doc = "Values that can be written to the field `OUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSELW {
    #[doc = "Selects the inputs elected by IOSEL."]
    SELECTS_THE_INPUTS_E,
    #[doc = "Selects the outputs selected by IOSEL."]
    SELECTS_THE_OUTPUTS,
}
impl OUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OUTSELW::SELECTS_THE_INPUTS_E => false,
            OUTSELW::SELECTS_THE_OUTPUTS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects the inputs elected by IOSEL."]
    #[inline(always)]
    pub fn selects_the_inputs_e(self) -> &'a mut W {
        self.variant(OUTSELW::SELECTS_THE_INPUTS_E)
    }
    #[doc = "Selects the outputs selected by IOSEL."]
    #[inline(always)]
    pub fn selects_the_outputs(self) -> &'a mut W {
        self.variant(OUTSELW::SELECTS_THE_OUTPUTS)
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
#[doc = r"Reader of the field"]
pub type IOSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _IOSELW<'a> {
    w: &'a mut W,
}
impl<'a> _IOSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
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
impl crate::ToBits<u8> for IOCONDR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IOCONDR::LOW => 0,
            IOCONDR::RISE => 1,
            IOCONDR::FALL => 2,
            IOCONDR::HIGH => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IOCOND_R = crate::FR<u8, IOCONDR>;
impl IOCOND_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IOCONDR::LOW
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == IOCONDR::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == IOCONDR::FALL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IOCONDR::HIGH
    }
}
#[doc = "Values that can be written to the field `IOCOND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOCONDW::LOW => 0,
            IOCONDW::RISE => 1,
            IOCONDW::FALL => 2,
            IOCONDW::HIGH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IOCONDW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCONDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCONDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IOCONDW::LOW)
    }
    #[doc = "Rise"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(IOCONDW::RISE)
    }
    #[doc = "Fall"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(IOCONDW::FALL)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IOCONDW::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
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
impl crate::ToBits<u8> for COMBMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            COMBMODER::OR => 0,
            COMBMODER::MATCH => 1,
            COMBMODER::IO => 2,
            COMBMODER::AND => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type COMBMODE_R = crate::FR<u8, COMBMODER>;
impl COMBMODE_R {
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == COMBMODER::OR
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == COMBMODER::MATCH
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == COMBMODER::IO
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == COMBMODER::AND
    }
}
#[doc = "Values that can be written to the field `COMBMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMBMODEW::OR => 0,
            COMBMODEW::MATCH => 1,
            COMBMODEW::IO => 2,
            COMBMODEW::AND => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _COMBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMBMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMBMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    #[inline(always)]
    pub fn or(self) -> &'a mut W {
        self.variant(COMBMODEW::OR)
    }
    #[doc = "MATCH. Uses the specified match only."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(COMBMODEW::MATCH)
    }
    #[doc = "IO. Uses the specified I/O condition only."]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(COMBMODEW::IO)
    }
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    #[inline(always)]
    pub fn and(self) -> &'a mut W {
        self.variant(COMBMODEW::AND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
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
impl crate::ToBits<bool> for STATELDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            STATELDR::STATEV_VALUE_IS_ADDE => false,
            STATELDR::STATEV_VALUE_IS_LOAD => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type STATELD_R = crate::FR<bool, STATELDR>;
impl STATELD_R {
    #[doc = "Checks if the value of the field is `STATEV_VALUE_IS_ADDE`"]
    #[inline(always)]
    pub fn is_statev_value_is_adde(&self) -> bool {
        *self == STATELDR::STATEV_VALUE_IS_ADDE
    }
    #[doc = "Checks if the value of the field is `STATEV_VALUE_IS_LOAD`"]
    #[inline(always)]
    pub fn is_statev_value_is_load(&self) -> bool {
        *self == STATELDR::STATEV_VALUE_IS_LOAD
    }
}
#[doc = "Values that can be written to the field `STATELD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATELDW {
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    STATEV_VALUE_IS_ADDE,
    #[doc = "STATEV value is loaded into STATE."]
    STATEV_VALUE_IS_LOAD,
}
impl STATELDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            STATELDW::STATEV_VALUE_IS_ADDE => false,
            STATELDW::STATEV_VALUE_IS_LOAD => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _STATELDW<'a> {
    w: &'a mut W,
}
impl<'a> _STATELDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATELDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    #[inline(always)]
    pub fn statev_value_is_adde(self) -> &'a mut W {
        self.variant(STATELDW::STATEV_VALUE_IS_ADDE)
    }
    #[doc = "STATEV value is loaded into STATE."]
    #[inline(always)]
    pub fn statev_value_is_load(self) -> &'a mut W {
        self.variant(STATELDW::STATEV_VALUE_IS_LOAD)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type STATEV_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _STATEVW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MATCHMEM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MATCHMEMW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHMEMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
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
}
impl crate::ToBits<u8> for DIRECTIONR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DIRECTIONR::DIRECTION_INDEPENDEN => 0,
            DIRECTIONR::COUNTING_UP => 1,
            DIRECTIONR::COUNTING_DOWN => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIRECTION_R = crate::FR<u8, DIRECTIONR>;
impl DIRECTION_R {
    #[doc = "Checks if the value of the field is `DIRECTION_INDEPENDEN`"]
    #[inline(always)]
    pub fn is_direction_independen(&self) -> bool {
        *self == DIRECTIONR::DIRECTION_INDEPENDEN
    }
    #[doc = "Checks if the value of the field is `COUNTING_UP`"]
    #[inline(always)]
    pub fn is_counting_up(&self) -> bool {
        *self == DIRECTIONR::COUNTING_UP
    }
    #[doc = "Checks if the value of the field is `COUNTING_DOWN`"]
    #[inline(always)]
    pub fn is_counting_down(&self) -> bool {
        *self == DIRECTIONR::COUNTING_DOWN
    }
}
#[doc = "Values that can be written to the field `DIRECTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIRECTIONW::DIRECTION_INDEPENDEN => 0,
            DIRECTIONW::COUNTING_UP => 1,
            DIRECTIONW::COUNTING_DOWN => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIRECTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTIONW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECTIONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    #[inline(always)]
    pub fn direction_independen(self) -> &'a mut W {
        self.variant(DIRECTIONW::DIRECTION_INDEPENDEN)
    }
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_up(self) -> &'a mut W {
        self.variant(DIRECTIONW::COUNTING_UP)
    }
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_down(self) -> &'a mut W {
        self.variant(DIRECTIONW::COUNTING_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub fn matchsel(&self) -> MATCHSEL_R {
        MATCHSEL_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub fn hevent(&self) -> HEVENT_R {
        HEVENT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number (0 to 3) associated with this event (if any). Do not select an input in this register, if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub fn iosel(&self) -> IOSEL_R {
        IOSEL_R::new(((self.bits() >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub fn iocond(&self) -> IOCOND_R {
        IOCOND_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub fn combmode(&self) -> COMBMODE_R {
        COMBMODE_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub fn stateld(&self) -> STATELD_R {
        STATELD_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub fn statev(&self) -> STATEV_R {
        STATEV_R::new(((self.bits() >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub fn matchmem(&self) -> MATCHMEM_R {
        MATCHMEM_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits() >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub fn matchsel(&mut self) -> _MATCHSELW {
        _MATCHSELW { w: self }
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub fn hevent(&mut self) -> _HEVENTW {
        _HEVENTW { w: self }
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline(always)]
    pub fn outsel(&mut self) -> _OUTSELW {
        _OUTSELW { w: self }
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number (0 to 3) associated with this event (if any). Do not select an input in this register, if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub fn iosel(&mut self) -> _IOSELW {
        _IOSELW { w: self }
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub fn iocond(&mut self) -> _IOCONDW {
        _IOCONDW { w: self }
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub fn combmode(&mut self) -> _COMBMODEW {
        _COMBMODEW { w: self }
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub fn stateld(&mut self) -> _STATELDW {
        _STATELDW { w: self }
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub fn statev(&mut self) -> _STATEVW {
        _STATEVW { w: self }
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub fn matchmem(&mut self) -> _MATCHMEMW {
        _MATCHMEMW { w: self }
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn direction(&mut self) -> _DIRECTIONW {
        _DIRECTIONW { w: self }
    }
}
