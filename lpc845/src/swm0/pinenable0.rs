#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINENABLE0 {
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
#[doc = "Possible values of the field `ACMP_I1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I1R {
    #[doc = "ACMP_I1 enabled on pin PIO0_00."]
    ENABLED,
    #[doc = "ACMP_I1 disabled."]
    DISABLED,
}
impl ACMP_I1R {
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
            ACMP_I1R::ENABLED => false,
            ACMP_I1R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I1R {
        match value {
            false => ACMP_I1R::ENABLED,
            true => ACMP_I1R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I1R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I1R::DISABLED
    }
}
#[doc = "Possible values of the field `ACMP_I2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I2R {
    #[doc = "ACMP_I2 enabled on pin PIO0_1."]
    ACMP_I2_0,
    #[doc = "ACMP_I2 disabled."]
    ACMP_I2_1,
}
impl ACMP_I2R {
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
            ACMP_I2R::ACMP_I2_0 => false,
            ACMP_I2R::ACMP_I2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I2R {
        match value {
            false => ACMP_I2R::ACMP_I2_0,
            true => ACMP_I2R::ACMP_I2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_0`"]
    #[inline]
    pub fn is_acmp_i2_0(&self) -> bool {
        *self == ACMP_I2R::ACMP_I2_0
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_1`"]
    #[inline]
    pub fn is_acmp_i2_1(&self) -> bool {
        *self == ACMP_I2R::ACMP_I2_1
    }
}
#[doc = "Possible values of the field `ACMP_I3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I3R {
    #[doc = "ACMP_I3 enabled on pin PIO0_14."]
    ENABLED,
    #[doc = "ACMP_I3 disabled."]
    DISABLED,
}
impl ACMP_I3R {
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
            ACMP_I3R::ENABLED => false,
            ACMP_I3R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I3R {
        match value {
            false => ACMP_I3R::ENABLED,
            true => ACMP_I3R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I3R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I3R::DISABLED
    }
}
#[doc = "Possible values of the field `ACMP_I4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I4R {
    #[doc = "ACMP_I4 enabled on pin PIO0_23."]
    ENABLED,
    #[doc = "ACMP_I4 disabled."]
    DISABLED,
}
impl ACMP_I4R {
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
            ACMP_I4R::ENABLED => false,
            ACMP_I4R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I4R {
        match value {
            false => ACMP_I4R::ENABLED,
            true => ACMP_I4R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I4R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I4R::DISABLED
    }
}
#[doc = "Possible values of the field `ACMP_I5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I5R {
    #[doc = "ACMP_I5 enabled on pin PIO0_30."]
    ENABLED,
    #[doc = "ACMP_I5 disabled."]
    DISABLED,
}
impl ACMP_I5R {
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
            ACMP_I5R::ENABLED => false,
            ACMP_I5R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I5R {
        match value {
            false => ACMP_I5R::ENABLED,
            true => ACMP_I5R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I5R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I5R::DISABLED
    }
}
#[doc = "Possible values of the field `SWCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCLKR {
    #[doc = "SWCLK enabled on pin PIO0_3."]
    ENABLED,
    #[doc = "SWCLK disabled."]
    DISABLED,
}
impl SWCLKR {
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
            SWCLKR::ENABLED => false,
            SWCLKR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWCLKR {
        match value {
            false => SWCLKR::ENABLED,
            true => SWCLKR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SWCLKR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SWCLKR::DISABLED
    }
}
#[doc = "Possible values of the field `SWDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDIOR {
    #[doc = "SWDIO enabled on pin PIO0_2."]
    ENABLED,
    #[doc = "SWDIO disabled."]
    DISABLED,
}
impl SWDIOR {
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
            SWDIOR::ENABLED => false,
            SWDIOR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWDIOR {
        match value {
            false => SWDIOR::ENABLED,
            true => SWDIOR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SWDIOR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SWDIOR::DISABLED
    }
}
#[doc = "Possible values of the field `XTALIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALINR {
    #[doc = "XTALIN enabled on pin PIO0_8."]
    ENABLED,
    #[doc = "XTALIN disabled."]
    DISABLED,
}
impl XTALINR {
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
            XTALINR::ENABLED => false,
            XTALINR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALINR {
        match value {
            false => XTALINR::ENABLED,
            true => XTALINR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == XTALINR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == XTALINR::DISABLED
    }
}
#[doc = "Possible values of the field `XTALOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOUTR {
    #[doc = "XTALOUT enabled on pin PIO0_9."]
    ENABLED,
    #[doc = "XTALOUT disabled."]
    DISABLED,
}
impl XTALOUTR {
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
            XTALOUTR::ENABLED => false,
            XTALOUTR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALOUTR {
        match value {
            false => XTALOUTR::ENABLED,
            true => XTALOUTR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == XTALOUTR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == XTALOUTR::DISABLED
    }
}
#[doc = "Possible values of the field `RESETN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETNR {
    #[doc = "RESETN enabled on pin PIO0_5."]
    ENABLED,
    #[doc = "RESETN disabled."]
    DISABLED,
}
impl RESETNR {
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
            RESETNR::ENABLED => false,
            RESETNR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETNR {
        match value {
            false => RESETNR::ENABLED,
            true => RESETNR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RESETNR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RESETNR::DISABLED
    }
}
#[doc = "Possible values of the field `CLKIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKINR {
    #[doc = "CLKIN enabled on pin PIO0_1."]
    ENABLED,
    #[doc = "CLKIN disabled."]
    DISABLED,
}
impl CLKINR {
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
            CLKINR::ENABLED => false,
            CLKINR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKINR {
        match value {
            false => CLKINR::ENABLED,
            true => CLKINR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CLKINR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CLKINR::DISABLED
    }
}
#[doc = "Possible values of the field `VDDCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCMPR {
    #[doc = "VDDCMP enabled on pin PIO0_6."]
    ENABLED,
    #[doc = "VDDCMP disabled."]
    DISABLED,
}
impl VDDCMPR {
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
            VDDCMPR::ENABLED => false,
            VDDCMPR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDCMPR {
        match value {
            false => VDDCMPR::ENABLED,
            true => VDDCMPR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == VDDCMPR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == VDDCMPR::DISABLED
    }
}
#[doc = "Possible values of the field `I2C0_SDA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_SDAR {
    #[doc = "I2C0_SDA enabled on pin PIO0_11."]
    ENABLED,
    #[doc = "I2C0_SDA disabled."]
    DISABLED,
}
impl I2C0_SDAR {
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
            I2C0_SDAR::ENABLED => false,
            I2C0_SDAR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C0_SDAR {
        match value {
            false => I2C0_SDAR::ENABLED,
            true => I2C0_SDAR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == I2C0_SDAR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == I2C0_SDAR::DISABLED
    }
}
#[doc = "Possible values of the field `I2C0_SCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_SCLR {
    #[doc = "I2C0_SCL enabled on pin PIO0_10."]
    ENABLED,
    #[doc = "I2C0_SCL disabled."]
    DISABLED,
}
impl I2C0_SCLR {
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
            I2C0_SCLR::ENABLED => false,
            I2C0_SCLR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C0_SCLR {
        match value {
            false => I2C0_SCLR::ENABLED,
            true => I2C0_SCLR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == I2C0_SCLR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == I2C0_SCLR::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_0R {
    #[doc = "ADC_0 enabled on pin PIO0_7."]
    ENABLED,
    #[doc = "ADC_0 disabled."]
    DISABLED,
}
impl ADC_0R {
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
            ADC_0R::ENABLED => false,
            ADC_0R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_0R {
        match value {
            false => ADC_0R::ENABLED,
            true => ADC_0R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_0R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_0R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_1R {
    #[doc = "ADC_1 enabled on pin PIO0_6."]
    ENABLED,
    #[doc = "ADC_1 disabled."]
    DISABLED,
}
impl ADC_1R {
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
            ADC_1R::ENABLED => false,
            ADC_1R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_1R {
        match value {
            false => ADC_1R::ENABLED,
            true => ADC_1R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_1R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_1R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_2R {
    #[doc = "ADC_2 enabled on pin PIO0_14."]
    ENABLED,
    #[doc = "ADC_2 disabled."]
    DISABLED,
}
impl ADC_2R {
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
            ADC_2R::ENABLED => false,
            ADC_2R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_2R {
        match value {
            false => ADC_2R::ENABLED,
            true => ADC_2R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_2R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_2R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_3R {
    #[doc = "ADC_3 enabled on pin PIO0_23."]
    ENABLED,
    #[doc = "ADC_3 disabled."]
    DISABLED,
}
impl ADC_3R {
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
            ADC_3R::ENABLED => false,
            ADC_3R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_3R {
        match value {
            false => ADC_3R::ENABLED,
            true => ADC_3R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_3R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_3R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_4R {
    #[doc = "ADC_4 enabled on pin PIO0_22."]
    ENABLED,
    #[doc = "ADC_4 disabled."]
    DISABLED,
}
impl ADC_4R {
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
            ADC_4R::ENABLED => false,
            ADC_4R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_4R {
        match value {
            false => ADC_4R::ENABLED,
            true => ADC_4R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_4R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_4R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_5R {
    #[doc = "ADC_5 enabled on pin PIO0_21."]
    ENABLED,
    #[doc = "ADC_5 disabled."]
    DISABLED,
}
impl ADC_5R {
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
            ADC_5R::ENABLED => false,
            ADC_5R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_5R {
        match value {
            false => ADC_5R::ENABLED,
            true => ADC_5R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_5R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_5R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_6R {
    #[doc = "ADC_6 enabled on pin PIO0_20."]
    ENABLED,
    #[doc = "ADC_6 disabled."]
    DISABLED,
}
impl ADC_6R {
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
            ADC_6R::ENABLED => false,
            ADC_6R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_6R {
        match value {
            false => ADC_6R::ENABLED,
            true => ADC_6R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_6R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_6R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_7R {
    #[doc = "ADC_7 enabled on pin PIO0_19."]
    ENABLED,
    #[doc = "ADC_7 disabled."]
    DISABLED,
}
impl ADC_7R {
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
            ADC_7R::ENABLED => false,
            ADC_7R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_7R {
        match value {
            false => ADC_7R::ENABLED,
            true => ADC_7R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_7R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_7R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_8R {
    #[doc = "ADC_8 enabled on pin PIO0_18."]
    ENABLED,
    #[doc = "ADC_8 disabled."]
    DISABLED,
}
impl ADC_8R {
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
            ADC_8R::ENABLED => false,
            ADC_8R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_8R {
        match value {
            false => ADC_8R::ENABLED,
            true => ADC_8R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_8R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_8R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_9R {
    #[doc = "ADC_9 enabled on pin PIO0_17."]
    ENABLED,
    #[doc = "ADC_9 disabled."]
    DISABLED,
}
impl ADC_9R {
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
            ADC_9R::ENABLED => false,
            ADC_9R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_9R {
        match value {
            false => ADC_9R::ENABLED,
            true => ADC_9R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_9R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_9R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_10R {
    #[doc = "ADC_10 enabled on pin PIO0_13."]
    ENABLED,
    #[doc = "ADC_10 disabled."]
    DISABLED,
}
impl ADC_10R {
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
            ADC_10R::ENABLED => false,
            ADC_10R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_10R {
        match value {
            false => ADC_10R::ENABLED,
            true => ADC_10R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_10R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_10R::DISABLED
    }
}
#[doc = "Possible values of the field `ADC_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_11R {
    #[doc = "ADC_11 enabled on pin PIO0_4."]
    ENABLED,
    #[doc = "ADC_11 disabled."]
    DISABLED,
}
impl ADC_11R {
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
            ADC_11R::ENABLED => false,
            ADC_11R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_11R {
        match value {
            false => ADC_11R::ENABLED,
            true => ADC_11R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_11R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_11R::DISABLED
    }
}
#[doc = "Possible values of the field `DACOUT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACOUT0R {
    #[doc = "DACOUT0 enabled on pin PIO0_17."]
    ENABLED,
    #[doc = "DACOUT0 disabled."]
    DISABLED,
}
impl DACOUT0R {
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
            DACOUT0R::ENABLED => false,
            DACOUT0R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACOUT0R {
        match value {
            false => DACOUT0R::ENABLED,
            true => DACOUT0R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DACOUT0R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DACOUT0R::DISABLED
    }
}
#[doc = "Possible values of the field `DACOUT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACOUT1R {
    #[doc = "DACOUT1 enabled on pin PIO0_29."]
    ENABLED,
    #[doc = "DACOUT1 disabled."]
    DISABLED,
}
impl DACOUT1R {
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
            DACOUT1R::ENABLED => false,
            DACOUT1R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACOUT1R {
        match value {
            false => DACOUT1R::ENABLED,
            true => DACOUT1R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DACOUT1R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DACOUT1R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X0R {
    #[doc = "CAPT_X0 enabled on pin PIO0_31."]
    ENABLED,
    #[doc = "CAPT_X0 disabled."]
    DISABLED,
}
impl CAPT_X0R {
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
            CAPT_X0R::ENABLED => false,
            CAPT_X0R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X0R {
        match value {
            false => CAPT_X0R::ENABLED,
            true => CAPT_X0R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X0R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X0R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X1R {
    #[doc = "CAPT_X1 enabled on pin PIO1_0."]
    ENABLED,
    #[doc = "CAPT_X1 disabled."]
    DISABLED,
}
impl CAPT_X1R {
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
            CAPT_X1R::ENABLED => false,
            CAPT_X1R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X1R {
        match value {
            false => CAPT_X1R::ENABLED,
            true => CAPT_X1R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X1R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X1R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X2R {
    #[doc = "CAPT_X2 enabled on pin PIO1_1."]
    ENABLED,
    #[doc = "CAPT_X2 disabled."]
    DISABLED,
}
impl CAPT_X2R {
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
            CAPT_X2R::ENABLED => false,
            CAPT_X2R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X2R {
        match value {
            false => CAPT_X2R::ENABLED,
            true => CAPT_X2R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X2R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X2R::DISABLED
    }
}
#[doc = "Possible values of the field `CAPT_X3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X3R {
    #[doc = "CAPT_X3 enabled on pin PIO1_2."]
    ENABLED,
    #[doc = "CAPT_X3 disabled."]
    DISABLED,
}
impl CAPT_X3R {
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
            CAPT_X3R::ENABLED => false,
            CAPT_X3R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_X3R {
        match value {
            false => CAPT_X3R::ENABLED,
            true => CAPT_X3R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X3R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X3R::DISABLED
    }
}
#[doc = "Values that can be written to the field `ACMP_I1`"]
pub enum ACMP_I1W {
    #[doc = "ACMP_I1 enabled on pin PIO0_00."]
    ENABLED,
    #[doc = "ACMP_I1 disabled."]
    DISABLED,
}
impl ACMP_I1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I1W::ENABLED => false,
            ACMP_I1W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_I1W<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_I1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_I1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACMP_I1 enabled on pin PIO0_00."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I1W::ENABLED)
    }
    #[doc = "ACMP_I1 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I1W::DISABLED)
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
#[doc = "Values that can be written to the field `ACMP_I2`"]
pub enum ACMP_I2W {
    #[doc = "ACMP_I2 enabled on pin PIO0_1."]
    ACMP_I2_0,
    #[doc = "ACMP_I2 disabled."]
    ACMP_I2_1,
}
impl ACMP_I2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I2W::ACMP_I2_0 => false,
            ACMP_I2W::ACMP_I2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_I2W<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_I2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_I2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACMP_I2 enabled on pin PIO0_1."]
    #[inline]
    pub fn acmp_i2_0(self) -> &'a mut W {
        self.variant(ACMP_I2W::ACMP_I2_0)
    }
    #[doc = "ACMP_I2 disabled."]
    #[inline]
    pub fn acmp_i2_1(self) -> &'a mut W {
        self.variant(ACMP_I2W::ACMP_I2_1)
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
#[doc = "Values that can be written to the field `ACMP_I3`"]
pub enum ACMP_I3W {
    #[doc = "ACMP_I3 enabled on pin PIO0_14."]
    ENABLED,
    #[doc = "ACMP_I3 disabled."]
    DISABLED,
}
impl ACMP_I3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I3W::ENABLED => false,
            ACMP_I3W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_I3W<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_I3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_I3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACMP_I3 enabled on pin PIO0_14."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I3W::ENABLED)
    }
    #[doc = "ACMP_I3 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I3W::DISABLED)
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
#[doc = "Values that can be written to the field `ACMP_I4`"]
pub enum ACMP_I4W {
    #[doc = "ACMP_I4 enabled on pin PIO0_23."]
    ENABLED,
    #[doc = "ACMP_I4 disabled."]
    DISABLED,
}
impl ACMP_I4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I4W::ENABLED => false,
            ACMP_I4W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_I4W<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_I4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_I4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACMP_I4 enabled on pin PIO0_23."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I4W::ENABLED)
    }
    #[doc = "ACMP_I4 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I4W::DISABLED)
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
#[doc = "Values that can be written to the field `ACMP_I5`"]
pub enum ACMP_I5W {
    #[doc = "ACMP_I5 enabled on pin PIO0_30."]
    ENABLED,
    #[doc = "ACMP_I5 disabled."]
    DISABLED,
}
impl ACMP_I5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I5W::ENABLED => false,
            ACMP_I5W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_I5W<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_I5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_I5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACMP_I5 enabled on pin PIO0_30."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I5W::ENABLED)
    }
    #[doc = "ACMP_I5 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I5W::DISABLED)
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
#[doc = "Values that can be written to the field `SWCLK`"]
pub enum SWCLKW {
    #[doc = "SWCLK enabled on pin PIO0_3."]
    ENABLED,
    #[doc = "SWCLK disabled."]
    DISABLED,
}
impl SWCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWCLKW::ENABLED => false,
            SWCLKW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SWCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SWCLK enabled on pin PIO0_3."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWCLKW::ENABLED)
    }
    #[doc = "SWCLK disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWCLKW::DISABLED)
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
#[doc = "Values that can be written to the field `SWDIO`"]
pub enum SWDIOW {
    #[doc = "SWDIO enabled on pin PIO0_2."]
    ENABLED,
    #[doc = "SWDIO disabled."]
    DISABLED,
}
impl SWDIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWDIOW::ENABLED => false,
            SWDIOW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SWDIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWDIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SWDIO enabled on pin PIO0_2."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWDIOW::ENABLED)
    }
    #[doc = "SWDIO disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWDIOW::DISABLED)
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
#[doc = "Values that can be written to the field `XTALIN`"]
pub enum XTALINW {
    #[doc = "XTALIN enabled on pin PIO0_8."]
    ENABLED,
    #[doc = "XTALIN disabled."]
    DISABLED,
}
impl XTALINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALINW::ENABLED => false,
            XTALINW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALINW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XTALIN enabled on pin PIO0_8."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(XTALINW::ENABLED)
    }
    #[doc = "XTALIN disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(XTALINW::DISABLED)
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
#[doc = "Values that can be written to the field `XTALOUT`"]
pub enum XTALOUTW {
    #[doc = "XTALOUT enabled on pin PIO0_9."]
    ENABLED,
    #[doc = "XTALOUT disabled."]
    DISABLED,
}
impl XTALOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALOUTW::ENABLED => false,
            XTALOUTW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XTALOUT enabled on pin PIO0_9."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(XTALOUTW::ENABLED)
    }
    #[doc = "XTALOUT disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(XTALOUTW::DISABLED)
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
#[doc = "Values that can be written to the field `RESETN`"]
pub enum RESETNW {
    #[doc = "RESETN enabled on pin PIO0_5."]
    ENABLED,
    #[doc = "RESETN disabled."]
    DISABLED,
}
impl RESETNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETNW::ENABLED => false,
            RESETNW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETNW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RESETN enabled on pin PIO0_5."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESETNW::ENABLED)
    }
    #[doc = "RESETN disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESETNW::DISABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKIN`"]
pub enum CLKINW {
    #[doc = "CLKIN enabled on pin PIO0_1."]
    ENABLED,
    #[doc = "CLKIN disabled."]
    DISABLED,
}
impl CLKINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKINW::ENABLED => false,
            CLKINW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKINW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CLKIN enabled on pin PIO0_1."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKINW::ENABLED)
    }
    #[doc = "CLKIN disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKINW::DISABLED)
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
#[doc = "Values that can be written to the field `VDDCMP`"]
pub enum VDDCMPW {
    #[doc = "VDDCMP enabled on pin PIO0_6."]
    ENABLED,
    #[doc = "VDDCMP disabled."]
    DISABLED,
}
impl VDDCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VDDCMPW::ENABLED => false,
            VDDCMPW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDDCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDCMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDDCMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VDDCMP enabled on pin PIO0_6."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VDDCMPW::ENABLED)
    }
    #[doc = "VDDCMP disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VDDCMPW::DISABLED)
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
#[doc = "Values that can be written to the field `I2C0_SDA`"]
pub enum I2C0_SDAW {
    #[doc = "I2C0_SDA enabled on pin PIO0_11."]
    ENABLED,
    #[doc = "I2C0_SDA disabled."]
    DISABLED,
}
impl I2C0_SDAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0_SDAW::ENABLED => false,
            I2C0_SDAW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C0_SDAW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0_SDAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C0_SDAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C0_SDA enabled on pin PIO0_11."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C0_SDAW::ENABLED)
    }
    #[doc = "I2C0_SDA disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C0_SDAW::DISABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C0_SCL`"]
pub enum I2C0_SCLW {
    #[doc = "I2C0_SCL enabled on pin PIO0_10."]
    ENABLED,
    #[doc = "I2C0_SCL disabled."]
    DISABLED,
}
impl I2C0_SCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0_SCLW::ENABLED => false,
            I2C0_SCLW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C0_SCLW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0_SCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C0_SCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C0_SCL enabled on pin PIO0_10."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C0_SCLW::ENABLED)
    }
    #[doc = "I2C0_SCL disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C0_SCLW::DISABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_0`"]
pub enum ADC_0W {
    #[doc = "ADC_0 enabled on pin PIO0_7."]
    ENABLED,
    #[doc = "ADC_0 disabled."]
    DISABLED,
}
impl ADC_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_0W::ENABLED => false,
            ADC_0W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_0 enabled on pin PIO0_7."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_0W::ENABLED)
    }
    #[doc = "ADC_0 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_0W::DISABLED)
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
#[doc = "Values that can be written to the field `ADC_1`"]
pub enum ADC_1W {
    #[doc = "ADC_1 enabled on pin PIO0_6."]
    ENABLED,
    #[doc = "ADC_1 disabled."]
    DISABLED,
}
impl ADC_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_1W::ENABLED => false,
            ADC_1W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_1 enabled on pin PIO0_6."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_1W::ENABLED)
    }
    #[doc = "ADC_1 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_1W::DISABLED)
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
#[doc = "Values that can be written to the field `ADC_2`"]
pub enum ADC_2W {
    #[doc = "ADC_2 enabled on pin PIO0_14."]
    ENABLED,
    #[doc = "ADC_2 disabled."]
    DISABLED,
}
impl ADC_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_2W::ENABLED => false,
            ADC_2W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_2 enabled on pin PIO0_14."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_2W::ENABLED)
    }
    #[doc = "ADC_2 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_2W::DISABLED)
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
#[doc = "Values that can be written to the field `ADC_3`"]
pub enum ADC_3W {
    #[doc = "ADC_3 enabled on pin PIO0_23."]
    ENABLED,
    #[doc = "ADC_3 disabled."]
    DISABLED,
}
impl ADC_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_3W::ENABLED => false,
            ADC_3W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_3 enabled on pin PIO0_23."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_3W::ENABLED)
    }
    #[doc = "ADC_3 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_3W::DISABLED)
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
#[doc = "Values that can be written to the field `ADC_4`"]
pub enum ADC_4W {
    #[doc = "ADC_4 enabled on pin PIO0_22."]
    ENABLED,
    #[doc = "ADC_4 disabled."]
    DISABLED,
}
impl ADC_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_4W::ENABLED => false,
            ADC_4W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_4 enabled on pin PIO0_22."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_4W::ENABLED)
    }
    #[doc = "ADC_4 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_4W::DISABLED)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_5`"]
pub enum ADC_5W {
    #[doc = "ADC_5 enabled on pin PIO0_21."]
    ENABLED,
    #[doc = "ADC_5 disabled."]
    DISABLED,
}
impl ADC_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_5W::ENABLED => false,
            ADC_5W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_5 enabled on pin PIO0_21."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_5W::ENABLED)
    }
    #[doc = "ADC_5 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_5W::DISABLED)
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
#[doc = "Values that can be written to the field `ADC_6`"]
pub enum ADC_6W {
    #[doc = "ADC_6 enabled on pin PIO0_20."]
    ENABLED,
    #[doc = "ADC_6 disabled."]
    DISABLED,
}
impl ADC_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_6W::ENABLED => false,
            ADC_6W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_6 enabled on pin PIO0_20."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_6W::ENABLED)
    }
    #[doc = "ADC_6 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_6W::DISABLED)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_7`"]
pub enum ADC_7W {
    #[doc = "ADC_7 enabled on pin PIO0_19."]
    ENABLED,
    #[doc = "ADC_7 disabled."]
    DISABLED,
}
impl ADC_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_7W::ENABLED => false,
            ADC_7W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_7 enabled on pin PIO0_19."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_7W::ENABLED)
    }
    #[doc = "ADC_7 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_7W::DISABLED)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_8`"]
pub enum ADC_8W {
    #[doc = "ADC_8 enabled on pin PIO0_18."]
    ENABLED,
    #[doc = "ADC_8 disabled."]
    DISABLED,
}
impl ADC_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_8W::ENABLED => false,
            ADC_8W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_8W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_8 enabled on pin PIO0_18."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_8W::ENABLED)
    }
    #[doc = "ADC_8 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_8W::DISABLED)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_9`"]
pub enum ADC_9W {
    #[doc = "ADC_9 enabled on pin PIO0_17."]
    ENABLED,
    #[doc = "ADC_9 disabled."]
    DISABLED,
}
impl ADC_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_9W::ENABLED => false,
            ADC_9W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_9W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_9 enabled on pin PIO0_17."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_9W::ENABLED)
    }
    #[doc = "ADC_9 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_9W::DISABLED)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_10`"]
pub enum ADC_10W {
    #[doc = "ADC_10 enabled on pin PIO0_13."]
    ENABLED,
    #[doc = "ADC_10 disabled."]
    DISABLED,
}
impl ADC_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_10W::ENABLED => false,
            ADC_10W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_10W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_10 enabled on pin PIO0_13."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_10W::ENABLED)
    }
    #[doc = "ADC_10 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_10W::DISABLED)
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
#[doc = "Values that can be written to the field `ADC_11`"]
pub enum ADC_11W {
    #[doc = "ADC_11 enabled on pin PIO0_4."]
    ENABLED,
    #[doc = "ADC_11 disabled."]
    DISABLED,
}
impl ADC_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_11W::ENABLED => false,
            ADC_11W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_11W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC_11 enabled on pin PIO0_4."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_11W::ENABLED)
    }
    #[doc = "ADC_11 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_11W::DISABLED)
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
#[doc = "Values that can be written to the field `DACOUT0`"]
pub enum DACOUT0W {
    #[doc = "DACOUT0 enabled on pin PIO0_17."]
    ENABLED,
    #[doc = "DACOUT0 disabled."]
    DISABLED,
}
impl DACOUT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACOUT0W::ENABLED => false,
            DACOUT0W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACOUT0W<'a> {
    w: &'a mut W,
}
impl<'a> _DACOUT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACOUT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DACOUT0 enabled on pin PIO0_17."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACOUT0W::ENABLED)
    }
    #[doc = "DACOUT0 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACOUT0W::DISABLED)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DACOUT1`"]
pub enum DACOUT1W {
    #[doc = "DACOUT1 enabled on pin PIO0_29."]
    ENABLED,
    #[doc = "DACOUT1 disabled."]
    DISABLED,
}
impl DACOUT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACOUT1W::ENABLED => false,
            DACOUT1W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACOUT1W<'a> {
    w: &'a mut W,
}
impl<'a> _DACOUT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACOUT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DACOUT1 enabled on pin PIO0_29."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACOUT1W::ENABLED)
    }
    #[doc = "DACOUT1 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACOUT1W::DISABLED)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPT_X0`"]
pub enum CAPT_X0W {
    #[doc = "CAPT_X0 enabled on pin PIO0_31."]
    ENABLED,
    #[doc = "CAPT_X0 disabled."]
    DISABLED,
}
impl CAPT_X0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X0W::ENABLED => false,
            CAPT_X0W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X0W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X0 enabled on pin PIO0_31."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X0W::ENABLED)
    }
    #[doc = "CAPT_X0 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X0W::DISABLED)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPT_X1`"]
pub enum CAPT_X1W {
    #[doc = "CAPT_X1 enabled on pin PIO1_0."]
    ENABLED,
    #[doc = "CAPT_X1 disabled."]
    DISABLED,
}
impl CAPT_X1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X1W::ENABLED => false,
            CAPT_X1W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X1W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X1 enabled on pin PIO1_0."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X1W::ENABLED)
    }
    #[doc = "CAPT_X1 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X1W::DISABLED)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPT_X2`"]
pub enum CAPT_X2W {
    #[doc = "CAPT_X2 enabled on pin PIO1_1."]
    ENABLED,
    #[doc = "CAPT_X2 disabled."]
    DISABLED,
}
impl CAPT_X2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X2W::ENABLED => false,
            CAPT_X2W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X2W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X2 enabled on pin PIO1_1."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X2W::ENABLED)
    }
    #[doc = "CAPT_X2 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X2W::DISABLED)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPT_X3`"]
pub enum CAPT_X3W {
    #[doc = "CAPT_X3 enabled on pin PIO1_2."]
    ENABLED,
    #[doc = "CAPT_X3 disabled."]
    DISABLED,
}
impl CAPT_X3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_X3W::ENABLED => false,
            CAPT_X3W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_X3W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_X3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_X3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAPT_X3 enabled on pin PIO1_2."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X3W::ENABLED)
    }
    #[doc = "CAPT_X3 disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X3W::DISABLED)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - ACMP_I1 function select."]
    #[inline]
    pub fn acmp_i1(&self) -> ACMP_I1R {
        ACMP_I1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ACMP_I2 function select."]
    #[inline]
    pub fn acmp_i2(&self) -> ACMP_I2R {
        ACMP_I2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ACMP_I3 function select."]
    #[inline]
    pub fn acmp_i3(&self) -> ACMP_I3R {
        ACMP_I3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ACMP_I4 function select."]
    #[inline]
    pub fn acmp_i4(&self) -> ACMP_I4R {
        ACMP_I4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ACMP_I5 function select."]
    #[inline]
    pub fn acmp_i5(&self) -> ACMP_I5R {
        ACMP_I5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - SWCLK function select."]
    #[inline]
    pub fn swclk(&self) -> SWCLKR {
        SWCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - SWDIO function select."]
    #[inline]
    pub fn swdio(&self) -> SWDIOR {
        SWDIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - XTALIN function select."]
    #[inline]
    pub fn xtalin(&self) -> XTALINR {
        XTALINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - XTALOUT function select."]
    #[inline]
    pub fn xtalout(&self) -> XTALOUTR {
        XTALOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - RESETN function select."]
    #[inline]
    pub fn resetn(&self) -> RESETNR {
        RESETNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CLKIN function select."]
    #[inline]
    pub fn clkin(&self) -> CLKINR {
        CLKINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - VDDCMP function select."]
    #[inline]
    pub fn vddcmp(&self) -> VDDCMPR {
        VDDCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - I2C0_SDA function select."]
    #[inline]
    pub fn i2c0_sda(&self) -> I2C0_SDAR {
        I2C0_SDAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - I2C0_SCL function select."]
    #[inline]
    pub fn i2c0_scl(&self) -> I2C0_SCLR {
        I2C0_SCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - ADC_0 function select."]
    #[inline]
    pub fn adc_0(&self) -> ADC_0R {
        ADC_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - ADC_1 function select."]
    #[inline]
    pub fn adc_1(&self) -> ADC_1R {
        ADC_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - ADC_2 function select."]
    #[inline]
    pub fn adc_2(&self) -> ADC_2R {
        ADC_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - ADC_3 function select."]
    #[inline]
    pub fn adc_3(&self) -> ADC_3R {
        ADC_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - ADC_4 function select."]
    #[inline]
    pub fn adc_4(&self) -> ADC_4R {
        ADC_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - ADC_5 function select."]
    #[inline]
    pub fn adc_5(&self) -> ADC_5R {
        ADC_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - ADC_6 function select."]
    #[inline]
    pub fn adc_6(&self) -> ADC_6R {
        ADC_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - ADC_7 function select."]
    #[inline]
    pub fn adc_7(&self) -> ADC_7R {
        ADC_7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - ADC_8 function select."]
    #[inline]
    pub fn adc_8(&self) -> ADC_8R {
        ADC_8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - ADC_9 function select."]
    #[inline]
    pub fn adc_9(&self) -> ADC_9R {
        ADC_9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - ADC_10 function select."]
    #[inline]
    pub fn adc_10(&self) -> ADC_10R {
        ADC_10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - ADC_11 function select."]
    #[inline]
    pub fn adc_11(&self) -> ADC_11R {
        ADC_11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - DACOUT0 function select."]
    #[inline]
    pub fn dacout0(&self) -> DACOUT0R {
        DACOUT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - DACOUT1 function select."]
    #[inline]
    pub fn dacout1(&self) -> DACOUT1R {
        DACOUT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - CAPT_X0 function select."]
    #[inline]
    pub fn capt_x0(&self) -> CAPT_X0R {
        CAPT_X0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - CAPT_X1 function select."]
    #[inline]
    pub fn capt_x1(&self) -> CAPT_X1R {
        CAPT_X1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - CAPT_X2 function select."]
    #[inline]
    pub fn capt_x2(&self) -> CAPT_X2R {
        CAPT_X2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - CAPT_X3 function select."]
    #[inline]
    pub fn capt_x3(&self) -> CAPT_X3R {
        CAPT_X3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294966687 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ACMP_I1 function select."]
    #[inline]
    pub fn acmp_i1(&mut self) -> _ACMP_I1W {
        _ACMP_I1W { w: self }
    }
    #[doc = "Bit 1 - ACMP_I2 function select."]
    #[inline]
    pub fn acmp_i2(&mut self) -> _ACMP_I2W {
        _ACMP_I2W { w: self }
    }
    #[doc = "Bit 2 - ACMP_I3 function select."]
    #[inline]
    pub fn acmp_i3(&mut self) -> _ACMP_I3W {
        _ACMP_I3W { w: self }
    }
    #[doc = "Bit 3 - ACMP_I4 function select."]
    #[inline]
    pub fn acmp_i4(&mut self) -> _ACMP_I4W {
        _ACMP_I4W { w: self }
    }
    #[doc = "Bit 4 - ACMP_I5 function select."]
    #[inline]
    pub fn acmp_i5(&mut self) -> _ACMP_I5W {
        _ACMP_I5W { w: self }
    }
    #[doc = "Bit 5 - SWCLK function select."]
    #[inline]
    pub fn swclk(&mut self) -> _SWCLKW {
        _SWCLKW { w: self }
    }
    #[doc = "Bit 6 - SWDIO function select."]
    #[inline]
    pub fn swdio(&mut self) -> _SWDIOW {
        _SWDIOW { w: self }
    }
    #[doc = "Bit 7 - XTALIN function select."]
    #[inline]
    pub fn xtalin(&mut self) -> _XTALINW {
        _XTALINW { w: self }
    }
    #[doc = "Bit 8 - XTALOUT function select."]
    #[inline]
    pub fn xtalout(&mut self) -> _XTALOUTW {
        _XTALOUTW { w: self }
    }
    #[doc = "Bit 9 - RESETN function select."]
    #[inline]
    pub fn resetn(&mut self) -> _RESETNW {
        _RESETNW { w: self }
    }
    #[doc = "Bit 10 - CLKIN function select."]
    #[inline]
    pub fn clkin(&mut self) -> _CLKINW {
        _CLKINW { w: self }
    }
    #[doc = "Bit 11 - VDDCMP function select."]
    #[inline]
    pub fn vddcmp(&mut self) -> _VDDCMPW {
        _VDDCMPW { w: self }
    }
    #[doc = "Bit 12 - I2C0_SDA function select."]
    #[inline]
    pub fn i2c0_sda(&mut self) -> _I2C0_SDAW {
        _I2C0_SDAW { w: self }
    }
    #[doc = "Bit 13 - I2C0_SCL function select."]
    #[inline]
    pub fn i2c0_scl(&mut self) -> _I2C0_SCLW {
        _I2C0_SCLW { w: self }
    }
    #[doc = "Bit 14 - ADC_0 function select."]
    #[inline]
    pub fn adc_0(&mut self) -> _ADC_0W {
        _ADC_0W { w: self }
    }
    #[doc = "Bit 15 - ADC_1 function select."]
    #[inline]
    pub fn adc_1(&mut self) -> _ADC_1W {
        _ADC_1W { w: self }
    }
    #[doc = "Bit 16 - ADC_2 function select."]
    #[inline]
    pub fn adc_2(&mut self) -> _ADC_2W {
        _ADC_2W { w: self }
    }
    #[doc = "Bit 17 - ADC_3 function select."]
    #[inline]
    pub fn adc_3(&mut self) -> _ADC_3W {
        _ADC_3W { w: self }
    }
    #[doc = "Bit 18 - ADC_4 function select."]
    #[inline]
    pub fn adc_4(&mut self) -> _ADC_4W {
        _ADC_4W { w: self }
    }
    #[doc = "Bit 19 - ADC_5 function select."]
    #[inline]
    pub fn adc_5(&mut self) -> _ADC_5W {
        _ADC_5W { w: self }
    }
    #[doc = "Bit 20 - ADC_6 function select."]
    #[inline]
    pub fn adc_6(&mut self) -> _ADC_6W {
        _ADC_6W { w: self }
    }
    #[doc = "Bit 21 - ADC_7 function select."]
    #[inline]
    pub fn adc_7(&mut self) -> _ADC_7W {
        _ADC_7W { w: self }
    }
    #[doc = "Bit 22 - ADC_8 function select."]
    #[inline]
    pub fn adc_8(&mut self) -> _ADC_8W {
        _ADC_8W { w: self }
    }
    #[doc = "Bit 23 - ADC_9 function select."]
    #[inline]
    pub fn adc_9(&mut self) -> _ADC_9W {
        _ADC_9W { w: self }
    }
    #[doc = "Bit 24 - ADC_10 function select."]
    #[inline]
    pub fn adc_10(&mut self) -> _ADC_10W {
        _ADC_10W { w: self }
    }
    #[doc = "Bit 25 - ADC_11 function select."]
    #[inline]
    pub fn adc_11(&mut self) -> _ADC_11W {
        _ADC_11W { w: self }
    }
    #[doc = "Bit 26 - DACOUT0 function select."]
    #[inline]
    pub fn dacout0(&mut self) -> _DACOUT0W {
        _DACOUT0W { w: self }
    }
    #[doc = "Bit 27 - DACOUT1 function select."]
    #[inline]
    pub fn dacout1(&mut self) -> _DACOUT1W {
        _DACOUT1W { w: self }
    }
    #[doc = "Bit 28 - CAPT_X0 function select."]
    #[inline]
    pub fn capt_x0(&mut self) -> _CAPT_X0W {
        _CAPT_X0W { w: self }
    }
    #[doc = "Bit 29 - CAPT_X1 function select."]
    #[inline]
    pub fn capt_x1(&mut self) -> _CAPT_X1W {
        _CAPT_X1W { w: self }
    }
    #[doc = "Bit 30 - CAPT_X2 function select."]
    #[inline]
    pub fn capt_x2(&mut self) -> _CAPT_X2W {
        _CAPT_X2W { w: self }
    }
    #[doc = "Bit 31 - CAPT_X3 function select."]
    #[inline]
    pub fn capt_x3(&mut self) -> _CAPT_X3W {
        _CAPT_X3W { w: self }
    }
}
