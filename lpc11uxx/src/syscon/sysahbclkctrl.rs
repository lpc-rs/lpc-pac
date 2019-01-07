#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSAHBCLKCTRL {
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
#[doc = "Possible values of the field `SYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSR {
    #[doc = "Enable"]
    ENABLED,
}
impl SYSR {
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
            SYSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSR {
        match value {
            true => SYSR::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SYSR::ENABLED
    }
}
#[doc = "Possible values of the field `ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl ROMR {
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
            ROMR::DISABLED => false,
            ROMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROMR {
        match value {
            false => ROMR::DISABLED,
            true => ROMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ROMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ROMR::ENABLED
    }
}
#[doc = "Possible values of the field `RAM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RAM0R {
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
            RAM0R::DISABLED => false,
            RAM0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM0R {
        match value {
            false => RAM0R::DISABLED,
            true => RAM0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RAM0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RAM0R::ENABLED
    }
}
#[doc = "Possible values of the field `FLASHREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREGR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl FLASHREGR {
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
            FLASHREGR::DISABLED => false,
            FLASHREGR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHREGR {
        match value {
            false => FLASHREGR::DISABLED,
            true => FLASHREGR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHREGR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHREGR::ENABLED
    }
}
#[doc = "Possible values of the field `FLASHARRAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHARRAYR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl FLASHARRAYR {
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
            FLASHARRAYR::DISABLED => false,
            FLASHARRAYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHARRAYR {
        match value {
            false => FLASHARRAYR::DISABLED,
            true => FLASHARRAYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHARRAYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHARRAYR::ENABLED
    }
}
#[doc = "Possible values of the field `I2C`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl I2CR {
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
            I2CR::DISABLED => false,
            I2CR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2CR {
        match value {
            false => I2CR::DISABLED,
            true => I2CR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == I2CR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == I2CR::ENABLED
    }
}
#[doc = "Possible values of the field `GPIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl GPIOR {
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
            GPIOR::DISABLED => false,
            GPIOR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIOR {
        match value {
            false => GPIOR::DISABLED,
            true => GPIOR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOR::ENABLED
    }
}
#[doc = "Possible values of the field `CT16B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT16B0R {
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
            CT16B0R::DISABLED => false,
            CT16B0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT16B0R {
        match value {
            false => CT16B0R::DISABLED,
            true => CT16B0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CT16B0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CT16B0R::ENABLED
    }
}
#[doc = "Possible values of the field `CT16B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT16B1R {
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
            CT16B1R::DISABLED => false,
            CT16B1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT16B1R {
        match value {
            false => CT16B1R::DISABLED,
            true => CT16B1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CT16B1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CT16B1R::ENABLED
    }
}
#[doc = "Possible values of the field `CT32B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT32B0R {
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
            CT32B0R::DISABLED => false,
            CT32B0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT32B0R {
        match value {
            false => CT32B0R::DISABLED,
            true => CT32B0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CT32B0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CT32B0R::ENABLED
    }
}
#[doc = "Possible values of the field `CT32B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B1R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT32B1R {
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
            CT32B1R::DISABLED => false,
            CT32B1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CT32B1R {
        match value {
            false => CT32B1R::DISABLED,
            true => CT32B1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CT32B1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CT32B1R::ENABLED
    }
}
#[doc = "Possible values of the field `SSP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SSP0R {
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
            SSP0R::DISABLED => false,
            SSP0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP0R {
        match value {
            false => SSP0R::DISABLED,
            true => SSP0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSP0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSP0R::ENABLED
    }
}
#[doc = "Possible values of the field `USART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USARTR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl USARTR {
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
            USARTR::DISABLED => false,
            USARTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USARTR {
        match value {
            false => USARTR::DISABLED,
            true => USARTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == USARTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == USARTR::ENABLED
    }
}
#[doc = "Possible values of the field `ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl ADCR {
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
            ADCR::DISABLED => false,
            ADCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCR {
        match value {
            false => ADCR::DISABLED,
            true => ADCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADCR::ENABLED
    }
}
#[doc = "Possible values of the field `USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl USBR {
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
            USBR::DISABLED => false,
            USBR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBR {
        match value {
            false => USBR::DISABLED,
            true => USBR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == USBR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == USBR::ENABLED
    }
}
#[doc = "Possible values of the field `WWDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl WWDTR {
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
            WWDTR::DISABLED => false,
            WWDTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDTR {
        match value {
            false => WWDTR::DISABLED,
            true => WWDTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WWDTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WWDTR::ENABLED
    }
}
#[doc = "Possible values of the field `IOCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCONR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl IOCONR {
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
            IOCONR::DISABLED => false,
            IOCONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOCONR {
        match value {
            false => IOCONR::DISABLED,
            true => IOCONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == IOCONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == IOCONR::ENABLED
    }
}
#[doc = "Possible values of the field `SSP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SSP1R {
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
            SSP1R::DISABLED => false,
            SSP1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP1R {
        match value {
            false => SSP1R::DISABLED,
            true => SSP1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSP1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSP1R::ENABLED
    }
}
#[doc = "Possible values of the field `PINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINTR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl PINTR {
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
            PINTR::DISABLED => false,
            PINTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINTR {
        match value {
            false => PINTR::DISABLED,
            true => PINTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PINTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PINTR::ENABLED
    }
}
#[doc = "Possible values of the field `GROUP0INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP0INTR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl GROUP0INTR {
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
            GROUP0INTR::DISABLED => false,
            GROUP0INTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GROUP0INTR {
        match value {
            false => GROUP0INTR::DISABLED,
            true => GROUP0INTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == GROUP0INTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == GROUP0INTR::ENABLED
    }
}
#[doc = "Possible values of the field `GROUP1INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1INTR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl GROUP1INTR {
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
            GROUP1INTR::DISABLED => false,
            GROUP1INTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GROUP1INTR {
        match value {
            false => GROUP1INTR::DISABLED,
            true => GROUP1INTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == GROUP1INTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == GROUP1INTR::ENABLED
    }
}
#[doc = "Possible values of the field `RAM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RAM1R {
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
            RAM1R::DISABLED => false,
            RAM1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM1R {
        match value {
            false => RAM1R::DISABLED,
            true => RAM1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RAM1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RAM1R::ENABLED
    }
}
#[doc = "Possible values of the field `USBRAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRAMR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl USBRAMR {
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
            USBRAMR::DISABLED => false,
            USBRAMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBRAMR {
        match value {
            false => USBRAMR::DISABLED,
            true => USBRAMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == USBRAMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == USBRAMR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SYS`"]
pub enum SYSW {
    #[doc = "Enable"]
    ENABLED,
}
impl SYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSW::ENABLED)
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
#[doc = "Values that can be written to the field `ROM`"]
pub enum ROMW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl ROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROMW::DISABLED => false,
            ROMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROMW<'a> {
    w: &'a mut W,
}
impl<'a> _ROMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROMW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROMW::ENABLED)
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
#[doc = "Values that can be written to the field `RAM0`"]
pub enum RAM0W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RAM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM0W::DISABLED => false,
            RAM0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAM0W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAM0W::ENABLED)
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
#[doc = "Values that can be written to the field `FLASHREG`"]
pub enum FLASHREGW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl FLASHREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHREGW::DISABLED => false,
            FLASHREGW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHREGW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHREGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHREGW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHREGW::ENABLED)
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
#[doc = "Values that can be written to the field `FLASHARRAY`"]
pub enum FLASHARRAYW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl FLASHARRAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHARRAYW::DISABLED => false,
            FLASHARRAYW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHARRAYW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHARRAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHARRAYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHARRAYW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHARRAYW::ENABLED)
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
#[doc = "Values that can be written to the field `I2C`"]
pub enum I2CW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl I2CW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2CW::DISABLED => false,
            I2CW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2CW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2CW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2CW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIO`"]
pub enum GPIOW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl GPIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOW::DISABLED => false,
            GPIOW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOW::ENABLED)
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
#[doc = "Values that can be written to the field `CT16B0`"]
pub enum CT16B0W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT16B0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT16B0W::DISABLED => false,
            CT16B0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT16B0W<'a> {
    w: &'a mut W,
}
impl<'a> _CT16B0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT16B0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B0W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B0W::ENABLED)
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
#[doc = "Values that can be written to the field `CT16B1`"]
pub enum CT16B1W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT16B1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT16B1W::DISABLED => false,
            CT16B1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT16B1W<'a> {
    w: &'a mut W,
}
impl<'a> _CT16B1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT16B1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B1W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B1W::ENABLED)
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
#[doc = "Values that can be written to the field `CT32B0`"]
pub enum CT32B0W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT32B0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT32B0W::DISABLED => false,
            CT32B0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT32B0W<'a> {
    w: &'a mut W,
}
impl<'a> _CT32B0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT32B0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B0W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B0W::ENABLED)
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
#[doc = "Values that can be written to the field `CT32B1`"]
pub enum CT32B1W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl CT32B1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CT32B1W::DISABLED => false,
            CT32B1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CT32B1W<'a> {
    w: &'a mut W,
}
impl<'a> _CT32B1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CT32B1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B1W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B1W::ENABLED)
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
#[doc = "Values that can be written to the field `SSP0`"]
pub enum SSP0W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SSP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP0W::DISABLED => false,
            SSP0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP0W<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSP0W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSP0W::ENABLED)
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
#[doc = "Values that can be written to the field `USART`"]
pub enum USARTW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl USARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USARTW::DISABLED => false,
            USARTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USARTW<'a> {
    w: &'a mut W,
}
impl<'a> _USARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USARTW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USARTW::ENABLED)
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
#[doc = "Values that can be written to the field `ADC`"]
pub enum ADCW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl ADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCW::DISABLED => false,
            ADCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCW::ENABLED)
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
#[doc = "Values that can be written to the field `USB`"]
pub enum USBW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBW::DISABLED => false,
            USBW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBW<'a> {
    w: &'a mut W,
}
impl<'a> _USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBW::ENABLED)
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
#[doc = "Values that can be written to the field `WWDT`"]
pub enum WWDTW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl WWDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDTW::DISABLED => false,
            WWDTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WWDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDTW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDTW::ENABLED)
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
#[doc = "Values that can be written to the field `IOCON`"]
pub enum IOCONW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl IOCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCONW::DISABLED => false,
            IOCONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCONW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOCONW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOCONW::ENABLED)
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
#[doc = "Values that can be written to the field `SSP1`"]
pub enum SSP1W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SSP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP1W::DISABLED => false,
            SSP1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSP1W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSP1W::ENABLED)
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
#[doc = "Values that can be written to the field `PINT`"]
pub enum PINTW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl PINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINTW::DISABLED => false,
            PINTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINTW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINTW::ENABLED)
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
#[doc = "Values that can be written to the field `GROUP0INT`"]
pub enum GROUP0INTW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl GROUP0INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GROUP0INTW::DISABLED => false,
            GROUP0INTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GROUP0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP0INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GROUP0INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP0INTW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP0INTW::ENABLED)
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
#[doc = "Values that can be written to the field `GROUP1INT`"]
pub enum GROUP1INTW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl GROUP1INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GROUP1INTW::DISABLED => false,
            GROUP1INTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GROUP1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP1INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GROUP1INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP1INTW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP1INTW::ENABLED)
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
#[doc = "Values that can be written to the field `RAM1`"]
pub enum RAM1W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RAM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM1W::DISABLED => false,
            RAM1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAM1W::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAM1W::ENABLED)
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
#[doc = "Values that can be written to the field `USBRAM`"]
pub enum USBRAMW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl USBRAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBRAMW::DISABLED => false,
            USBRAMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBRAMW<'a> {
    w: &'a mut W,
}
impl<'a> _USBRAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBRAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBRAMW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBRAMW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
    #[inline]
    pub fn sys(&self) -> SYSR {
        SYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline]
    pub fn rom(&self) -> ROMR {
        ROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline]
    pub fn ram0(&self) -> RAM0R {
        RAM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline]
    pub fn flashreg(&self) -> FLASHREGR {
        FLASHREGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline]
    pub fn flasharray(&self) -> FLASHARRAYR {
        FLASHARRAYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline]
    pub fn i2c(&self) -> I2CR {
        I2CR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline]
    pub fn gpio(&self) -> GPIOR {
        GPIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline]
    pub fn ct16b0(&self) -> CT16B0R {
        CT16B0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline]
    pub fn ct16b1(&self) -> CT16B1R {
        CT16B1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline]
    pub fn ct32b0(&self) -> CT32B0R {
        CT32B0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline]
    pub fn ct32b1(&self) -> CT32B1R {
        CT32B1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline]
    pub fn ssp0(&self) -> SSP0R {
        SSP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enables clock for UART."]
    #[inline]
    pub fn usart(&self) -> USARTR {
        USARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline]
    pub fn adc(&self) -> ADCR {
        ADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline]
    pub fn usb(&self) -> USBR {
        USBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline]
    pub fn wwdt(&self) -> WWDTR {
        WWDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline]
    pub fn iocon(&self) -> IOCONR {
        IOCONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline]
    pub fn ssp1(&self) -> SSP1R {
        SSP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupts register interface."]
    #[inline]
    pub fn pint(&self) -> PINTR {
        PINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline]
    pub fn group0int(&self) -> GROUP0INTR {
        GROUP0INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline]
    pub fn group1int(&self) -> GROUP1INTR {
        GROUP1INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
    #[inline]
    pub fn ram1(&self) -> RAM1R {
        RAM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enables USB SRAM block at address 0x2000 4000."]
    #[inline]
    pub fn usbram(&self) -> USBRAMR {
        USBRAMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 63 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
    #[inline]
    pub fn sys(&mut self) -> _SYSW {
        _SYSW { w: self }
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline]
    pub fn rom(&mut self) -> _ROMW {
        _ROMW { w: self }
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline]
    pub fn ram0(&mut self) -> _RAM0W {
        _RAM0W { w: self }
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline]
    pub fn flashreg(&mut self) -> _FLASHREGW {
        _FLASHREGW { w: self }
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline]
    pub fn flasharray(&mut self) -> _FLASHARRAYW {
        _FLASHARRAYW { w: self }
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline]
    pub fn i2c(&mut self) -> _I2CW {
        _I2CW { w: self }
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline]
    pub fn gpio(&mut self) -> _GPIOW {
        _GPIOW { w: self }
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline]
    pub fn ct16b0(&mut self) -> _CT16B0W {
        _CT16B0W { w: self }
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline]
    pub fn ct16b1(&mut self) -> _CT16B1W {
        _CT16B1W { w: self }
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline]
    pub fn ct32b0(&mut self) -> _CT32B0W {
        _CT32B0W { w: self }
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline]
    pub fn ct32b1(&mut self) -> _CT32B1W {
        _CT32B1W { w: self }
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline]
    pub fn ssp0(&mut self) -> _SSP0W {
        _SSP0W { w: self }
    }
    #[doc = "Bit 12 - Enables clock for UART."]
    #[inline]
    pub fn usart(&mut self) -> _USARTW {
        _USARTW { w: self }
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline]
    pub fn adc(&mut self) -> _ADCW {
        _ADCW { w: self }
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline]
    pub fn usb(&mut self) -> _USBW {
        _USBW { w: self }
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline]
    pub fn wwdt(&mut self) -> _WWDTW {
        _WWDTW { w: self }
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline]
    pub fn iocon(&mut self) -> _IOCONW {
        _IOCONW { w: self }
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline]
    pub fn ssp1(&mut self) -> _SSP1W {
        _SSP1W { w: self }
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupts register interface."]
    #[inline]
    pub fn pint(&mut self) -> _PINTW {
        _PINTW { w: self }
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline]
    pub fn group0int(&mut self) -> _GROUP0INTW {
        _GROUP0INTW { w: self }
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline]
    pub fn group1int(&mut self) -> _GROUP1INTW {
        _GROUP1INTW { w: self }
    }
    #[doc = "Bit 26 - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
    #[inline]
    pub fn ram1(&mut self) -> _RAM1W {
        _RAM1W { w: self }
    }
    #[doc = "Bit 27 - Enables USB SRAM block at address 0x2000 4000."]
    #[inline]
    pub fn usbram(&mut self) -> _USBRAMW {
        _USBRAMW { w: self }
    }
}
