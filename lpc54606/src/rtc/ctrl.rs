#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Software reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRESET_A {
    #[doc = "0: Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    NOT_IN_RESET = 0,
    #[doc = "1: In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    IN_RESET = 1,
}
impl From<SWRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SWRESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRESET`"]
pub type SWRESET_R = crate::R<bool, SWRESET_A>;
impl SWRESET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRESET_A {
        match self.bits {
            false => SWRESET_A::NOT_IN_RESET,
            true => SWRESET_A::IN_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IN_RESET`"]
    #[inline(always)]
    pub fn is_not_in_reset(&self) -> bool {
        *self == SWRESET_A::NOT_IN_RESET
    }
    #[doc = "Checks if the value of the field is `IN_RESET`"]
    #[inline(always)]
    pub fn is_in_reset(&self) -> bool {
        *self == SWRESET_A::IN_RESET
    }
}
#[doc = "Write proxy for field `SWRESET`"]
pub struct SWRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRESET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    #[inline(always)]
    pub fn not_in_reset(self) -> &'a mut W {
        self.variant(SWRESET_A::NOT_IN_RESET)
    }
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    #[inline(always)]
    pub fn in_reset(self) -> &'a mut W {
        self.variant(SWRESET_A::IN_RESET)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "RTC 1 Hz timer alarm flag status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM1HZ_A {
    #[doc = "0: No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NO_MATCH = 0,
    #[doc = "1: Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    MATCH = 1,
}
impl From<ALARM1HZ_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM1HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALARM1HZ`"]
pub type ALARM1HZ_R = crate::R<bool, ALARM1HZ_A>;
impl ALARM1HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM1HZ_A {
        match self.bits {
            false => ALARM1HZ_A::NO_MATCH,
            true => ALARM1HZ_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == ALARM1HZ_A::NO_MATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == ALARM1HZ_A::MATCH
    }
}
#[doc = "Write proxy for field `ALARM1HZ`"]
pub struct ALARM1HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM1HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM1HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(ALARM1HZ_A::NO_MATCH)
    }
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(ALARM1HZ_A::MATCH)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "RTC 1 kHz timer wake-up flag status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE1KHZ_A {
    #[doc = "0: Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN = 0,
    #[doc = "1: Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIMEOUT = 1,
}
impl From<WAKE1KHZ_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE1KHZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKE1KHZ`"]
pub type WAKE1KHZ_R = crate::R<bool, WAKE1KHZ_A>;
impl WAKE1KHZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE1KHZ_A {
        match self.bits {
            false => WAKE1KHZ_A::RUN,
            true => WAKE1KHZ_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WAKE1KHZ_A::RUN
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == WAKE1KHZ_A::TIMEOUT
    }
}
#[doc = "Write proxy for field `WAKE1KHZ`"]
pub struct WAKE1KHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE1KHZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKE1KHZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(WAKE1KHZ_A::RUN)
    }
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(WAKE1KHZ_A::TIMEOUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "RTC 1 Hz timer alarm enable for Deep power-down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMDPD_EN_A {
    #[doc = "0: Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE = 0,
    #[doc = "1: Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    ENABLE = 1,
}
impl From<ALARMDPD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALARMDPD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALARMDPD_EN`"]
pub type ALARMDPD_EN_R = crate::R<bool, ALARMDPD_EN_A>;
impl ALARMDPD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARMDPD_EN_A {
        match self.bits {
            false => ALARMDPD_EN_A::DISABLE,
            true => ALARMDPD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALARMDPD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALARMDPD_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `ALARMDPD_EN`"]
pub struct ALARMDPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARMDPD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARMDPD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALARMDPD_EN_A::DISABLE)
    }
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALARMDPD_EN_A::ENABLE)
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
#[doc = "RTC 1 kHz timer wake-up enable for Deep power-down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEDPD_EN_A {
    #[doc = "0: Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE = 0,
    #[doc = "1: Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    ENABLE = 1,
}
impl From<WAKEDPD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEDPD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEDPD_EN`"]
pub type WAKEDPD_EN_R = crate::R<bool, WAKEDPD_EN_A>;
impl WAKEDPD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEDPD_EN_A {
        match self.bits {
            false => WAKEDPD_EN_A::DISABLE,
            true => WAKEDPD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAKEDPD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAKEDPD_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `WAKEDPD_EN`"]
pub struct WAKEDPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEDPD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEDPD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEDPD_EN_A::DISABLE)
    }
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEDPD_EN_A::ENABLE)
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
#[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC1KHZ_EN_A {
    #[doc = "0: Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE = 0,
    #[doc = "1: Enable. The 1 kHz RTC timer is enabled."]
    ENABLE = 1,
}
impl From<RTC1KHZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RTC1KHZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC1KHZ_EN`"]
pub type RTC1KHZ_EN_R = crate::R<bool, RTC1KHZ_EN_A>;
impl RTC1KHZ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC1KHZ_EN_A {
        match self.bits {
            false => RTC1KHZ_EN_A::DISABLE,
            true => RTC1KHZ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC1KHZ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC1KHZ_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `RTC1KHZ_EN`"]
pub struct RTC1KHZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC1KHZ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC1KHZ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC1KHZ_EN_A::DISABLE)
    }
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC1KHZ_EN_A::ENABLE)
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
#[doc = "RTC enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_EN_A {
    #[doc = "0: Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    DISABLE = 0,
    #[doc = "1: Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    ENABLE = 1,
}
impl From<RTC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_EN`"]
pub type RTC_EN_R = crate::R<bool, RTC_EN_A>;
impl RTC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_EN_A {
        match self.bits {
            false => RTC_EN_A::DISABLE,
            true => RTC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `RTC_EN`"]
pub struct RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_EN_A::DISABLE)
    }
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "RTC oscillator power-down control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_OSC_PD_A {
    #[doc = "0: See RTC_OSC_BYPASS"]
    POWER_UP = 0,
    #[doc = "1: RTC oscillator is powered-down."]
    POWERED_DOWN = 1,
}
impl From<RTC_OSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_OSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_OSC_PD`"]
pub type RTC_OSC_PD_R = crate::R<bool, RTC_OSC_PD_A>;
impl RTC_OSC_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_OSC_PD_A {
        match self.bits {
            false => RTC_OSC_PD_A::POWER_UP,
            true => RTC_OSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == RTC_OSC_PD_A::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == RTC_OSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `RTC_OSC_PD`"]
pub struct RTC_OSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_OSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_OSC_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "See RTC_OSC_BYPASS"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(RTC_OSC_PD_A::POWER_UP)
    }
    #[doc = "RTC oscillator is powered-down."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(RTC_OSC_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software reset control"]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    pub fn alarm1hz(&self) -> ALARM1HZ_R {
        ALARM1HZ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    pub fn wake1khz(&self) -> WAKE1KHZ_R {
        WAKE1KHZ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    pub fn alarmdpd_en(&self) -> ALARMDPD_EN_R {
        ALARMDPD_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    pub fn wakedpd_en(&self) -> WAKEDPD_EN_R {
        WAKEDPD_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    pub fn rtc1khz_en(&self) -> RTC1KHZ_EN_R {
        RTC1KHZ_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline(always)]
    pub fn rtc_en(&self) -> RTC_EN_R {
        RTC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTC oscillator power-down control."]
    #[inline(always)]
    pub fn rtc_osc_pd(&self) -> RTC_OSC_PD_R {
        RTC_OSC_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset control"]
    #[inline(always)]
    pub fn swreset(&mut self) -> SWRESET_W {
        SWRESET_W { w: self }
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    pub fn alarm1hz(&mut self) -> ALARM1HZ_W {
        ALARM1HZ_W { w: self }
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    pub fn wake1khz(&mut self) -> WAKE1KHZ_W {
        WAKE1KHZ_W { w: self }
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    pub fn alarmdpd_en(&mut self) -> ALARMDPD_EN_W {
        ALARMDPD_EN_W { w: self }
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    pub fn wakedpd_en(&mut self) -> WAKEDPD_EN_W {
        WAKEDPD_EN_W { w: self }
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    pub fn rtc1khz_en(&mut self) -> RTC1KHZ_EN_W {
        RTC1KHZ_EN_W { w: self }
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline(always)]
    pub fn rtc_en(&mut self) -> RTC_EN_W {
        RTC_EN_W { w: self }
    }
    #[doc = "Bit 8 - RTC oscillator power-down control."]
    #[inline(always)]
    pub fn rtc_osc_pd(&mut self) -> RTC_OSC_PD_W {
        RTC_OSC_PD_W { w: self }
    }
}
