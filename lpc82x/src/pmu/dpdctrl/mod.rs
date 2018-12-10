#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPDCTRL {
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
#[doc = "Possible values of the field `WAKEUPHYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYSR {
    #[doc = "Disabled. Hysteresis for WAKEUP pin disabled."]
    DISABLED,
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    ENABLED,
}
impl WAKEUPHYSR {
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
            WAKEUPHYSR::DISABLED => false,
            WAKEUPHYSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUPHYSR {
        match value {
            false => WAKEUPHYSR::DISABLED,
            true => WAKEUPHYSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPHYSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPHYSR::ENABLED
    }
}
#[doc = "Possible values of the field `WAKEPAD_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEPAD_DISABLER {
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    ENABLED,
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    DISABLED,
}
impl WAKEPAD_DISABLER {
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
            WAKEPAD_DISABLER::ENABLED => false,
            WAKEPAD_DISABLER::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEPAD_DISABLER {
        match value {
            false => WAKEPAD_DISABLER::ENABLED,
            true => WAKEPAD_DISABLER::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEPAD_DISABLER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEPAD_DISABLER::DISABLED
    }
}
#[doc = "Possible values of the field `LPOSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCENR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl LPOSCENR {
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
            LPOSCENR::DISABLED => false,
            LPOSCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPOSCENR {
        match value {
            false => LPOSCENR::DISABLED,
            true => LPOSCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LPOSCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LPOSCENR::ENABLED
    }
}
#[doc = "Possible values of the field `LPOSCDPDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCDPDENR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl LPOSCDPDENR {
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
            LPOSCDPDENR::DISABLED => false,
            LPOSCDPDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPOSCDPDENR {
        match value {
            false => LPOSCDPDENR::DISABLED,
            true => LPOSCDPDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LPOSCDPDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LPOSCDPDENR::ENABLED
    }
}
#[doc = "Possible values of the field `WAKEUPCLKHYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPCLKHYSR {
    #[doc = "Disabled. Hysteresis for WAKEUP clock pin disabled."]
    DISABLED,
    #[doc = "Enabled. Hysteresis for WAKEUP clock pin enabled."]
    ENABLED,
}
impl WAKEUPCLKHYSR {
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
            WAKEUPCLKHYSR::DISABLED => false,
            WAKEUPCLKHYSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUPCLKHYSR {
        match value {
            false => WAKEUPCLKHYSR::DISABLED,
            true => WAKEUPCLKHYSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPCLKHYSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPCLKHYSR::ENABLED
    }
}
#[doc = "Possible values of the field `WAKECLKPAD_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKECLKPAD_DISABLER {
    #[doc = "Disabled. Setting this bit disables external clock input on pin PIO0_28."]
    DISABLED,
    #[doc = "Enabled. The external clock input for the self wake-up timer is enabled on pin PIO0_28."]
    ENABLED,
}
impl WAKECLKPAD_DISABLER {
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
            WAKECLKPAD_DISABLER::DISABLED => false,
            WAKECLKPAD_DISABLER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKECLKPAD_DISABLER {
        match value {
            false => WAKECLKPAD_DISABLER::DISABLED,
            true => WAKECLKPAD_DISABLER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAKECLKPAD_DISABLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WAKECLKPAD_DISABLER::ENABLED
    }
}
#[doc = "Values that can be written to the field `WAKEUPHYS`"]
pub enum WAKEUPHYSW {
    #[doc = "Disabled. Hysteresis for WAKEUP pin disabled."]
    DISABLED,
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    ENABLED,
}
impl WAKEUPHYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUPHYSW::DISABLED => false,
            WAKEUPHYSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPHYSW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPHYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUPHYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Hysteresis for WAKEUP pin disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::DISABLED)
    }
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::ENABLED)
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
#[doc = "Values that can be written to the field `WAKEPAD_DISABLE`"]
pub enum WAKEPAD_DISABLEW {
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    ENABLED,
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    DISABLED,
}
impl WAKEPAD_DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEPAD_DISABLEW::ENABLED => false,
            WAKEPAD_DISABLEW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEPAD_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEPAD_DISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEPAD_DISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLEW::ENABLED)
    }
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLEW::DISABLED)
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
#[doc = "Values that can be written to the field `LPOSCEN`"]
pub enum LPOSCENW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl LPOSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPOSCENW::DISABLED => false,
            LPOSCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPOSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPOSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPOSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPOSCENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPOSCENW::ENABLED)
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
#[doc = "Values that can be written to the field `LPOSCDPDEN`"]
pub enum LPOSCDPDENW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl LPOSCDPDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPOSCDPDENW::DISABLED => false,
            LPOSCDPDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPOSCDPDENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPOSCDPDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPOSCDPDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPOSCDPDENW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPOSCDPDENW::ENABLED)
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
#[doc = "Values that can be written to the field `WAKEUPCLKHYS`"]
pub enum WAKEUPCLKHYSW {
    #[doc = "Disabled. Hysteresis for WAKEUP clock pin disabled."]
    DISABLED,
    #[doc = "Enabled. Hysteresis for WAKEUP clock pin enabled."]
    ENABLED,
}
impl WAKEUPCLKHYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUPCLKHYSW::DISABLED => false,
            WAKEUPCLKHYSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPCLKHYSW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPCLKHYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUPCLKHYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Hysteresis for WAKEUP clock pin disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPCLKHYSW::DISABLED)
    }
    #[doc = "Enabled. Hysteresis for WAKEUP clock pin enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPCLKHYSW::ENABLED)
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
#[doc = "Values that can be written to the field `WAKECLKPAD_DISABLE`"]
pub enum WAKECLKPAD_DISABLEW {
    #[doc = "Disabled. Setting this bit disables external clock input on pin PIO0_28."]
    DISABLED,
    #[doc = "Enabled. The external clock input for the self wake-up timer is enabled on pin PIO0_28."]
    ENABLED,
}
impl WAKECLKPAD_DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKECLKPAD_DISABLEW::DISABLED => false,
            WAKECLKPAD_DISABLEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKECLKPAD_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKECLKPAD_DISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKECLKPAD_DISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Setting this bit disables external clock input on pin PIO0_28."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKECLKPAD_DISABLEW::DISABLED)
    }
    #[doc = "Enabled. The external clock input for the self wake-up timer is enabled on pin PIO0_28."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKECLKPAD_DISABLEW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline]
    pub fn wakeuphys(&self) -> WAKEUPHYSR {
        WAKEUPHYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline]
    pub fn wakepad_disable(&self) -> WAKEPAD_DISABLER {
        WAKEPAD_DISABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
    #[inline]
    pub fn lposcen(&self) -> LPOSCENR {
        LPOSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable the low-power oscillator in Deep power-down mode. Setting this bit causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
    #[inline]
    pub fn lposcdpden(&self) -> LPOSCDPDENR {
        LPOSCDPDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - External clock input for the self wake-up timer WKTCLKIN hysteresis enable."]
    #[inline]
    pub fn wakeupclkhys(&self) -> WAKEUPCLKHYSR {
        WAKEUPCLKHYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Disable the external clock input for the self wake-up timer. Setting this bit enables the self wake-up timer clock pin WKTCLKLIN. To minimize power consumption, especially in deep power-down mode, disable this clock input when not using the external clock option for the self wake-up timer."]
    #[inline]
    pub fn wakeclkpad_disable(&self) -> WAKECLKPAD_DISABLER {
        WAKECLKPAD_DISABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline]
    pub fn wakeuphys(&mut self) -> _WAKEUPHYSW {
        _WAKEUPHYSW { w: self }
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline]
    pub fn wakepad_disable(&mut self) -> _WAKEPAD_DISABLEW {
        _WAKEPAD_DISABLEW { w: self }
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
    #[inline]
    pub fn lposcen(&mut self) -> _LPOSCENW {
        _LPOSCENW { w: self }
    }
    #[doc = "Bit 3 - Enable the low-power oscillator in Deep power-down mode. Setting this bit causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
    #[inline]
    pub fn lposcdpden(&mut self) -> _LPOSCDPDENW {
        _LPOSCDPDENW { w: self }
    }
    #[doc = "Bit 4 - External clock input for the self wake-up timer WKTCLKIN hysteresis enable."]
    #[inline]
    pub fn wakeupclkhys(&mut self) -> _WAKEUPCLKHYSW {
        _WAKEUPCLKHYSW { w: self }
    }
    #[doc = "Bit 5 - Disable the external clock input for the self wake-up timer. Setting this bit enables the self wake-up timer clock pin WKTCLKLIN. To minimize power consumption, especially in deep power-down mode, disable this clock input when not using the external clock option for the self wake-up timer."]
    #[inline]
    pub fn wakeclkpad_disable(&mut self) -> _WAKECLKPAD_DISABLEW {
        _WAKECLKPAD_DISABLEW { w: self }
    }
}
