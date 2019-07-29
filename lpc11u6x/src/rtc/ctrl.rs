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
#[doc = "Possible values of the field `SWRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRESETR {
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    NOT_IN_RESET,
    #[doc = "In reset. The RTC is held in reset.     All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register.   Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared. This bit may also serve as a Power Fail Detect flag for the always-on voltage domain."]
    IN_RESET,
}
impl SWRESETR {
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
            SWRESETR::NOT_IN_RESET => false,
            SWRESETR::IN_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRESETR {
        match value {
            false => SWRESETR::NOT_IN_RESET,
            true => SWRESETR::IN_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IN_RESET`"]
    #[inline]
    pub fn is_not_in_reset(&self) -> bool {
        *self == SWRESETR::NOT_IN_RESET
    }
    #[doc = "Checks if the value of the field is `IN_RESET`"]
    #[inline]
    pub fn is_in_reset(&self) -> bool {
        *self == SWRESETR::IN_RESET
    }
}
#[doc = "Possible values of the field `OFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFDR {
    #[doc = "Run. The RTC oscillator is running properly. Writing a 0 has no effect."]
    RUN,
    #[doc = "Fail. RTC oscillator fail detected. Clear this flag after the following power-up. Writing a 1 clears this bit."]
    FAIL,
}
impl OFDR {
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
            OFDR::RUN => false,
            OFDR::FAIL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFDR {
        match value {
            false => OFDR::RUN,
            true => OFDR::FAIL,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == OFDR::RUN
    }
    #[doc = "Checks if the value of the field is `FAIL`"]
    #[inline]
    pub fn is_fail(&self) -> bool {
        *self == OFDR::FAIL
    }
}
#[doc = "Possible values of the field `ALARM1HZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM1HZR {
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NO_MATCH,
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    MATCH,
}
impl ALARM1HZR {
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
            ALARM1HZR::NO_MATCH => false,
            ALARM1HZR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARM1HZR {
        match value {
            false => ALARM1HZR::NO_MATCH,
            true => ALARM1HZR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline]
    pub fn is_no_match(&self) -> bool {
        *self == ALARM1HZR::NO_MATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == ALARM1HZR::MATCH
    }
}
#[doc = "Possible values of the field `WAKE1KHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE1KHZR {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIME_OUT,
}
impl WAKE1KHZR {
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
            WAKE1KHZR::RUN => false,
            WAKE1KHZR::TIME_OUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKE1KHZR {
        match value {
            false => WAKE1KHZR::RUN,
            true => WAKE1KHZR::TIME_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == WAKE1KHZR::RUN
    }
    #[doc = "Checks if the value of the field is `TIME_OUT`"]
    #[inline]
    pub fn is_time_out(&self) -> bool {
        *self == WAKE1KHZR::TIME_OUT
    }
}
#[doc = "Possible values of the field `ALARMDPD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMDPD_ENR {
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl ALARMDPD_ENR {
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
            ALARMDPD_ENR::DISABLE => false,
            ALARMDPD_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARMDPD_ENR {
        match value {
            false => ALARMDPD_ENR::DISABLE,
            true => ALARMDPD_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ALARMDPD_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ALARMDPD_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `WAKEDPD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEDPD_ENR {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl WAKEDPD_ENR {
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
            WAKEDPD_ENR::DISABLE => false,
            WAKEDPD_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEDPD_ENR {
        match value {
            false => WAKEDPD_ENR::DISABLE,
            true => WAKEDPD_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WAKEDPD_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WAKEDPD_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTC1KHZ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC1KHZ_ENR {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    ENABLE,
}
impl RTC1KHZ_ENR {
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
            RTC1KHZ_ENR::DISABLE => false,
            RTC1KHZ_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC1KHZ_ENR {
        match value {
            false => RTC1KHZ_ENR::DISABLE,
            true => RTC1KHZ_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RTC1KHZ_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTC1KHZ_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ENR {
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    DISABLE,
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. You must set this bit to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    ENABLE,
}
impl RTC_ENR {
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
            RTC_ENR::DISABLE => false,
            RTC_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ENR {
        match value {
            false => RTC_ENR::DISABLE,
            true => RTC_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RTC_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTC_ENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `SWRESET`"]
pub enum SWRESETW {
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    NOT_IN_RESET,
    #[doc = "In reset. The RTC is held in reset.     All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register.   Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared. This bit may also serve as a Power Fail Detect flag for the always-on voltage domain."]
    IN_RESET,
}
impl SWRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRESETW::NOT_IN_RESET => false,
            SWRESETW::IN_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    #[inline]
    pub fn not_in_reset(self) -> &'a mut W {
        self.variant(SWRESETW::NOT_IN_RESET)
    }
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared. This bit may also serve as a Power Fail Detect flag for the always-on voltage domain."]
    #[inline]
    pub fn in_reset(self) -> &'a mut W {
        self.variant(SWRESETW::IN_RESET)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OFD`"]
pub enum OFDW {
    #[doc = "Run. The RTC oscillator is running properly. Writing a 0 has no effect."]
    RUN,
    #[doc = "Fail. RTC oscillator fail detected. Clear this flag after the following power-up. Writing a 1 clears this bit."]
    FAIL,
}
impl OFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFDW::RUN => false,
            OFDW::FAIL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFDW<'a> {
    w: &'a mut W,
}
impl<'a> _OFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Run. The RTC oscillator is running properly. Writing a 0 has no effect."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(OFDW::RUN)
    }
    #[doc = "Fail. RTC oscillator fail detected. Clear this flag after the following power-up. Writing a 1 clears this bit."]
    #[inline]
    pub fn fail(self) -> &'a mut W {
        self.variant(OFDW::FAIL)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALARM1HZ`"]
pub enum ALARM1HZW {
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NO_MATCH,
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    MATCH,
}
impl ALARM1HZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALARM1HZW::NO_MATCH => false,
            ALARM1HZW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARM1HZW<'a> {
    w: &'a mut W,
}
impl<'a> _ALARM1HZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARM1HZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    #[inline]
    pub fn no_match(self) -> &'a mut W {
        self.variant(ALARM1HZW::NO_MATCH)
    }
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(ALARM1HZW::MATCH)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKE1KHZ`"]
pub enum WAKE1KHZW {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIME_OUT,
}
impl WAKE1KHZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKE1KHZW::RUN => false,
            WAKE1KHZW::TIME_OUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKE1KHZW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE1KHZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKE1KHZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(WAKE1KHZW::RUN)
    }
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline]
    pub fn time_out(self) -> &'a mut W {
        self.variant(WAKE1KHZW::TIME_OUT)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALARMDPD_EN`"]
pub enum ALARMDPD_ENW {
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl ALARMDPD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALARMDPD_ENW::DISABLE => false,
            ALARMDPD_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARMDPD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALARMDPD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARMDPD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALARMDPD_ENW::DISABLE)
    }
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALARMDPD_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `WAKEDPD_EN`"]
pub enum WAKEDPD_ENW {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl WAKEDPD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEDPD_ENW::DISABLE => false,
            WAKEDPD_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEDPD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEDPD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEDPD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEDPD_ENW::DISABLE)
    }
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEDPD_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `RTC1KHZ_EN`"]
pub enum RTC1KHZ_ENW {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    ENABLE,
}
impl RTC1KHZ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC1KHZ_ENW::DISABLE => false,
            RTC1KHZ_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC1KHZ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC1KHZ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC1KHZ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC1KHZ_ENW::DISABLE)
    }
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC1KHZ_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `RTC_EN`"]
pub enum RTC_ENW {
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    DISABLE,
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. You must set this bit to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    ENABLE,
}
impl RTC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ENW::DISABLE => false,
            RTC_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_ENW::DISABLE)
    }
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. You must set this bit to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_ENW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software reset control"]
    #[inline]
    pub fn swreset(&self) -> SWRESETR {
        SWRESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Oscillator fail detect status."]
    #[inline]
    pub fn ofd(&self) -> OFDR {
        OFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline]
    pub fn alarm1hz(&self) -> ALARM1HZR {
        ALARM1HZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline]
    pub fn wake1khz(&self) -> WAKE1KHZR {
        WAKE1KHZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline]
    pub fn alarmdpd_en(&self) -> ALARMDPD_ENR {
        ALARMDPD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline]
    pub fn wakedpd_en(&self) -> WAKEDPD_ENR {
        WAKEDPD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline]
    pub fn rtc1khz_en(&self) -> RTC1KHZ_ENR {
        RTC1KHZ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline]
    pub fn rtc_en(&self) -> RTC_ENR {
        RTC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software reset control"]
    #[inline]
    pub fn swreset(&mut self) -> _SWRESETW {
        _SWRESETW { w: self }
    }
    #[doc = "Bit 1 - Oscillator fail detect status."]
    #[inline]
    pub fn ofd(&mut self) -> _OFDW {
        _OFDW { w: self }
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline]
    pub fn alarm1hz(&mut self) -> _ALARM1HZW {
        _ALARM1HZW { w: self }
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline]
    pub fn wake1khz(&mut self) -> _WAKE1KHZW {
        _WAKE1KHZW { w: self }
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline]
    pub fn alarmdpd_en(&mut self) -> _ALARMDPD_ENW {
        _ALARMDPD_ENW { w: self }
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline]
    pub fn wakedpd_en(&mut self) -> _WAKEDPD_ENW {
        _WAKEDPD_ENW { w: self }
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline]
    pub fn rtc1khz_en(&mut self) -> _RTC1KHZ_ENW {
        _RTC1KHZ_ENW { w: self }
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline]
    pub fn rtc_en(&mut self) -> _RTC_ENW {
        _RTC_ENW { w: self }
    }
}
