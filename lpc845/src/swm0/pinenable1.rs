#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINENABLE1 {
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
#[doc = "Possible values of the field `CAPT_X4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X4R {
    #[doc = "CAPT_X4 enabled on pin PIO1_3."]
    ENABLED,
    #[doc = "CAPT_X4 disabled."]
    DISABLED,
}
impl CAPT_X4R {
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
            CAPT_X4R::ENABLED => false,
            CAPT_X4R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X4R {
        match value {
            false => CAPT_X4R::ENABLED,
            true => CAPT_X4R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X4R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X4R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X5R {
    #[doc = "CAPT_X5 enabled on pin PIO1_4."]
    ENABLED,
    #[doc = "CAPT_X5 disabled."]
    DISABLED,
}
impl CAPT_X5R {
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
            CAPT_X5R::ENABLED => false,
            CAPT_X5R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X5R {
        match value {
            false => CAPT_X5R::ENABLED,
            true => CAPT_X5R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X5R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X5R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X6R {
    #[doc = "CAPT_X6 enabled on pin PIO1_5."]
    ENABLED,
    #[doc = "CAPT_X6 disabled."]
    DISABLED,
}
impl CAPT_X6R {
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
            CAPT_X6R::ENABLED => false,
            CAPT_X6R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X6R {
        match value {
            false => CAPT_X6R::ENABLED,
            true => CAPT_X6R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X6R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X6R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X7R {
    #[doc = "CAPT_X7 enabled on pin PIO1_6."]
    ENABLED,
    #[doc = "CAPT_X7 disabled."]
    DISABLED,
}
impl CAPT_X7R {
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
            CAPT_X7R::ENABLED => false,
            CAPT_X7R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X7R {
        match value {
            false => CAPT_X7R::ENABLED,
            true => CAPT_X7R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X7R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X7R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X8R {
    #[doc = "CAPT_X8 enabled on pin PIO1_7."]
    ENABLED,
    #[doc = "CAPT_X8 disabled."]
    DISABLED,
}
impl CAPT_X8R {
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
            CAPT_X8R::ENABLED => false,
            CAPT_X8R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X8R {
        match value {
            false => CAPT_X8R::ENABLED,
            true => CAPT_X8R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X8R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X8R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_YL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_YLR {
    #[doc = "CAPT_YL enabled on pin PIO1_8."]
    ENABLED,
    #[doc = "CAPT_YL disabled."]
    DISABLED,
}
impl CAPT_YLR {
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
            CAPT_YLR::ENABLED => false,
            CAPT_YLR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_YLR {
        match value {
            false => CAPT_YLR::ENABLED,
            true => CAPT_YLR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_YLR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_YLR::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_YH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_YHR {
    #[doc = "CAPT_YH enabled on pin PIO1_9."]
    ENABLED,
    #[doc = "CAPT_YH disabled."]
    DISABLED,
}
impl CAPT_YHR {
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
            CAPT_YHR::ENABLED => false,
            CAPT_YHR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_YHR {
        match value {
            false => CAPT_YHR::ENABLED,
            true => CAPT_YHR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_YHR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_YHR::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAPT_X4`"]
pub enum CAPT_X4W {
    #[doc = "CAPT_X4 enabled on pin PIO1_3."]
    ENABLED,
    #[doc = "CAPT_X4 disabled."]
    DISABLED,
}
impl CAPT_X4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X4W::ENABLED => false,
            CAPT_X4W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X4W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X4 enabled on pin PIO1_3."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X4W::ENABLED)
    }
    #[doc = "CAPT_X4 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X4W::DISABLED)
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
#[doc = "Values that can be written to the field `CAPT_X5`"]
pub enum CAPT_X5W {
    #[doc = "CAPT_X5 enabled on pin PIO1_4."]
    ENABLED,
    #[doc = "CAPT_X5 disabled."]
    DISABLED,
}
impl CAPT_X5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X5W::ENABLED => false,
            CAPT_X5W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X5W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X5 enabled on pin PIO1_4."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X5W::ENABLED)
    }
    #[doc = "CAPT_X5 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X5W::DISABLED)
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
#[doc = "Values that can be written to the field `CAPT_X6`"]
pub enum CAPT_X6W {
    #[doc = "CAPT_X6 enabled on pin PIO1_5."]
    ENABLED,
    #[doc = "CAPT_X6 disabled."]
    DISABLED,
}
impl CAPT_X6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X6W::ENABLED => false,
            CAPT_X6W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X6W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X6 enabled on pin PIO1_5."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X6W::ENABLED)
    }
    #[doc = "CAPT_X6 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X6W::DISABLED)
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
#[doc = "Values that can be written to the field `CAPT_X7`"]
pub enum CAPT_X7W {
    #[doc = "CAPT_X7 enabled on pin PIO1_6."]
    ENABLED,
    #[doc = "CAPT_X7 disabled."]
    DISABLED,
}
impl CAPT_X7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X7W::ENABLED => false,
            CAPT_X7W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X7W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X7 enabled on pin PIO1_6."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X7W::ENABLED)
    }
    #[doc = "CAPT_X7 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X7W::DISABLED)
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
#[doc = "Values that can be written to the field `CAPT_X8`"]
pub enum CAPT_X8W {
    #[doc = "CAPT_X8 enabled on pin PIO1_7."]
    ENABLED,
    #[doc = "CAPT_X8 disabled."]
    DISABLED,
}
impl CAPT_X8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X8W::ENABLED => false,
            CAPT_X8W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X8W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X8 enabled on pin PIO1_7."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X8W::ENABLED)
    }
    #[doc = "CAPT_X8 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X8W::DISABLED)
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
#[doc = "Values that can be written to the field `CAPT_YL`"]
pub enum CAPT_YLW {
    #[doc = "CAPT_YL enabled on pin PIO1_8."]
    ENABLED,
    #[doc = "CAPT_YL disabled."]
    DISABLED,
}
impl CAPT_YLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_YLW::ENABLED => false,
            CAPT_YLW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_YLW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_YLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_YLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_YL enabled on pin PIO1_8."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_YLW::ENABLED)
    }
    #[doc = "CAPT_YL disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_YLW::DISABLED)
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
#[doc = "Values that can be written to the field `CAPT_YH`"]
pub enum CAPT_YHW {
    #[doc = "CAPT_YH enabled on pin PIO1_9."]
    ENABLED,
    #[doc = "CAPT_YH disabled."]
    DISABLED,
}
impl CAPT_YHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_YHW::ENABLED => false,
            CAPT_YHW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_YHW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_YHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_YHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_YH enabled on pin PIO1_9."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_YHW::ENABLED)
    }
    #[doc = "CAPT_YH disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_YHW::DISABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CAPT_X4 function select."]
    #[inline]
    pub fn capt_x4(&self) -> CAPT_X4R {
        CAPT_X4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CAPT_X5 function select."]
    #[inline]
    pub fn capt_x5(&self) -> CAPT_X5R {
        CAPT_X5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CAPT_X6 function select."]
    #[inline]
    pub fn capt_x6(&self) -> CAPT_X6R {
        CAPT_X6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - CAPT_X7 function select."]
    #[inline]
    pub fn capt_x7(&self) -> CAPT_X7R {
        CAPT_X7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - CAPT_X8 function select."]
    #[inline]
    pub fn capt_x8(&self) -> CAPT_X8R {
        CAPT_X8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - CAPT_YL function select."]
    #[inline]
    pub fn capt_yl(&self) -> CAPT_YLR {
        CAPT_YLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - CAPT_YH function select."]
    #[inline]
    pub fn capt_yh(&self) -> CAPT_YHR {
        CAPT_YHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - CAPT_X4 function select."]
    #[inline]
    pub fn capt_x4(&mut self) -> _CAPT_X4W {
        _CAPT_X4W { w: self }
    }
    #[doc = "Bit 1 - CAPT_X5 function select."]
    #[inline]
    pub fn capt_x5(&mut self) -> _CAPT_X5W {
        _CAPT_X5W { w: self }
    }
    #[doc = "Bit 2 - CAPT_X6 function select."]
    #[inline]
    pub fn capt_x6(&mut self) -> _CAPT_X6W {
        _CAPT_X6W { w: self }
    }
    #[doc = "Bit 3 - CAPT_X7 function select."]
    #[inline]
    pub fn capt_x7(&mut self) -> _CAPT_X7W {
        _CAPT_X7W { w: self }
    }
    #[doc = "Bit 4 - CAPT_X8 function select."]
    #[inline]
    pub fn capt_x8(&mut self) -> _CAPT_X8W {
        _CAPT_X8W { w: self }
    }
    #[doc = "Bit 5 - CAPT_YL function select."]
    #[inline]
    pub fn capt_yl(&mut self) -> _CAPT_YLW {
        _CAPT_YLW { w: self }
    }
    #[doc = "Bit 6 - CAPT_YH function select."]
    #[inline]
    pub fn capt_yh(&mut self) -> _CAPT_YHW {
        _CAPT_YHW { w: self }
    }
}
