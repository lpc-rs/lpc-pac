#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XFERCFG {
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
#[doc = "Possible values of the field `CFGVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGVALIDR {
    #[doc = "Not valid. The current channel descriptor is not considered valid."]
    NOT_VALID,
    #[doc = "Valid. The current channel descriptor is considered valid."]
    VALID,
}
impl crate::ToBits<bool> for CFGVALIDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CFGVALIDR::NOT_VALID => false,
            CFGVALIDR::VALID => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFGVALID_R = crate::FR<bool, CFGVALIDR>;
impl CFGVALID_R {
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == CFGVALIDR::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == CFGVALIDR::VALID
    }
}
#[doc = "Values that can be written to the field `CFGVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGVALIDW {
    #[doc = "Not valid. The current channel descriptor is not considered valid."]
    NOT_VALID,
    #[doc = "Valid. The current channel descriptor is considered valid."]
    VALID,
}
impl CFGVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGVALIDW::NOT_VALID => false,
            CFGVALIDW::VALID => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFGVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _CFGVALIDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid. The current channel descriptor is not considered valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(CFGVALIDW::NOT_VALID)
    }
    #[doc = "Valid. The current channel descriptor is considered valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(CFGVALIDW::VALID)
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
#[doc = "Possible values of the field `RELOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOADR {
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    DISABLED,
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    ENABLED,
}
impl crate::ToBits<bool> for RELOADR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RELOADR::DISABLED => false,
            RELOADR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RELOAD_R = crate::FR<bool, RELOADR>;
impl RELOAD_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RELOADR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RELOADR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RELOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOADW {
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    DISABLED,
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    ENABLED,
}
impl RELOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RELOADW::DISABLED => false,
            RELOADW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RELOADW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOADW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RELOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RELOADW::DISABLED)
    }
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RELOADW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `SWTRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIGR {
    #[doc = "When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_NOT_SET,
    #[doc = "When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_SET,
}
impl crate::ToBits<bool> for SWTRIGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SWTRIGR::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_NOT_SET => false,
            SWTRIGR::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SWTRIG_R = crate::FR<bool, SWTRIGR>;
impl SWTRIG_R {
    #[doc = "Checks if the value of the field is `WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_NOT_SET`"]
    #[inline(always)]
    pub fn is_when_written_by_software_trigger_not_set(&self) -> bool {
        *self == SWTRIGR::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_SET`"]
    #[inline(always)]
    pub fn is_when_written_by_software_trigger_set(&self) -> bool {
        *self == SWTRIGR::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_SET
    }
}
#[doc = "Values that can be written to the field `SWTRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIGW {
    #[doc = "When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_NOT_SET,
    #[doc = "When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_SET,
}
impl SWTRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SWTRIGW::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_NOT_SET => false,
            SWTRIGW::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_SET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SWTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    #[inline(always)]
    pub fn when_written_by_software_trigger_not_set(self) -> &'a mut W {
        self.variant(SWTRIGW::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_NOT_SET)
    }
    #[doc = "When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    #[inline(always)]
    pub fn when_written_by_software_trigger_set(self) -> &'a mut W {
        self.variant(SWTRIGW::WHEN_WRITTEN_BY_SOFTWARE_TRIGGER_SET)
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
#[doc = "Possible values of the field `CLRTRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRTRIGR {
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NOT_CLEARED,
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted."]
    CLEARED,
}
impl crate::ToBits<bool> for CLRTRIGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CLRTRIGR::NOT_CLEARED => false,
            CLRTRIGR::CLEARED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLRTRIG_R = crate::FR<bool, CLRTRIGR>;
impl CLRTRIG_R {
    #[doc = "Checks if the value of the field is `NOT_CLEARED`"]
    #[inline(always)]
    pub fn is_not_cleared(&self) -> bool {
        *self == CLRTRIGR::NOT_CLEARED
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLRTRIGR::CLEARED
    }
}
#[doc = "Values that can be written to the field `CLRTRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRTRIGW {
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NOT_CLEARED,
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted."]
    CLEARED,
}
impl CLRTRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRTRIGW::NOT_CLEARED => false,
            CLRTRIGW::CLEARED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLRTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRTRIGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRTRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    #[inline(always)]
    pub fn not_cleared(self) -> &'a mut W {
        self.variant(CLRTRIGW::NOT_CLEARED)
    }
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLRTRIGW::CLEARED)
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
#[doc = "Possible values of the field `SETINTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETINTAR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    SET,
}
impl crate::ToBits<bool> for SETINTAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SETINTAR::NO_EFFECT => false,
            SETINTAR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETINTA_R = crate::FR<bool, SETINTAR>;
impl SETINTA_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SETINTAR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SETINTAR::SET
    }
}
#[doc = "Values that can be written to the field `SETINTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETINTAW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    SET,
}
impl SETINTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SETINTAW::NO_EFFECT => false,
            SETINTAW::SET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETINTAW<'a> {
    w: &'a mut W,
}
impl<'a> _SETINTAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETINTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETINTAW::NO_EFFECT)
    }
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SETINTAW::SET)
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
#[doc = "Possible values of the field `SETINTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETINTBR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    SET,
}
impl crate::ToBits<bool> for SETINTBR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SETINTBR::NO_EFFECT => false,
            SETINTBR::SET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETINTB_R = crate::FR<bool, SETINTBR>;
impl SETINTB_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SETINTBR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SETINTBR::SET
    }
}
#[doc = "Values that can be written to the field `SETINTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETINTBW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    SET,
}
impl SETINTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SETINTBW::NO_EFFECT => false,
            SETINTBW::SET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETINTBW<'a> {
    w: &'a mut W,
}
impl<'a> _SETINTBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETINTBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETINTBW::NO_EFFECT)
    }
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SETINTBW::SET)
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
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHR {
    #[doc = "8-bit transfers are performed (8-bit source reads and destination writes)."]
    _8_BIT_TRANSFERS,
    #[doc = "16-bit transfers are performed (16-bit source reads and destination writes)."]
    _16_BIT_TRANSFERS,
    #[doc = "32-bit transfers are performed (32-bit source reads and destination writes)."]
    _32_BIT_TRANSFERS,
}
impl crate::ToBits<u8> for WIDTHR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WIDTHR::_8_BIT_TRANSFERS => 0,
            WIDTHR::_16_BIT_TRANSFERS => 1,
            WIDTHR::_32_BIT_TRANSFERS => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WIDTH_R = crate::FR<u8, WIDTHR>;
impl WIDTH_R {
    #[doc = "Checks if the value of the field is `_8_BIT_TRANSFERS`"]
    #[inline(always)]
    pub fn is_8_bit_transfers(&self) -> bool {
        *self == WIDTHR::_8_BIT_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_16_BIT_TRANSFERS`"]
    #[inline(always)]
    pub fn is_16_bit_transfers(&self) -> bool {
        *self == WIDTHR::_16_BIT_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_32_BIT_TRANSFERS`"]
    #[inline(always)]
    pub fn is_32_bit_transfers(&self) -> bool {
        *self == WIDTHR::_32_BIT_TRANSFERS
    }
}
#[doc = "Values that can be written to the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHW {
    #[doc = "8-bit transfers are performed (8-bit source reads and destination writes)."]
    _8_BIT_TRANSFERS,
    #[doc = "16-bit transfers are performed (16-bit source reads and destination writes)."]
    _16_BIT_TRANSFERS,
    #[doc = "32-bit transfers are performed (32-bit source reads and destination writes)."]
    _32_BIT_TRANSFERS,
}
impl WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            WIDTHW::_8_BIT_TRANSFERS => 0,
            WIDTHW::_16_BIT_TRANSFERS => 1,
            WIDTHW::_32_BIT_TRANSFERS => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit transfers are performed (8-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn _8_bit_transfers(self) -> &'a mut W {
        self.variant(WIDTHW::_8_BIT_TRANSFERS)
    }
    #[doc = "16-bit transfers are performed (16-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn _16_bit_transfers(self) -> &'a mut W {
        self.variant(WIDTHW::_16_BIT_TRANSFERS)
    }
    #[doc = "32-bit transfers are performed (32-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn _32_bit_transfers(self) -> &'a mut W {
        self.variant(WIDTHW::_32_BIT_TRANSFERS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `SRCINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCINCR {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    _1_X_WIDTH,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    _2_X_WIDTH,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    _4_X_WIDTH,
}
impl crate::ToBits<u8> for SRCINCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SRCINCR::NO_INCREMENT => 0,
            SRCINCR::_1_X_WIDTH => 1,
            SRCINCR::_2_X_WIDTH => 2,
            SRCINCR::_4_X_WIDTH => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SRCINC_R = crate::FR<u8, SRCINCR>;
impl SRCINC_R {
    #[doc = "Checks if the value of the field is `NO_INCREMENT`"]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == SRCINCR::NO_INCREMENT
    }
    #[doc = "Checks if the value of the field is `_1_X_WIDTH`"]
    #[inline(always)]
    pub fn is_1_x_width(&self) -> bool {
        *self == SRCINCR::_1_X_WIDTH
    }
    #[doc = "Checks if the value of the field is `_2_X_WIDTH`"]
    #[inline(always)]
    pub fn is_2_x_width(&self) -> bool {
        *self == SRCINCR::_2_X_WIDTH
    }
    #[doc = "Checks if the value of the field is `_4_X_WIDTH`"]
    #[inline(always)]
    pub fn is_4_x_width(&self) -> bool {
        *self == SRCINCR::_4_X_WIDTH
    }
}
#[doc = "Values that can be written to the field `SRCINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCINCW {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    _1_X_WIDTH,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    _2_X_WIDTH,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    _4_X_WIDTH,
}
impl SRCINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCINCW::NO_INCREMENT => 0,
            SRCINCW::_1_X_WIDTH => 1,
            SRCINCW::_2_X_WIDTH => 2,
            SRCINCW::_4_X_WIDTH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SRCINCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCINCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCINCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut W {
        self.variant(SRCINCW::NO_INCREMENT)
    }
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    #[inline(always)]
    pub fn _1_x_width(self) -> &'a mut W {
        self.variant(SRCINCW::_1_X_WIDTH)
    }
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn _2_x_width(self) -> &'a mut W {
        self.variant(SRCINCW::_2_X_WIDTH)
    }
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn _4_x_width(self) -> &'a mut W {
        self.variant(SRCINCW::_4_X_WIDTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `DSTINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTINCR {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    _1_X_WIDTH,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    _2_X_WIDTH,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    _4_X_WIDTH,
}
impl crate::ToBits<u8> for DSTINCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DSTINCR::NO_INCREMENT => 0,
            DSTINCR::_1_X_WIDTH => 1,
            DSTINCR::_2_X_WIDTH => 2,
            DSTINCR::_4_X_WIDTH => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DSTINC_R = crate::FR<u8, DSTINCR>;
impl DSTINC_R {
    #[doc = "Checks if the value of the field is `NO_INCREMENT`"]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == DSTINCR::NO_INCREMENT
    }
    #[doc = "Checks if the value of the field is `_1_X_WIDTH`"]
    #[inline(always)]
    pub fn is_1_x_width(&self) -> bool {
        *self == DSTINCR::_1_X_WIDTH
    }
    #[doc = "Checks if the value of the field is `_2_X_WIDTH`"]
    #[inline(always)]
    pub fn is_2_x_width(&self) -> bool {
        *self == DSTINCR::_2_X_WIDTH
    }
    #[doc = "Checks if the value of the field is `_4_X_WIDTH`"]
    #[inline(always)]
    pub fn is_4_x_width(&self) -> bool {
        *self == DSTINCR::_4_X_WIDTH
    }
}
#[doc = "Values that can be written to the field `DSTINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTINCW {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    _1_X_WIDTH,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    _2_X_WIDTH,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    _4_X_WIDTH,
}
impl DSTINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSTINCW::NO_INCREMENT => 0,
            DSTINCW::_1_X_WIDTH => 1,
            DSTINCW::_2_X_WIDTH => 2,
            DSTINCW::_4_X_WIDTH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DSTINCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTINCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTINCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut W {
        self.variant(DSTINCW::NO_INCREMENT)
    }
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    #[inline(always)]
    pub fn _1_x_width(self) -> &'a mut W {
        self.variant(DSTINCW::_1_X_WIDTH)
    }
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn _2_x_width(self) -> &'a mut W {
        self.variant(DSTINCW::_2_X_WIDTH)
    }
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn _4_x_width(self) -> &'a mut W {
        self.variant(DSTINCW::_4_X_WIDTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type XFERCOUNT_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _XFERCOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _XFERCOUNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub fn cfgvalid(&self) -> CFGVALID_R {
        CFGVALID_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline(always)]
    pub fn clrtrig(&self) -> CLRTRIG_R {
        CLRTRIG_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setinta(&self) -> SETINTA_R {
        SETINTA_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setintb(&self) -> SETINTB_R {
        SETINTB_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. ... 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub fn xfercount(&self) -> XFERCOUNT_R {
        XFERCOUNT_R::new(((self.bits() >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub fn cfgvalid(&mut self) -> _CFGVALIDW {
        _CFGVALIDW { w: self }
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub fn reload(&mut self) -> _RELOADW {
        _RELOADW { w: self }
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline(always)]
    pub fn swtrig(&mut self) -> _SWTRIGW {
        _SWTRIGW { w: self }
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline(always)]
    pub fn clrtrig(&mut self) -> _CLRTRIGW {
        _CLRTRIGW { w: self }
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setinta(&mut self) -> _SETINTAW {
        _SETINTAW { w: self }
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setintb(&mut self) -> _SETINTBW {
        _SETINTBW { w: self }
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline(always)]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn srcinc(&mut self) -> _SRCINCW {
        _SRCINCW { w: self }
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn dstinc(&mut self) -> _DSTINCW {
        _DSTINCW { w: self }
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. ... 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub fn xfercount(&mut self) -> _XFERCOUNTW {
        _XFERCOUNTW { w: self }
    }
}
