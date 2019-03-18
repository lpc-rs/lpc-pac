#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `MSTPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGENR {
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    ENABLED,
}
impl MSTPENDINGENR {
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
            MSTPENDINGENR::DISABLED => false,
            MSTPENDINGENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTPENDINGENR {
        match value {
            false => MSTPENDINGENR::DISABLED,
            true => MSTPENDINGENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MSTPENDINGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MSTPENDINGENR::ENABLED
    }
}
#[doc = "Possible values of the field `MSTARBLOSSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSENR {
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    ENABLED,
}
impl MSTARBLOSSENR {
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
            MSTARBLOSSENR::DISABLED => false,
            MSTARBLOSSENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTARBLOSSENR {
        match value {
            false => MSTARBLOSSENR::DISABLED,
            true => MSTARBLOSSENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MSTARBLOSSENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MSTARBLOSSENR::ENABLED
    }
}
#[doc = "Possible values of the field `MSTSTSTPERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRENR {
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    ENABLED,
}
impl MSTSTSTPERRENR {
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
            MSTSTSTPERRENR::DISABLED => false,
            MSTSTSTPERRENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTSTPERRENR {
        match value {
            false => MSTSTSTPERRENR::DISABLED,
            true => MSTSTSTPERRENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MSTSTSTPERRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MSTSTSTPERRENR::ENABLED
    }
}
#[doc = "Possible values of the field `SLVPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGENR {
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    ENABLED,
}
impl SLVPENDINGENR {
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
            SLVPENDINGENR::DISABLED => false,
            SLVPENDINGENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVPENDINGENR {
        match value {
            false => SLVPENDINGENR::DISABLED,
            true => SLVPENDINGENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLVPENDINGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SLVPENDINGENR::ENABLED
    }
}
#[doc = "Possible values of the field `SLVNOTSTREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRENR {
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    ENABLED,
}
impl SLVNOTSTRENR {
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
            SLVNOTSTRENR::DISABLED => false,
            SLVNOTSTRENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVNOTSTRENR {
        match value {
            false => SLVNOTSTRENR::DISABLED,
            true => SLVNOTSTRENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLVNOTSTRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SLVNOTSTRENR::ENABLED
    }
}
#[doc = "Possible values of the field `SLVDESELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELENR {
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    ENABLED,
}
impl SLVDESELENR {
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
            SLVDESELENR::DISABLED => false,
            SLVDESELENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVDESELENR {
        match value {
            false => SLVDESELENR::DISABLED,
            true => SLVDESELENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDESELENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SLVDESELENR::ENABLED
    }
}
#[doc = "Possible values of the field `MONRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYENR {
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    ENABLED,
}
impl MONRDYENR {
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
            MONRDYENR::DISABLED => false,
            MONRDYENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONRDYENR {
        match value {
            false => MONRDYENR::DISABLED,
            true => MONRDYENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MONRDYENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MONRDYENR::ENABLED
    }
}
#[doc = "Possible values of the field `MONOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVENR {
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    ENABLED,
}
impl MONOVENR {
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
            MONOVENR::DISABLED => false,
            MONOVENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONOVENR {
        match value {
            false => MONOVENR::DISABLED,
            true => MONOVENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MONOVENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MONOVENR::ENABLED
    }
}
#[doc = "Possible values of the field `MONIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEENR {
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    ENABLED,
}
impl MONIDLEENR {
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
            MONIDLEENR::DISABLED => false,
            MONIDLEENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONIDLEENR {
        match value {
            false => MONIDLEENR::DISABLED,
            true => MONIDLEENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MONIDLEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MONIDLEENR::ENABLED
    }
}
#[doc = "Possible values of the field `EVENTTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTENR {
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    ENABLED,
}
impl EVENTTIMEOUTENR {
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
            EVENTTIMEOUTENR::DISABLED => false,
            EVENTTIMEOUTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVENTTIMEOUTENR {
        match value {
            false => EVENTTIMEOUTENR::DISABLED,
            true => EVENTTIMEOUTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EVENTTIMEOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EVENTTIMEOUTENR::ENABLED
    }
}
#[doc = "Possible values of the field `SCLTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTENR {
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    ENABLED,
}
impl SCLTIMEOUTENR {
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
            SCLTIMEOUTENR::DISABLED => false,
            SCLTIMEOUTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLTIMEOUTENR {
        match value {
            false => SCLTIMEOUTENR::DISABLED,
            true => SCLTIMEOUTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SCLTIMEOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SCLTIMEOUTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MSTPENDINGEN`"]
pub enum MSTPENDINGENW {
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    ENABLED,
}
impl MSTPENDINGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTPENDINGENW::DISABLED => false,
            MSTPENDINGENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTPENDINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTPENDINGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTPENDINGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTPENDINGENW::DISABLED)
    }
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTPENDINGENW::ENABLED)
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
#[doc = "Values that can be written to the field `MSTARBLOSSEN`"]
pub enum MSTARBLOSSENW {
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    ENABLED,
}
impl MSTARBLOSSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSENW::DISABLED => false,
            MSTARBLOSSENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTARBLOSSENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTARBLOSSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTARBLOSSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSENW::DISABLED)
    }
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSENW::ENABLED)
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
#[doc = "Values that can be written to the field `MSTSTSTPERREN`"]
pub enum MSTSTSTPERRENW {
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    ENABLED,
}
impl MSTSTSTPERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRENW::DISABLED => false,
            MSTSTSTPERRENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTSTPERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTSTPERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTSTPERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERRENW::DISABLED)
    }
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERRENW::ENABLED)
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
#[doc = "Values that can be written to the field `SLVPENDINGEN`"]
pub enum SLVPENDINGENW {
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    ENABLED,
}
impl SLVPENDINGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVPENDINGENW::DISABLED => false,
            SLVPENDINGENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVPENDINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVPENDINGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVPENDINGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVPENDINGENW::DISABLED)
    }
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVPENDINGENW::ENABLED)
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
#[doc = "Values that can be written to the field `SLVNOTSTREN`"]
pub enum SLVNOTSTRENW {
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    ENABLED,
}
impl SLVNOTSTRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNOTSTRENW::DISABLED => false,
            SLVNOTSTRENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVNOTSTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNOTSTRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVNOTSTRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVNOTSTRENW::DISABLED)
    }
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVNOTSTRENW::ENABLED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVDESELEN`"]
pub enum SLVDESELENW {
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    ENABLED,
}
impl SLVDESELENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDESELENW::DISABLED => false,
            SLVDESELENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVDESELENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDESELENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVDESELENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDESELENW::DISABLED)
    }
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVDESELENW::ENABLED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONRDYEN`"]
pub enum MONRDYENW {
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    ENABLED,
}
impl MONRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONRDYENW::DISABLED => false,
            MONRDYENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONRDYENW::DISABLED)
    }
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONRDYENW::ENABLED)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONOVEN`"]
pub enum MONOVENW {
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    ENABLED,
}
impl MONOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONOVENW::DISABLED => false,
            MONOVENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONOVENW::DISABLED)
    }
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONOVENW::ENABLED)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONIDLEEN`"]
pub enum MONIDLEENW {
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    ENABLED,
}
impl MONIDLEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONIDLEENW::DISABLED => false,
            MONIDLEENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONIDLEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONIDLEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONIDLEENW::DISABLED)
    }
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONIDLEENW::ENABLED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EVENTTIMEOUTEN`"]
pub enum EVENTTIMEOUTENW {
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    ENABLED,
}
impl EVENTTIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTENW::DISABLED => false,
            EVENTTIMEOUTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVENTTIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTTIMEOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVENTTIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTENW::DISABLED)
    }
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTENW::ENABLED)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLTIMEOUTEN`"]
pub enum SCLTIMEOUTENW {
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    ENABLED,
}
impl SCLTIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTENW::DISABLED => false,
            SCLTIMEOUTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLTIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLTIMEOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLTIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTENW::DISABLED)
    }
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTENW::ENABLED)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline]
    pub fn mstpendingen(&self) -> MSTPENDINGENR {
        MSTPENDINGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline]
    pub fn mstarblossen(&self) -> MSTARBLOSSENR {
        MSTARBLOSSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline]
    pub fn mstststperren(&self) -> MSTSTSTPERRENR {
        MSTSTSTPERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline]
    pub fn slvpendingen(&self) -> SLVPENDINGENR {
        SLVPENDINGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline]
    pub fn slvnotstren(&self) -> SLVNOTSTRENR {
        SLVNOTSTRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline]
    pub fn slvdeselen(&self) -> SLVDESELENR {
        SLVDESELENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline]
    pub fn monrdyen(&self) -> MONRDYENR {
        MONRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline]
    pub fn monoven(&self) -> MONOVENR {
        MONOVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline]
    pub fn monidleen(&self) -> MONIDLEENR {
        MONIDLEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline]
    pub fn eventtimeouten(&self) -> EVENTTIMEOUTENR {
        EVENTTIMEOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline]
    pub fn scltimeouten(&self) -> SCLTIMEOUTENR {
        SCLTIMEOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline]
    pub fn mstpendingen(&mut self) -> _MSTPENDINGENW {
        _MSTPENDINGENW { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline]
    pub fn mstarblossen(&mut self) -> _MSTARBLOSSENW {
        _MSTARBLOSSENW { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline]
    pub fn mstststperren(&mut self) -> _MSTSTSTPERRENW {
        _MSTSTSTPERRENW { w: self }
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline]
    pub fn slvpendingen(&mut self) -> _SLVPENDINGENW {
        _SLVPENDINGENW { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline]
    pub fn slvnotstren(&mut self) -> _SLVNOTSTRENW {
        _SLVNOTSTRENW { w: self }
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline]
    pub fn slvdeselen(&mut self) -> _SLVDESELENW {
        _SLVDESELENW { w: self }
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline]
    pub fn monrdyen(&mut self) -> _MONRDYENW {
        _MONRDYENW { w: self }
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline]
    pub fn monoven(&mut self) -> _MONOVENW {
        _MONOVENW { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline]
    pub fn monidleen(&mut self) -> _MONIDLEENW {
        _MONIDLEENW { w: self }
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline]
    pub fn eventtimeouten(&mut self) -> _EVENTTIMEOUTENW {
        _EVENTTIMEOUTENW { w: self }
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline]
    pub fn scltimeouten(&mut self) -> _SCLTIMEOUTENW {
        _SCLTIMEOUTENW { w: self }
    }
}
