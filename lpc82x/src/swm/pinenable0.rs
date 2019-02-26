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
    ACMP_I1_ENABLED_ON_P,
    #[doc = "ACMP_I1 disabled."]
    ACMP_I1_DISABLED,
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
            ACMP_I1R::ACMP_I1_ENABLED_ON_P => false,
            ACMP_I1R::ACMP_I1_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I1R {
        match value {
            false => ACMP_I1R::ACMP_I1_ENABLED_ON_P,
            true => ACMP_I1R::ACMP_I1_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_I1_ENABLED_ON_P`"]
    #[inline]
    pub fn is_acmp_i1_enabled_on_p(&self) -> bool {
        *self == ACMP_I1R::ACMP_I1_ENABLED_ON_P
    }
    #[doc = "Checks if the value of the field is `ACMP_I1_DISABLED`"]
    #[inline]
    pub fn is_acmp_i1_disabled(&self) -> bool {
        *self == ACMP_I1R::ACMP_I1_DISABLED
    }
}
#[doc = "Possible values of the field `ACMP_I2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I2R {
    #[doc = "ACMP_I2 enabled on pin PIO0_1."]
    ACMP_I2_ENABLED_ON_P,
    #[doc = "ACMP_I2 disabled."]
    ACMP_I2_DISABLED,
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
            ACMP_I2R::ACMP_I2_ENABLED_ON_P => false,
            ACMP_I2R::ACMP_I2_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I2R {
        match value {
            false => ACMP_I2R::ACMP_I2_ENABLED_ON_P,
            true => ACMP_I2R::ACMP_I2_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_ENABLED_ON_P`"]
    #[inline]
    pub fn is_acmp_i2_enabled_on_p(&self) -> bool {
        *self == ACMP_I2R::ACMP_I2_ENABLED_ON_P
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_DISABLED`"]
    #[inline]
    pub fn is_acmp_i2_disabled(&self) -> bool {
        *self == ACMP_I2R::ACMP_I2_DISABLED
    }
}
#[doc = "Possible values of the field `ACMP_I3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I3R {
    #[doc = "ACMP_I3 enabled on pin PIO0_14."]
    ACMP_I3_ENABLED_ON_P,
    #[doc = "ACMP_I3 disabled."]
    ACMP_I3_DISABLED,
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
            ACMP_I3R::ACMP_I3_ENABLED_ON_P => false,
            ACMP_I3R::ACMP_I3_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I3R {
        match value {
            false => ACMP_I3R::ACMP_I3_ENABLED_ON_P,
            true => ACMP_I3R::ACMP_I3_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_I3_ENABLED_ON_P`"]
    #[inline]
    pub fn is_acmp_i3_enabled_on_p(&self) -> bool {
        *self == ACMP_I3R::ACMP_I3_ENABLED_ON_P
    }
    #[doc = "Checks if the value of the field is `ACMP_I3_DISABLED`"]
    #[inline]
    pub fn is_acmp_i3_disabled(&self) -> bool {
        *self == ACMP_I3R::ACMP_I3_DISABLED
    }
}
#[doc = "Possible values of the field `ACMP_I4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I4R {
    #[doc = "ACMP_I4 enabled on pin PIO0_23."]
    ACMP_I4_ENABLED_ON_P,
    #[doc = "ACMP_I4 disabled."]
    ACMP_I4_DISABLED,
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
            ACMP_I4R::ACMP_I4_ENABLED_ON_P => false,
            ACMP_I4R::ACMP_I4_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_I4R {
        match value {
            false => ACMP_I4R::ACMP_I4_ENABLED_ON_P,
            true => ACMP_I4R::ACMP_I4_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_I4_ENABLED_ON_P`"]
    #[inline]
    pub fn is_acmp_i4_enabled_on_p(&self) -> bool {
        *self == ACMP_I4R::ACMP_I4_ENABLED_ON_P
    }
    #[doc = "Checks if the value of the field is `ACMP_I4_DISABLED`"]
    #[inline]
    pub fn is_acmp_i4_disabled(&self) -> bool {
        *self == ACMP_I4R::ACMP_I4_DISABLED
    }
}
#[doc = "Possible values of the field `SWCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCLKR {
    #[doc = "SWCLK enabled on pin PIO0_3."]
    SWCLK_ENABLED_ON_PIN,
    #[doc = "SWCLK disabled."]
    SWCLK_DISABLED,
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
            SWCLKR::SWCLK_ENABLED_ON_PIN => false,
            SWCLKR::SWCLK_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWCLKR {
        match value {
            false => SWCLKR::SWCLK_ENABLED_ON_PIN,
            true => SWCLKR::SWCLK_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SWCLK_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_swclk_enabled_on_pin(&self) -> bool {
        *self == SWCLKR::SWCLK_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `SWCLK_DISABLED`"]
    #[inline]
    pub fn is_swclk_disabled(&self) -> bool {
        *self == SWCLKR::SWCLK_DISABLED
    }
}
#[doc = "Possible values of the field `SWDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDIOR {
    #[doc = "SWDIO enabled on pin PIO0_2."]
    SWDIO_ENABLED_ON_PIN,
    #[doc = "SWDIO disabled."]
    SWDIO_DISABLED,
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
            SWDIOR::SWDIO_ENABLED_ON_PIN => false,
            SWDIOR::SWDIO_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWDIOR {
        match value {
            false => SWDIOR::SWDIO_ENABLED_ON_PIN,
            true => SWDIOR::SWDIO_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SWDIO_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_swdio_enabled_on_pin(&self) -> bool {
        *self == SWDIOR::SWDIO_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `SWDIO_DISABLED`"]
    #[inline]
    pub fn is_swdio_disabled(&self) -> bool {
        *self == SWDIOR::SWDIO_DISABLED
    }
}
#[doc = "Possible values of the field `XTALIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALINR {
    #[doc = "XTALIN enabled on pin PIO0_8."]
    XTALIN_ENABLED_ON_PI,
    #[doc = "XTALIN disabled."]
    XTALIN_DISABLED,
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
            XTALINR::XTALIN_ENABLED_ON_PI => false,
            XTALINR::XTALIN_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALINR {
        match value {
            false => XTALINR::XTALIN_ENABLED_ON_PI,
            true => XTALINR::XTALIN_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `XTALIN_ENABLED_ON_PI`"]
    #[inline]
    pub fn is_xtalin_enabled_on_pi(&self) -> bool {
        *self == XTALINR::XTALIN_ENABLED_ON_PI
    }
    #[doc = "Checks if the value of the field is `XTALIN_DISABLED`"]
    #[inline]
    pub fn is_xtalin_disabled(&self) -> bool {
        *self == XTALINR::XTALIN_DISABLED
    }
}
#[doc = "Possible values of the field `XTALOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOUTR {
    #[doc = "XTALOUT enabled on pin PIO0_9."]
    XTALOUT_ENABLED_ON_P,
    #[doc = "XTALOUT disabled."]
    XTALOUT_DISABLED,
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
            XTALOUTR::XTALOUT_ENABLED_ON_P => false,
            XTALOUTR::XTALOUT_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTALOUTR {
        match value {
            false => XTALOUTR::XTALOUT_ENABLED_ON_P,
            true => XTALOUTR::XTALOUT_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `XTALOUT_ENABLED_ON_P`"]
    #[inline]
    pub fn is_xtalout_enabled_on_p(&self) -> bool {
        *self == XTALOUTR::XTALOUT_ENABLED_ON_P
    }
    #[doc = "Checks if the value of the field is `XTALOUT_DISABLED`"]
    #[inline]
    pub fn is_xtalout_disabled(&self) -> bool {
        *self == XTALOUTR::XTALOUT_DISABLED
    }
}
#[doc = "Possible values of the field `RESETN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETNR {
    #[doc = "RESETN enabled on pin PIO0_5."]
    RESETN_ENABLED_ON_PI,
    #[doc = "RESETN disabled."]
    RESETN_DISABLED,
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
            RESETNR::RESETN_ENABLED_ON_PI => false,
            RESETNR::RESETN_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETNR {
        match value {
            false => RESETNR::RESETN_ENABLED_ON_PI,
            true => RESETNR::RESETN_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `RESETN_ENABLED_ON_PI`"]
    #[inline]
    pub fn is_resetn_enabled_on_pi(&self) -> bool {
        *self == RESETNR::RESETN_ENABLED_ON_PI
    }
    #[doc = "Checks if the value of the field is `RESETN_DISABLED`"]
    #[inline]
    pub fn is_resetn_disabled(&self) -> bool {
        *self == RESETNR::RESETN_DISABLED
    }
}
#[doc = "Possible values of the field `CLKIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKINR {
    #[doc = "CLKIN enabled on pin PIO0_1."]
    CLKIN_ENABLED_ON_PIN,
    #[doc = "CLKIN disabled."]
    CLKIN_DISABLED,
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
            CLKINR::CLKIN_ENABLED_ON_PIN => false,
            CLKINR::CLKIN_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKINR {
        match value {
            false => CLKINR::CLKIN_ENABLED_ON_PIN,
            true => CLKINR::CLKIN_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CLKIN_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_clkin_enabled_on_pin(&self) -> bool {
        *self == CLKINR::CLKIN_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `CLKIN_DISABLED`"]
    #[inline]
    pub fn is_clkin_disabled(&self) -> bool {
        *self == CLKINR::CLKIN_DISABLED
    }
}
#[doc = "Possible values of the field `VDDCMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCMPR {
    #[doc = "VDDCMP enabled on pin PIO0_6."]
    VDDCMP_ENABLED_ON_PI,
    #[doc = "VDDCMP disabled."]
    VDDCMP_DISABLED,
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
            VDDCMPR::VDDCMP_ENABLED_ON_PI => false,
            VDDCMPR::VDDCMP_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDCMPR {
        match value {
            false => VDDCMPR::VDDCMP_ENABLED_ON_PI,
            true => VDDCMPR::VDDCMP_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `VDDCMP_ENABLED_ON_PI`"]
    #[inline]
    pub fn is_vddcmp_enabled_on_pi(&self) -> bool {
        *self == VDDCMPR::VDDCMP_ENABLED_ON_PI
    }
    #[doc = "Checks if the value of the field is `VDDCMP_DISABLED`"]
    #[inline]
    pub fn is_vddcmp_disabled(&self) -> bool {
        *self == VDDCMPR::VDDCMP_DISABLED
    }
}
#[doc = "Possible values of the field `I2C0_SDA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_SDAR {
    #[doc = "I2C0_SDA enabled on pin PIO0_11."]
    I2C0_SDA_ENABLED_ON,
    #[doc = "I2C0_SDA disabled."]
    I2C0_SDA_DISABLED,
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
            I2C0_SDAR::I2C0_SDA_ENABLED_ON => false,
            I2C0_SDAR::I2C0_SDA_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C0_SDAR {
        match value {
            false => I2C0_SDAR::I2C0_SDA_ENABLED_ON,
            true => I2C0_SDAR::I2C0_SDA_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `I2C0_SDA_ENABLED_ON`"]
    #[inline]
    pub fn is_i2c0_sda_enabled_on(&self) -> bool {
        *self == I2C0_SDAR::I2C0_SDA_ENABLED_ON
    }
    #[doc = "Checks if the value of the field is `I2C0_SDA_DISABLED`"]
    #[inline]
    pub fn is_i2c0_sda_disabled(&self) -> bool {
        *self == I2C0_SDAR::I2C0_SDA_DISABLED
    }
}
#[doc = "Possible values of the field `I2C0_SCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_SCLR {
    #[doc = "I2C0_SCL enabled on pin PIO0_10."]
    I2C0_SCL_ENABLED_ON,
    #[doc = "I2C0_SCL disabled."]
    I2C0_SCL_DISABLED,
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
            I2C0_SCLR::I2C0_SCL_ENABLED_ON => false,
            I2C0_SCLR::I2C0_SCL_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C0_SCLR {
        match value {
            false => I2C0_SCLR::I2C0_SCL_ENABLED_ON,
            true => I2C0_SCLR::I2C0_SCL_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `I2C0_SCL_ENABLED_ON`"]
    #[inline]
    pub fn is_i2c0_scl_enabled_on(&self) -> bool {
        *self == I2C0_SCLR::I2C0_SCL_ENABLED_ON
    }
    #[doc = "Checks if the value of the field is `I2C0_SCL_DISABLED`"]
    #[inline]
    pub fn is_i2c0_scl_disabled(&self) -> bool {
        *self == I2C0_SCLR::I2C0_SCL_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_0R {
    #[doc = "ADC_0 enabled on pin PIO0_7."]
    ADC_0_ENABLED_ON_PIN,
    #[doc = "ADC_0 disabled."]
    ADC_0_DISABLED,
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
            ADC_0R::ADC_0_ENABLED_ON_PIN => false,
            ADC_0R::ADC_0_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_0R {
        match value {
            false => ADC_0R::ADC_0_ENABLED_ON_PIN,
            true => ADC_0R::ADC_0_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_0_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_0_enabled_on_pin(&self) -> bool {
        *self == ADC_0R::ADC_0_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_0_DISABLED`"]
    #[inline]
    pub fn is_adc_0_disabled(&self) -> bool {
        *self == ADC_0R::ADC_0_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_1R {
    #[doc = "ADC_1 enabled on pin PIO0_6."]
    ADC_1_ENABLED_ON_PIN,
    #[doc = "ADC_1 disabled."]
    ADC_1_DISABLED,
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
            ADC_1R::ADC_1_ENABLED_ON_PIN => false,
            ADC_1R::ADC_1_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_1R {
        match value {
            false => ADC_1R::ADC_1_ENABLED_ON_PIN,
            true => ADC_1R::ADC_1_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_1_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_1_enabled_on_pin(&self) -> bool {
        *self == ADC_1R::ADC_1_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_1_DISABLED`"]
    #[inline]
    pub fn is_adc_1_disabled(&self) -> bool {
        *self == ADC_1R::ADC_1_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_2R {
    #[doc = "ADC_2 enabled on pin PIO0_14."]
    ADC_2_ENABLED_ON_PIN,
    #[doc = "ADC_2 disabled."]
    ADC_2_DISABLED,
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
            ADC_2R::ADC_2_ENABLED_ON_PIN => false,
            ADC_2R::ADC_2_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_2R {
        match value {
            false => ADC_2R::ADC_2_ENABLED_ON_PIN,
            true => ADC_2R::ADC_2_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_2_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_2_enabled_on_pin(&self) -> bool {
        *self == ADC_2R::ADC_2_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_2_DISABLED`"]
    #[inline]
    pub fn is_adc_2_disabled(&self) -> bool {
        *self == ADC_2R::ADC_2_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_3R {
    #[doc = "ADC_3 enabled on pin PIO0_23."]
    ADC_3_ENABLED_ON_PIN,
    #[doc = "ADC_3 disabled."]
    ADC_3_DISABLED,
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
            ADC_3R::ADC_3_ENABLED_ON_PIN => false,
            ADC_3R::ADC_3_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_3R {
        match value {
            false => ADC_3R::ADC_3_ENABLED_ON_PIN,
            true => ADC_3R::ADC_3_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_3_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_3_enabled_on_pin(&self) -> bool {
        *self == ADC_3R::ADC_3_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_3_DISABLED`"]
    #[inline]
    pub fn is_adc_3_disabled(&self) -> bool {
        *self == ADC_3R::ADC_3_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_4R {
    #[doc = "ADC_4 enabled on pin PIO0_22."]
    ADC_4_ENABLED_ON_PIN,
    #[doc = "ADC_4 disabled."]
    ADC_4_DISABLED,
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
            ADC_4R::ADC_4_ENABLED_ON_PIN => false,
            ADC_4R::ADC_4_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_4R {
        match value {
            false => ADC_4R::ADC_4_ENABLED_ON_PIN,
            true => ADC_4R::ADC_4_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_4_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_4_enabled_on_pin(&self) -> bool {
        *self == ADC_4R::ADC_4_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_4_DISABLED`"]
    #[inline]
    pub fn is_adc_4_disabled(&self) -> bool {
        *self == ADC_4R::ADC_4_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_5R {
    #[doc = "ADC_5 enabled on pin PIO0_21."]
    ADC_5_ENABLED_ON_PIN,
    #[doc = "ADC_5 disabled."]
    ADC_5_DISABLED,
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
            ADC_5R::ADC_5_ENABLED_ON_PIN => false,
            ADC_5R::ADC_5_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_5R {
        match value {
            false => ADC_5R::ADC_5_ENABLED_ON_PIN,
            true => ADC_5R::ADC_5_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_5_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_5_enabled_on_pin(&self) -> bool {
        *self == ADC_5R::ADC_5_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_5_DISABLED`"]
    #[inline]
    pub fn is_adc_5_disabled(&self) -> bool {
        *self == ADC_5R::ADC_5_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_6R {
    #[doc = "ADC_6 enabled on pin PIO0_20."]
    ADC_6_ENABLED_ON_PIN,
    #[doc = "ADC_6 disabled."]
    ADC_6_DISABLED,
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
            ADC_6R::ADC_6_ENABLED_ON_PIN => false,
            ADC_6R::ADC_6_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_6R {
        match value {
            false => ADC_6R::ADC_6_ENABLED_ON_PIN,
            true => ADC_6R::ADC_6_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_6_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_6_enabled_on_pin(&self) -> bool {
        *self == ADC_6R::ADC_6_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_6_DISABLED`"]
    #[inline]
    pub fn is_adc_6_disabled(&self) -> bool {
        *self == ADC_6R::ADC_6_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_7R {
    #[doc = "ADC_7 enabled on pin PIO0_19."]
    ADC_7_ENABLED_ON_PIN,
    #[doc = "ADC_7 disabled."]
    ADC_7_DISABLED,
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
            ADC_7R::ADC_7_ENABLED_ON_PIN => false,
            ADC_7R::ADC_7_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_7R {
        match value {
            false => ADC_7R::ADC_7_ENABLED_ON_PIN,
            true => ADC_7R::ADC_7_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_7_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_7_enabled_on_pin(&self) -> bool {
        *self == ADC_7R::ADC_7_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_7_DISABLED`"]
    #[inline]
    pub fn is_adc_7_disabled(&self) -> bool {
        *self == ADC_7R::ADC_7_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_8R {
    #[doc = "ADC_8 enabled on pin PIO0_18."]
    ADC_8_ENABLED_ON_PIN,
    #[doc = "ADC_8 disabled."]
    ADC_8_DISABLED,
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
            ADC_8R::ADC_8_ENABLED_ON_PIN => false,
            ADC_8R::ADC_8_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_8R {
        match value {
            false => ADC_8R::ADC_8_ENABLED_ON_PIN,
            true => ADC_8R::ADC_8_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_8_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_8_enabled_on_pin(&self) -> bool {
        *self == ADC_8R::ADC_8_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_8_DISABLED`"]
    #[inline]
    pub fn is_adc_8_disabled(&self) -> bool {
        *self == ADC_8R::ADC_8_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_9R {
    #[doc = "ADC_9 enabled on pin PIO0_17."]
    ADC_9_ENABLED_ON_PIN,
    #[doc = "ADC_9 disabled."]
    ADC_9_DISABLED,
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
            ADC_9R::ADC_9_ENABLED_ON_PIN => false,
            ADC_9R::ADC_9_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_9R {
        match value {
            false => ADC_9R::ADC_9_ENABLED_ON_PIN,
            true => ADC_9R::ADC_9_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_9_ENABLED_ON_PIN`"]
    #[inline]
    pub fn is_adc_9_enabled_on_pin(&self) -> bool {
        *self == ADC_9R::ADC_9_ENABLED_ON_PIN
    }
    #[doc = "Checks if the value of the field is `ADC_9_DISABLED`"]
    #[inline]
    pub fn is_adc_9_disabled(&self) -> bool {
        *self == ADC_9R::ADC_9_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_10R {
    #[doc = "ADC_10 enabled on pin PIO0_13."]
    ADC_10_ENABLED_ON_PI,
    #[doc = "ADC_10 disabled."]
    ADC_10_DISABLED,
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
            ADC_10R::ADC_10_ENABLED_ON_PI => false,
            ADC_10R::ADC_10_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_10R {
        match value {
            false => ADC_10R::ADC_10_ENABLED_ON_PI,
            true => ADC_10R::ADC_10_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_10_ENABLED_ON_PI`"]
    #[inline]
    pub fn is_adc_10_enabled_on_pi(&self) -> bool {
        *self == ADC_10R::ADC_10_ENABLED_ON_PI
    }
    #[doc = "Checks if the value of the field is `ADC_10_DISABLED`"]
    #[inline]
    pub fn is_adc_10_disabled(&self) -> bool {
        *self == ADC_10R::ADC_10_DISABLED
    }
}
#[doc = "Possible values of the field `ADC_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_11R {
    #[doc = "ADC_11 enabled on pin PIO0_4."]
    ADC_11_ENABLED_ON_PI,
    #[doc = "ADC_11 disabled."]
    ADC_11_DISABLED,
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
            ADC_11R::ADC_11_ENABLED_ON_PI => false,
            ADC_11R::ADC_11_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_11R {
        match value {
            false => ADC_11R::ADC_11_ENABLED_ON_PI,
            true => ADC_11R::ADC_11_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_11_ENABLED_ON_PI`"]
    #[inline]
    pub fn is_adc_11_enabled_on_pi(&self) -> bool {
        *self == ADC_11R::ADC_11_ENABLED_ON_PI
    }
    #[doc = "Checks if the value of the field is `ADC_11_DISABLED`"]
    #[inline]
    pub fn is_adc_11_disabled(&self) -> bool {
        *self == ADC_11R::ADC_11_DISABLED
    }
}
#[doc = "Values that can be written to the field `ACMP_I1`"]
pub enum ACMP_I1W {
    #[doc = "ACMP_I1 enabled on pin PIO0_00."]
    ACMP_I1_ENABLED_ON_P,
    #[doc = "ACMP_I1 disabled."]
    ACMP_I1_DISABLED,
}
impl ACMP_I1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I1W::ACMP_I1_ENABLED_ON_P => false,
            ACMP_I1W::ACMP_I1_DISABLED => true,
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
    pub fn acmp_i1_enabled_on_p(self) -> &'a mut W {
        self.variant(ACMP_I1W::ACMP_I1_ENABLED_ON_P)
    }
    #[doc = "ACMP_I1 disabled."]
    #[inline]
    pub fn acmp_i1_disabled(self) -> &'a mut W {
        self.variant(ACMP_I1W::ACMP_I1_DISABLED)
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
    ACMP_I2_ENABLED_ON_P,
    #[doc = "ACMP_I2 disabled."]
    ACMP_I2_DISABLED,
}
impl ACMP_I2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I2W::ACMP_I2_ENABLED_ON_P => false,
            ACMP_I2W::ACMP_I2_DISABLED => true,
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
    pub fn acmp_i2_enabled_on_p(self) -> &'a mut W {
        self.variant(ACMP_I2W::ACMP_I2_ENABLED_ON_P)
    }
    #[doc = "ACMP_I2 disabled."]
    #[inline]
    pub fn acmp_i2_disabled(self) -> &'a mut W {
        self.variant(ACMP_I2W::ACMP_I2_DISABLED)
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
    ACMP_I3_ENABLED_ON_P,
    #[doc = "ACMP_I3 disabled."]
    ACMP_I3_DISABLED,
}
impl ACMP_I3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I3W::ACMP_I3_ENABLED_ON_P => false,
            ACMP_I3W::ACMP_I3_DISABLED => true,
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
    pub fn acmp_i3_enabled_on_p(self) -> &'a mut W {
        self.variant(ACMP_I3W::ACMP_I3_ENABLED_ON_P)
    }
    #[doc = "ACMP_I3 disabled."]
    #[inline]
    pub fn acmp_i3_disabled(self) -> &'a mut W {
        self.variant(ACMP_I3W::ACMP_I3_DISABLED)
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
    ACMP_I4_ENABLED_ON_P,
    #[doc = "ACMP_I4 disabled."]
    ACMP_I4_DISABLED,
}
impl ACMP_I4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_I4W::ACMP_I4_ENABLED_ON_P => false,
            ACMP_I4W::ACMP_I4_DISABLED => true,
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
    pub fn acmp_i4_enabled_on_p(self) -> &'a mut W {
        self.variant(ACMP_I4W::ACMP_I4_ENABLED_ON_P)
    }
    #[doc = "ACMP_I4 disabled."]
    #[inline]
    pub fn acmp_i4_disabled(self) -> &'a mut W {
        self.variant(ACMP_I4W::ACMP_I4_DISABLED)
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
#[doc = "Values that can be written to the field `SWCLK`"]
pub enum SWCLKW {
    #[doc = "SWCLK enabled on pin PIO0_3."]
    SWCLK_ENABLED_ON_PIN,
    #[doc = "SWCLK disabled."]
    SWCLK_DISABLED,
}
impl SWCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWCLKW::SWCLK_ENABLED_ON_PIN => false,
            SWCLKW::SWCLK_DISABLED => true,
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
    pub fn swclk_enabled_on_pin(self) -> &'a mut W {
        self.variant(SWCLKW::SWCLK_ENABLED_ON_PIN)
    }
    #[doc = "SWCLK disabled."]
    #[inline]
    pub fn swclk_disabled(self) -> &'a mut W {
        self.variant(SWCLKW::SWCLK_DISABLED)
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
#[doc = "Values that can be written to the field `SWDIO`"]
pub enum SWDIOW {
    #[doc = "SWDIO enabled on pin PIO0_2."]
    SWDIO_ENABLED_ON_PIN,
    #[doc = "SWDIO disabled."]
    SWDIO_DISABLED,
}
impl SWDIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWDIOW::SWDIO_ENABLED_ON_PIN => false,
            SWDIOW::SWDIO_DISABLED => true,
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
    pub fn swdio_enabled_on_pin(self) -> &'a mut W {
        self.variant(SWDIOW::SWDIO_ENABLED_ON_PIN)
    }
    #[doc = "SWDIO disabled."]
    #[inline]
    pub fn swdio_disabled(self) -> &'a mut W {
        self.variant(SWDIOW::SWDIO_DISABLED)
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
#[doc = "Values that can be written to the field `XTALIN`"]
pub enum XTALINW {
    #[doc = "XTALIN enabled on pin PIO0_8."]
    XTALIN_ENABLED_ON_PI,
    #[doc = "XTALIN disabled."]
    XTALIN_DISABLED,
}
impl XTALINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALINW::XTALIN_ENABLED_ON_PI => false,
            XTALINW::XTALIN_DISABLED => true,
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
    pub fn xtalin_enabled_on_pi(self) -> &'a mut W {
        self.variant(XTALINW::XTALIN_ENABLED_ON_PI)
    }
    #[doc = "XTALIN disabled."]
    #[inline]
    pub fn xtalin_disabled(self) -> &'a mut W {
        self.variant(XTALINW::XTALIN_DISABLED)
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
#[doc = "Values that can be written to the field `XTALOUT`"]
pub enum XTALOUTW {
    #[doc = "XTALOUT enabled on pin PIO0_9."]
    XTALOUT_ENABLED_ON_P,
    #[doc = "XTALOUT disabled."]
    XTALOUT_DISABLED,
}
impl XTALOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALOUTW::XTALOUT_ENABLED_ON_P => false,
            XTALOUTW::XTALOUT_DISABLED => true,
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
    pub fn xtalout_enabled_on_p(self) -> &'a mut W {
        self.variant(XTALOUTW::XTALOUT_ENABLED_ON_P)
    }
    #[doc = "XTALOUT disabled."]
    #[inline]
    pub fn xtalout_disabled(self) -> &'a mut W {
        self.variant(XTALOUTW::XTALOUT_DISABLED)
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
#[doc = "Values that can be written to the field `RESETN`"]
pub enum RESETNW {
    #[doc = "RESETN enabled on pin PIO0_5."]
    RESETN_ENABLED_ON_PI,
    #[doc = "RESETN disabled."]
    RESETN_DISABLED,
}
impl RESETNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETNW::RESETN_ENABLED_ON_PI => false,
            RESETNW::RESETN_DISABLED => true,
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
    pub fn resetn_enabled_on_pi(self) -> &'a mut W {
        self.variant(RESETNW::RESETN_ENABLED_ON_PI)
    }
    #[doc = "RESETN disabled."]
    #[inline]
    pub fn resetn_disabled(self) -> &'a mut W {
        self.variant(RESETNW::RESETN_DISABLED)
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
#[doc = "Values that can be written to the field `CLKIN`"]
pub enum CLKINW {
    #[doc = "CLKIN enabled on pin PIO0_1."]
    CLKIN_ENABLED_ON_PIN,
    #[doc = "CLKIN disabled."]
    CLKIN_DISABLED,
}
impl CLKINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKINW::CLKIN_ENABLED_ON_PIN => false,
            CLKINW::CLKIN_DISABLED => true,
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
    pub fn clkin_enabled_on_pin(self) -> &'a mut W {
        self.variant(CLKINW::CLKIN_ENABLED_ON_PIN)
    }
    #[doc = "CLKIN disabled."]
    #[inline]
    pub fn clkin_disabled(self) -> &'a mut W {
        self.variant(CLKINW::CLKIN_DISABLED)
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
#[doc = "Values that can be written to the field `VDDCMP`"]
pub enum VDDCMPW {
    #[doc = "VDDCMP enabled on pin PIO0_6."]
    VDDCMP_ENABLED_ON_PI,
    #[doc = "VDDCMP disabled."]
    VDDCMP_DISABLED,
}
impl VDDCMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VDDCMPW::VDDCMP_ENABLED_ON_PI => false,
            VDDCMPW::VDDCMP_DISABLED => true,
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
    pub fn vddcmp_enabled_on_pi(self) -> &'a mut W {
        self.variant(VDDCMPW::VDDCMP_ENABLED_ON_PI)
    }
    #[doc = "VDDCMP disabled."]
    #[inline]
    pub fn vddcmp_disabled(self) -> &'a mut W {
        self.variant(VDDCMPW::VDDCMP_DISABLED)
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
#[doc = "Values that can be written to the field `I2C0_SDA`"]
pub enum I2C0_SDAW {
    #[doc = "I2C0_SDA enabled on pin PIO0_11."]
    I2C0_SDA_ENABLED_ON,
    #[doc = "I2C0_SDA disabled."]
    I2C0_SDA_DISABLED,
}
impl I2C0_SDAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0_SDAW::I2C0_SDA_ENABLED_ON => false,
            I2C0_SDAW::I2C0_SDA_DISABLED => true,
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
    pub fn i2c0_sda_enabled_on(self) -> &'a mut W {
        self.variant(I2C0_SDAW::I2C0_SDA_ENABLED_ON)
    }
    #[doc = "I2C0_SDA disabled."]
    #[inline]
    pub fn i2c0_sda_disabled(self) -> &'a mut W {
        self.variant(I2C0_SDAW::I2C0_SDA_DISABLED)
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
#[doc = "Values that can be written to the field `I2C0_SCL`"]
pub enum I2C0_SCLW {
    #[doc = "I2C0_SCL enabled on pin PIO0_10."]
    I2C0_SCL_ENABLED_ON,
    #[doc = "I2C0_SCL disabled."]
    I2C0_SCL_DISABLED,
}
impl I2C0_SCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0_SCLW::I2C0_SCL_ENABLED_ON => false,
            I2C0_SCLW::I2C0_SCL_DISABLED => true,
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
    pub fn i2c0_scl_enabled_on(self) -> &'a mut W {
        self.variant(I2C0_SCLW::I2C0_SCL_ENABLED_ON)
    }
    #[doc = "I2C0_SCL disabled."]
    #[inline]
    pub fn i2c0_scl_disabled(self) -> &'a mut W {
        self.variant(I2C0_SCLW::I2C0_SCL_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_0`"]
pub enum ADC_0W {
    #[doc = "ADC_0 enabled on pin PIO0_7."]
    ADC_0_ENABLED_ON_PIN,
    #[doc = "ADC_0 disabled."]
    ADC_0_DISABLED,
}
impl ADC_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_0W::ADC_0_ENABLED_ON_PIN => false,
            ADC_0W::ADC_0_DISABLED => true,
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
    pub fn adc_0_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_0W::ADC_0_ENABLED_ON_PIN)
    }
    #[doc = "ADC_0 disabled."]
    #[inline]
    pub fn adc_0_disabled(self) -> &'a mut W {
        self.variant(ADC_0W::ADC_0_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_1`"]
pub enum ADC_1W {
    #[doc = "ADC_1 enabled on pin PIO0_6."]
    ADC_1_ENABLED_ON_PIN,
    #[doc = "ADC_1 disabled."]
    ADC_1_DISABLED,
}
impl ADC_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_1W::ADC_1_ENABLED_ON_PIN => false,
            ADC_1W::ADC_1_DISABLED => true,
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
    pub fn adc_1_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_1W::ADC_1_ENABLED_ON_PIN)
    }
    #[doc = "ADC_1 disabled."]
    #[inline]
    pub fn adc_1_disabled(self) -> &'a mut W {
        self.variant(ADC_1W::ADC_1_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_2`"]
pub enum ADC_2W {
    #[doc = "ADC_2 enabled on pin PIO0_14."]
    ADC_2_ENABLED_ON_PIN,
    #[doc = "ADC_2 disabled."]
    ADC_2_DISABLED,
}
impl ADC_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_2W::ADC_2_ENABLED_ON_PIN => false,
            ADC_2W::ADC_2_DISABLED => true,
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
    pub fn adc_2_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_2W::ADC_2_ENABLED_ON_PIN)
    }
    #[doc = "ADC_2 disabled."]
    #[inline]
    pub fn adc_2_disabled(self) -> &'a mut W {
        self.variant(ADC_2W::ADC_2_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_3`"]
pub enum ADC_3W {
    #[doc = "ADC_3 enabled on pin PIO0_23."]
    ADC_3_ENABLED_ON_PIN,
    #[doc = "ADC_3 disabled."]
    ADC_3_DISABLED,
}
impl ADC_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_3W::ADC_3_ENABLED_ON_PIN => false,
            ADC_3W::ADC_3_DISABLED => true,
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
    pub fn adc_3_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_3W::ADC_3_ENABLED_ON_PIN)
    }
    #[doc = "ADC_3 disabled."]
    #[inline]
    pub fn adc_3_disabled(self) -> &'a mut W {
        self.variant(ADC_3W::ADC_3_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_4`"]
pub enum ADC_4W {
    #[doc = "ADC_4 enabled on pin PIO0_22."]
    ADC_4_ENABLED_ON_PIN,
    #[doc = "ADC_4 disabled."]
    ADC_4_DISABLED,
}
impl ADC_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_4W::ADC_4_ENABLED_ON_PIN => false,
            ADC_4W::ADC_4_DISABLED => true,
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
    pub fn adc_4_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_4W::ADC_4_ENABLED_ON_PIN)
    }
    #[doc = "ADC_4 disabled."]
    #[inline]
    pub fn adc_4_disabled(self) -> &'a mut W {
        self.variant(ADC_4W::ADC_4_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_5`"]
pub enum ADC_5W {
    #[doc = "ADC_5 enabled on pin PIO0_21."]
    ADC_5_ENABLED_ON_PIN,
    #[doc = "ADC_5 disabled."]
    ADC_5_DISABLED,
}
impl ADC_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_5W::ADC_5_ENABLED_ON_PIN => false,
            ADC_5W::ADC_5_DISABLED => true,
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
    pub fn adc_5_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_5W::ADC_5_ENABLED_ON_PIN)
    }
    #[doc = "ADC_5 disabled."]
    #[inline]
    pub fn adc_5_disabled(self) -> &'a mut W {
        self.variant(ADC_5W::ADC_5_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_6`"]
pub enum ADC_6W {
    #[doc = "ADC_6 enabled on pin PIO0_20."]
    ADC_6_ENABLED_ON_PIN,
    #[doc = "ADC_6 disabled."]
    ADC_6_DISABLED,
}
impl ADC_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_6W::ADC_6_ENABLED_ON_PIN => false,
            ADC_6W::ADC_6_DISABLED => true,
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
    pub fn adc_6_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_6W::ADC_6_ENABLED_ON_PIN)
    }
    #[doc = "ADC_6 disabled."]
    #[inline]
    pub fn adc_6_disabled(self) -> &'a mut W {
        self.variant(ADC_6W::ADC_6_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_7`"]
pub enum ADC_7W {
    #[doc = "ADC_7 enabled on pin PIO0_19."]
    ADC_7_ENABLED_ON_PIN,
    #[doc = "ADC_7 disabled."]
    ADC_7_DISABLED,
}
impl ADC_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_7W::ADC_7_ENABLED_ON_PIN => false,
            ADC_7W::ADC_7_DISABLED => true,
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
    pub fn adc_7_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_7W::ADC_7_ENABLED_ON_PIN)
    }
    #[doc = "ADC_7 disabled."]
    #[inline]
    pub fn adc_7_disabled(self) -> &'a mut W {
        self.variant(ADC_7W::ADC_7_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_8`"]
pub enum ADC_8W {
    #[doc = "ADC_8 enabled on pin PIO0_18."]
    ADC_8_ENABLED_ON_PIN,
    #[doc = "ADC_8 disabled."]
    ADC_8_DISABLED,
}
impl ADC_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_8W::ADC_8_ENABLED_ON_PIN => false,
            ADC_8W::ADC_8_DISABLED => true,
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
    pub fn adc_8_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_8W::ADC_8_ENABLED_ON_PIN)
    }
    #[doc = "ADC_8 disabled."]
    #[inline]
    pub fn adc_8_disabled(self) -> &'a mut W {
        self.variant(ADC_8W::ADC_8_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_9`"]
pub enum ADC_9W {
    #[doc = "ADC_9 enabled on pin PIO0_17."]
    ADC_9_ENABLED_ON_PIN,
    #[doc = "ADC_9 disabled."]
    ADC_9_DISABLED,
}
impl ADC_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_9W::ADC_9_ENABLED_ON_PIN => false,
            ADC_9W::ADC_9_DISABLED => true,
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
    pub fn adc_9_enabled_on_pin(self) -> &'a mut W {
        self.variant(ADC_9W::ADC_9_ENABLED_ON_PIN)
    }
    #[doc = "ADC_9 disabled."]
    #[inline]
    pub fn adc_9_disabled(self) -> &'a mut W {
        self.variant(ADC_9W::ADC_9_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_10`"]
pub enum ADC_10W {
    #[doc = "ADC_10 enabled on pin PIO0_13."]
    ADC_10_ENABLED_ON_PI,
    #[doc = "ADC_10 disabled."]
    ADC_10_DISABLED,
}
impl ADC_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_10W::ADC_10_ENABLED_ON_PI => false,
            ADC_10W::ADC_10_DISABLED => true,
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
    pub fn adc_10_enabled_on_pi(self) -> &'a mut W {
        self.variant(ADC_10W::ADC_10_ENABLED_ON_PI)
    }
    #[doc = "ADC_10 disabled."]
    #[inline]
    pub fn adc_10_disabled(self) -> &'a mut W {
        self.variant(ADC_10W::ADC_10_DISABLED)
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
#[doc = "Values that can be written to the field `ADC_11`"]
pub enum ADC_11W {
    #[doc = "ADC_11 enabled on pin PIO0_4."]
    ADC_11_ENABLED_ON_PI,
    #[doc = "ADC_11 disabled."]
    ADC_11_DISABLED,
}
impl ADC_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_11W::ADC_11_ENABLED_ON_PI => false,
            ADC_11W::ADC_11_DISABLED => true,
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
    pub fn adc_11_enabled_on_pi(self) -> &'a mut W {
        self.variant(ADC_11W::ADC_11_ENABLED_ON_PI)
    }
    #[doc = "ADC_11 disabled."]
    #[inline]
    pub fn adc_11_disabled(self) -> &'a mut W {
        self.variant(ADC_11W::ADC_11_DISABLED)
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
    #[doc = "Bit 4 - SWCLK function select."]
    #[inline]
    pub fn swclk(&self) -> SWCLKR {
        SWCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - SWDIO function select."]
    #[inline]
    pub fn swdio(&self) -> SWDIOR {
        SWDIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - XTALIN function select."]
    #[inline]
    pub fn xtalin(&self) -> XTALINR {
        XTALINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - XTALOUT function select."]
    #[inline]
    pub fn xtalout(&self) -> XTALOUTR {
        XTALOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - RESETN function select."]
    #[inline]
    pub fn resetn(&self) -> RESETNR {
        RESETNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CLKIN function select."]
    #[inline]
    pub fn clkin(&self) -> CLKINR {
        CLKINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - VDDCMP function select."]
    #[inline]
    pub fn vddcmp(&self) -> VDDCMPR {
        VDDCMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - I2C0_SDA function select."]
    #[inline]
    pub fn i2c0_sda(&self) -> I2C0_SDAR {
        I2C0_SDAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - I2C0_SCL function select."]
    #[inline]
    pub fn i2c0_scl(&self) -> I2C0_SCLR {
        I2C0_SCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - ADC_0 function select."]
    #[inline]
    pub fn adc_0(&self) -> ADC_0R {
        ADC_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - ADC_1 function select."]
    #[inline]
    pub fn adc_1(&self) -> ADC_1R {
        ADC_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - ADC_2 function select."]
    #[inline]
    pub fn adc_2(&self) -> ADC_2R {
        ADC_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - ADC_3 function select."]
    #[inline]
    pub fn adc_3(&self) -> ADC_3R {
        ADC_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - ADC_4 function select."]
    #[inline]
    pub fn adc_4(&self) -> ADC_4R {
        ADC_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - ADC_5 function select."]
    #[inline]
    pub fn adc_5(&self) -> ADC_5R {
        ADC_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - ADC_6 function select."]
    #[inline]
    pub fn adc_6(&self) -> ADC_6R {
        ADC_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - ADC_7 function select."]
    #[inline]
    pub fn adc_7(&self) -> ADC_7R {
        ADC_7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - ADC_8 function select."]
    #[inline]
    pub fn adc_8(&self) -> ADC_8R {
        ADC_8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - ADC_9 function select."]
    #[inline]
    pub fn adc_9(&self) -> ADC_9R {
        ADC_9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - ADC_10 function select."]
    #[inline]
    pub fn adc_10(&self) -> ADC_10R {
        ADC_10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - ADC_11 function select."]
    #[inline]
    pub fn adc_11(&self) -> ADC_11R {
        ADC_11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294966991 }
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
    #[doc = "Bit 4 - SWCLK function select."]
    #[inline]
    pub fn swclk(&mut self) -> _SWCLKW {
        _SWCLKW { w: self }
    }
    #[doc = "Bit 5 - SWDIO function select."]
    #[inline]
    pub fn swdio(&mut self) -> _SWDIOW {
        _SWDIOW { w: self }
    }
    #[doc = "Bit 6 - XTALIN function select."]
    #[inline]
    pub fn xtalin(&mut self) -> _XTALINW {
        _XTALINW { w: self }
    }
    #[doc = "Bit 7 - XTALOUT function select."]
    #[inline]
    pub fn xtalout(&mut self) -> _XTALOUTW {
        _XTALOUTW { w: self }
    }
    #[doc = "Bit 8 - RESETN function select."]
    #[inline]
    pub fn resetn(&mut self) -> _RESETNW {
        _RESETNW { w: self }
    }
    #[doc = "Bit 9 - CLKIN function select."]
    #[inline]
    pub fn clkin(&mut self) -> _CLKINW {
        _CLKINW { w: self }
    }
    #[doc = "Bit 10 - VDDCMP function select."]
    #[inline]
    pub fn vddcmp(&mut self) -> _VDDCMPW {
        _VDDCMPW { w: self }
    }
    #[doc = "Bit 11 - I2C0_SDA function select."]
    #[inline]
    pub fn i2c0_sda(&mut self) -> _I2C0_SDAW {
        _I2C0_SDAW { w: self }
    }
    #[doc = "Bit 12 - I2C0_SCL function select."]
    #[inline]
    pub fn i2c0_scl(&mut self) -> _I2C0_SCLW {
        _I2C0_SCLW { w: self }
    }
    #[doc = "Bit 13 - ADC_0 function select."]
    #[inline]
    pub fn adc_0(&mut self) -> _ADC_0W {
        _ADC_0W { w: self }
    }
    #[doc = "Bit 14 - ADC_1 function select."]
    #[inline]
    pub fn adc_1(&mut self) -> _ADC_1W {
        _ADC_1W { w: self }
    }
    #[doc = "Bit 15 - ADC_2 function select."]
    #[inline]
    pub fn adc_2(&mut self) -> _ADC_2W {
        _ADC_2W { w: self }
    }
    #[doc = "Bit 16 - ADC_3 function select."]
    #[inline]
    pub fn adc_3(&mut self) -> _ADC_3W {
        _ADC_3W { w: self }
    }
    #[doc = "Bit 17 - ADC_4 function select."]
    #[inline]
    pub fn adc_4(&mut self) -> _ADC_4W {
        _ADC_4W { w: self }
    }
    #[doc = "Bit 18 - ADC_5 function select."]
    #[inline]
    pub fn adc_5(&mut self) -> _ADC_5W {
        _ADC_5W { w: self }
    }
    #[doc = "Bit 19 - ADC_6 function select."]
    #[inline]
    pub fn adc_6(&mut self) -> _ADC_6W {
        _ADC_6W { w: self }
    }
    #[doc = "Bit 20 - ADC_7 function select."]
    #[inline]
    pub fn adc_7(&mut self) -> _ADC_7W {
        _ADC_7W { w: self }
    }
    #[doc = "Bit 21 - ADC_8 function select."]
    #[inline]
    pub fn adc_8(&mut self) -> _ADC_8W {
        _ADC_8W { w: self }
    }
    #[doc = "Bit 22 - ADC_9 function select."]
    #[inline]
    pub fn adc_9(&mut self) -> _ADC_9W {
        _ADC_9W { w: self }
    }
    #[doc = "Bit 23 - ADC_10 function select."]
    #[inline]
    pub fn adc_10(&mut self) -> _ADC_10W {
        _ADC_10W { w: self }
    }
    #[doc = "Bit 24 - ADC_11 function select."]
    #[inline]
    pub fn adc_11(&mut self) -> _ADC_11W {
        _ADC_11W { w: self }
    }
}
