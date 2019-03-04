#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XFERCFG {
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
#[doc = "Possible values of the field `CFGVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGVALIDR {
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    NOT_VALID,
    #[doc = "Valid. The current channel descriptor is considered valid."]
    VALID,
}
impl CFGVALIDR {
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
            CFGVALIDR::NOT_VALID => false,
            CFGVALIDR::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFGVALIDR {
        match value {
            false => CFGVALIDR::NOT_VALID,
            true => CFGVALIDR::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == CFGVALIDR::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == CFGVALIDR::VALID
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
impl RELOADR {
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
            RELOADR::DISABLED => false,
            RELOADR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RELOADR {
        match value {
            false => RELOADR::DISABLED,
            true => RELOADR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RELOADR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RELOADR::ENABLED
    }
}
#[doc = "Possible values of the field `SWTRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIGR {
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    NOT_SET,
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    SET,
}
impl SWTRIGR {
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
            SWTRIGR::NOT_SET => false,
            SWTRIGR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWTRIGR {
        match value {
            false => SWTRIGR::NOT_SET,
            true => SWTRIGR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SET`"]
    #[inline]
    pub fn is_not_set(&self) -> bool {
        *self == SWTRIGR::NOT_SET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == SWTRIGR::SET
    }
}
#[doc = "Possible values of the field `CLRTRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRTRIGR {
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NOT_CLEARED,
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    CLEARED,
}
impl CLRTRIGR {
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
            CLRTRIGR::NOT_CLEARED => false,
            CLRTRIGR::CLEARED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLRTRIGR {
        match value {
            false => CLRTRIGR::NOT_CLEARED,
            true => CLRTRIGR::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CLEARED`"]
    #[inline]
    pub fn is_not_cleared(&self) -> bool {
        *self == CLRTRIGR::NOT_CLEARED
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline]
    pub fn is_cleared(&self) -> bool {
        *self == CLRTRIGR::CLEARED
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
impl SETINTAR {
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
            SETINTAR::NO_EFFECT => false,
            SETINTAR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SETINTAR {
        match value {
            false => SETINTAR::NO_EFFECT,
            true => SETINTAR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == SETINTAR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == SETINTAR::SET
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
impl SETINTBR {
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
            SETINTBR::NO_EFFECT => false,
            SETINTBR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SETINTBR {
        match value {
            false => SETINTBR::NO_EFFECT,
            true => SETINTBR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == SETINTBR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == SETINTBR::SET
    }
}
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHR {
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    BIT_8,
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    BIT_16,
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    BIT_32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WIDTHR::BIT_8 => 0,
            WIDTHR::BIT_16 => 1,
            WIDTHR::BIT_32 => 2,
            WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WIDTHR {
        match value {
            0 => WIDTHR::BIT_8,
            1 => WIDTHR::BIT_16,
            2 => WIDTHR::BIT_32,
            i => WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline]
    pub fn is_bit_8(&self) -> bool {
        *self == WIDTHR::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_16`"]
    #[inline]
    pub fn is_bit_16(&self) -> bool {
        *self == WIDTHR::BIT_16
    }
    #[doc = "Checks if the value of the field is `BIT_32`"]
    #[inline]
    pub fn is_bit_32(&self) -> bool {
        *self == WIDTHR::BIT_32
    }
}
#[doc = "Possible values of the field `SRCINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCINCR {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WIDTH_X_1,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4,
}
impl SRCINCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCINCR::NO_INCREMENT => 0,
            SRCINCR::WIDTH_X_1 => 1,
            SRCINCR::WIDTH_X_2 => 2,
            SRCINCR::WIDTH_X_4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCINCR {
        match value {
            0 => SRCINCR::NO_INCREMENT,
            1 => SRCINCR::WIDTH_X_1,
            2 => SRCINCR::WIDTH_X_2,
            3 => SRCINCR::WIDTH_X_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_INCREMENT`"]
    #[inline]
    pub fn is_no_increment(&self) -> bool {
        *self == SRCINCR::NO_INCREMENT
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_1`"]
    #[inline]
    pub fn is_width_x_1(&self) -> bool {
        *self == SRCINCR::WIDTH_X_1
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_2`"]
    #[inline]
    pub fn is_width_x_2(&self) -> bool {
        *self == SRCINCR::WIDTH_X_2
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_4`"]
    #[inline]
    pub fn is_width_x_4(&self) -> bool {
        *self == SRCINCR::WIDTH_X_4
    }
}
#[doc = "Possible values of the field `DSTINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTINCR {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WIDTH_X_1,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4,
}
impl DSTINCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSTINCR::NO_INCREMENT => 0,
            DSTINCR::WIDTH_X_1 => 1,
            DSTINCR::WIDTH_X_2 => 2,
            DSTINCR::WIDTH_X_4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSTINCR {
        match value {
            0 => DSTINCR::NO_INCREMENT,
            1 => DSTINCR::WIDTH_X_1,
            2 => DSTINCR::WIDTH_X_2,
            3 => DSTINCR::WIDTH_X_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_INCREMENT`"]
    #[inline]
    pub fn is_no_increment(&self) -> bool {
        *self == DSTINCR::NO_INCREMENT
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_1`"]
    #[inline]
    pub fn is_width_x_1(&self) -> bool {
        *self == DSTINCR::WIDTH_X_1
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_2`"]
    #[inline]
    pub fn is_width_x_2(&self) -> bool {
        *self == DSTINCR::WIDTH_X_2
    }
    #[doc = "Checks if the value of the field is `WIDTH_X_4`"]
    #[inline]
    pub fn is_width_x_4(&self) -> bool {
        *self == DSTINCR::WIDTH_X_4
    }
}
#[doc = r" Value of the field"]
pub struct XFERCOUNTR {
    bits: u16,
}
impl XFERCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CFGVALID`"]
pub enum CFGVALIDW {
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    NOT_VALID,
    #[doc = "Valid. The current channel descriptor is considered valid."]
    VALID,
}
impl CFGVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFGVALIDW::NOT_VALID => false,
            CFGVALIDW::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _CFGVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(CFGVALIDW::NOT_VALID)
    }
    #[doc = "Valid. The current channel descriptor is considered valid."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(CFGVALIDW::VALID)
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
#[doc = "Values that can be written to the field `RELOAD`"]
pub enum RELOADW {
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    DISABLED,
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    ENABLED,
}
impl RELOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RELOADW::DISABLED => false,
            RELOADW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RELOADW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RELOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RELOADW::DISABLED)
    }
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RELOADW::ENABLED)
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
#[doc = "Values that can be written to the field `SWTRIG`"]
pub enum SWTRIGW {
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    NOT_SET,
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    SET,
}
impl SWTRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWTRIGW::NOT_SET => false,
            SWTRIGW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWTRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    #[inline]
    pub fn not_set(self) -> &'a mut W {
        self.variant(SWTRIGW::NOT_SET)
    }
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SWTRIGW::SET)
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
#[doc = "Values that can be written to the field `CLRTRIG`"]
pub enum CLRTRIGW {
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NOT_CLEARED,
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    CLEARED,
}
impl CLRTRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRTRIGW::NOT_CLEARED => false,
            CLRTRIGW::CLEARED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRTRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRTRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    #[inline]
    pub fn not_cleared(self) -> &'a mut W {
        self.variant(CLRTRIGW::NOT_CLEARED)
    }
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    #[inline]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLRTRIGW::CLEARED)
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
#[doc = "Values that can be written to the field `SETINTA`"]
pub enum SETINTAW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    SET,
}
impl SETINTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SETINTAW::NO_EFFECT => false,
            SETINTAW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETINTAW<'a> {
    w: &'a mut W,
}
impl<'a> _SETINTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETINTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETINTAW::NO_EFFECT)
    }
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SETINTAW::SET)
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
#[doc = "Values that can be written to the field `SETINTB`"]
pub enum SETINTBW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    SET,
}
impl SETINTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SETINTBW::NO_EFFECT => false,
            SETINTBW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETINTBW<'a> {
    w: &'a mut W,
}
impl<'a> _SETINTBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETINTBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETINTBW::NO_EFFECT)
    }
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SETINTBW::SET)
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
#[doc = "Values that can be written to the field `WIDTH`"]
pub enum WIDTHW {
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    BIT_8,
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    BIT_16,
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    BIT_32,
}
impl WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WIDTHW::BIT_8 => 0,
            WIDTHW::BIT_16 => 1,
            WIDTHW::BIT_32 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    #[inline]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(WIDTHW::BIT_8)
    }
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    #[inline]
    pub fn bit_16(self) -> &'a mut W {
        self.variant(WIDTHW::BIT_16)
    }
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    #[inline]
    pub fn bit_32(self) -> &'a mut W {
        self.variant(WIDTHW::BIT_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRCINC`"]
pub enum SRCINCW {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WIDTH_X_1,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4,
}
impl SRCINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCINCW::NO_INCREMENT => 0,
            SRCINCW::WIDTH_X_1 => 1,
            SRCINCW::WIDTH_X_2 => 2,
            SRCINCW::WIDTH_X_4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCINCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCINCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    #[inline]
    pub fn no_increment(self) -> &'a mut W {
        self.variant(SRCINCW::NO_INCREMENT)
    }
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    #[inline]
    pub fn width_x_1(self) -> &'a mut W {
        self.variant(SRCINCW::WIDTH_X_1)
    }
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline]
    pub fn width_x_2(self) -> &'a mut W {
        self.variant(SRCINCW::WIDTH_X_2)
    }
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline]
    pub fn width_x_4(self) -> &'a mut W {
        self.variant(SRCINCW::WIDTH_X_4)
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
#[doc = "Values that can be written to the field `DSTINC`"]
pub enum DSTINCW {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WIDTH_X_1,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4,
}
impl DSTINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSTINCW::NO_INCREMENT => 0,
            DSTINCW::WIDTH_X_1 => 1,
            DSTINCW::WIDTH_X_2 => 2,
            DSTINCW::WIDTH_X_4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSTINCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSTINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSTINCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    #[inline]
    pub fn no_increment(self) -> &'a mut W {
        self.variant(DSTINCW::NO_INCREMENT)
    }
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    #[inline]
    pub fn width_x_1(self) -> &'a mut W {
        self.variant(DSTINCW::WIDTH_X_1)
    }
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline]
    pub fn width_x_2(self) -> &'a mut W {
        self.variant(DSTINCW::WIDTH_X_2)
    }
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline]
    pub fn width_x_4(self) -> &'a mut W {
        self.variant(DSTINCW::WIDTH_X_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XFERCOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _XFERCOUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline]
    pub fn cfgvalid(&self) -> CFGVALIDR {
        CFGVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline]
    pub fn reload(&self) -> RELOADR {
        RELOADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline]
    pub fn swtrig(&self) -> SWTRIGR {
        SWTRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline]
    pub fn clrtrig(&self) -> CLRTRIGR {
        CLRTRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline]
    pub fn setinta(&self) -> SETINTAR {
        SETINTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline]
    pub fn setintb(&self) -> SETINTBR {
        SETINTBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline]
    pub fn width(&self) -> WIDTHR {
        WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline]
    pub fn srcinc(&self) -> SRCINCR {
        SRCINCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline]
    pub fn dstinc(&self) -> DSTINCR {
        DSTINCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline]
    pub fn xfercount(&self) -> XFERCOUNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XFERCOUNTR { bits }
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
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline]
    pub fn cfgvalid(&mut self) -> _CFGVALIDW {
        _CFGVALIDW { w: self }
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline]
    pub fn reload(&mut self) -> _RELOADW {
        _RELOADW { w: self }
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline]
    pub fn swtrig(&mut self) -> _SWTRIGW {
        _SWTRIGW { w: self }
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline]
    pub fn clrtrig(&mut self) -> _CLRTRIGW {
        _CLRTRIGW { w: self }
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline]
    pub fn setinta(&mut self) -> _SETINTAW {
        _SETINTAW { w: self }
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline]
    pub fn setintb(&mut self) -> _SETINTBW {
        _SETINTBW { w: self }
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline]
    pub fn srcinc(&mut self) -> _SRCINCW {
        _SRCINCW { w: self }
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline]
    pub fn dstinc(&mut self) -> _DSTINCW {
        _DSTINCW { w: self }
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline]
    pub fn xfercount(&mut self) -> _XFERCOUNTW {
        _XFERCOUNTW { w: self }
    }
}
