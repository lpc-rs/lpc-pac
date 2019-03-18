#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSAHBCLKCTRL0 {
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
#[doc = r" Value of the field"]
pub struct SYSR {
    bits: bool,
}
impl SYSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
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
            ROMR::DISABLE => false,
            ROMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROMR {
        match value {
            false => ROMR::DISABLE,
            true => ROMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ROMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ROMR::ENABLE
    }
}
#[doc = "Possible values of the field `RAM0_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_1R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl RAM0_1R {
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
            RAM0_1R::DISABLE => false,
            RAM0_1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM0_1R {
        match value {
            false => RAM0_1R::DISABLE,
            true => RAM0_1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RAM0_1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RAM0_1R::ENABLE
    }
}
#[doc = "Possible values of the field `FLASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl FLASHR {
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
            FLASHR::DISABLE => false,
            FLASHR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHR {
        match value {
            false => FLASHR::DISABLE,
            true => FLASHR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLASHR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLASHR::ENABLE
    }
}
#[doc = "Possible values of the field `I2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C0R {
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
            I2C0R::DISABLE => false,
            I2C0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C0R {
        match value {
            false => I2C0R::DISABLE,
            true => I2C0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == I2C0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == I2C0R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl GPIO0R {
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
            GPIO0R::DISABLE => false,
            GPIO0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO0R {
        match value {
            false => GPIO0R::DISABLE,
            true => GPIO0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO0R::ENABLE
    }
}
#[doc = "Possible values of the field `SWM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWMR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SWMR {
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
            SWMR::DISABLE => false,
            SWMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWMR {
        match value {
            false => SWMR::DISABLE,
            true => SWMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SWMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SWMR::ENABLE
    }
}
#[doc = "Possible values of the field `SCT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SCTR {
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
            SCTR::DISABLE => false,
            SCTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCTR {
        match value {
            false => SCTR::DISABLE,
            true => SCTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SCTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SCTR::ENABLE
    }
}
#[doc = "Possible values of the field `WKT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKTR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl WKTR {
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
            WKTR::DISABLE => false,
            WKTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKTR {
        match value {
            false => WKTR::DISABLE,
            true => WKTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKTR::ENABLE
    }
}
#[doc = "Possible values of the field `MRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRTR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl MRTR {
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
            MRTR::DISABLE => false,
            MRTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRTR {
        match value {
            false => MRTR::DISABLE,
            true => MRTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MRTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MRTR::ENABLE
    }
}
#[doc = "Possible values of the field `SPI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SPI0R {
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
            SPI0R::DISABLE => false,
            SPI0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI0R {
        match value {
            false => SPI0R::DISABLE,
            true => SPI0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SPI0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SPI0R::ENABLE
    }
}
#[doc = "Possible values of the field `SPI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SPI1R {
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
            SPI1R::DISABLE => false,
            SPI1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI1R {
        match value {
            false => SPI1R::DISABLE,
            true => SPI1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SPI1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SPI1R::ENABLE
    }
}
#[doc = "Possible values of the field `CRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl CRCR {
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
            CRCR::DISABLE => false,
            CRCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCR {
        match value {
            false => CRCR::DISABLE,
            true => CRCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CRCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CRCR::ENABLE
    }
}
#[doc = "Possible values of the field `UART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART0R {
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
            UART0R::DISABLE => false,
            UART0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART0R {
        match value {
            false => UART0R::DISABLE,
            true => UART0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UART0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UART0R::ENABLE
    }
}
#[doc = "Possible values of the field `UART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART1R {
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
            UART1R::DISABLE => false,
            UART1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART1R {
        match value {
            false => UART1R::DISABLE,
            true => UART1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UART1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UART1R::ENABLE
    }
}
#[doc = "Possible values of the field `UART2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART2R {
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
            UART2R::DISABLE => false,
            UART2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART2R {
        match value {
            false => UART2R::DISABLE,
            true => UART2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UART2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UART2R::ENABLE
    }
}
#[doc = "Possible values of the field `WWDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
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
            WWDTR::DISABLE => false,
            WWDTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDTR {
        match value {
            false => WWDTR::DISABLE,
            true => WWDTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WWDTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WWDTR::ENABLE
    }
}
#[doc = "Possible values of the field `IOCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCONR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
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
            IOCONR::DISABLE => false,
            IOCONR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOCONR {
        match value {
            false => IOCONR::DISABLE,
            true => IOCONR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IOCONR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IOCONR::ENABLE
    }
}
#[doc = "Possible values of the field `ACMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMPR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl ACMPR {
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
            ACMPR::DISABLE => false,
            ACMPR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMPR {
        match value {
            false => ACMPR::DISABLE,
            true => ACMPR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ACMPR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ACMPR::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl GPIO1R {
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
            GPIO1R::DISABLE => false,
            GPIO1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO1R {
        match value {
            false => GPIO1R::DISABLE,
            true => GPIO1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO1R::ENABLE
    }
}
#[doc = "Possible values of the field `I2C1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C1R {
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
            I2C1R::DISABLE => false,
            I2C1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C1R {
        match value {
            false => I2C1R::DISABLE,
            true => I2C1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == I2C1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == I2C1R::ENABLE
    }
}
#[doc = "Possible values of the field `I2C2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C2R {
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
            I2C2R::DISABLE => false,
            I2C2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C2R {
        match value {
            false => I2C2R::DISABLE,
            true => I2C2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == I2C2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == I2C2R::ENABLE
    }
}
#[doc = "Possible values of the field `I2C3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C3R {
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
            I2C3R::DISABLE => false,
            I2C3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C3R {
        match value {
            false => I2C3R::DISABLE,
            true => I2C3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == I2C3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == I2C3R::ENABLE
    }
}
#[doc = "Possible values of the field `ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
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
            ADCR::DISABLE => false,
            ADCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCR {
        match value {
            false => ADCR::DISABLE,
            true => ADCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADCR::ENABLE
    }
}
#[doc = "Possible values of the field `CTIMER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMERR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl CTIMERR {
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
            CTIMERR::DISABLE => false,
            CTIMERR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMERR {
        match value {
            false => CTIMERR::DISABLE,
            true => CTIMERR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CTIMERR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CTIMERR::ENABLE
    }
}
#[doc = "Possible values of the field `MTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTBR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl MTBR {
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
            MTBR::DISABLE => false,
            MTBR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTBR {
        match value {
            false => MTBR::DISABLE,
            true => MTBR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MTBR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MTBR::ENABLE
    }
}
#[doc = "Possible values of the field `DAC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl DAC0R {
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
            DAC0R::DISABLE => false,
            DAC0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAC0R {
        match value {
            false => DAC0R::DISABLE,
            true => DAC0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DAC0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DAC0R::ENABLE
    }
}
#[doc = "Possible values of the field `GPIO_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INTR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl GPIO_INTR {
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
            GPIO_INTR::DISABLE => false,
            GPIO_INTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INTR {
        match value {
            false => GPIO_INTR::DISABLE,
            true => GPIO_INTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INTR::ENABLE
    }
}
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl DMAR {
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
            DMAR::DISABLE => false,
            DMAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAR {
        match value {
            false => DMAR::DISABLE,
            true => DMAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DMAR::ENABLE
    }
}
#[doc = "Possible values of the field `UART3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART3R {
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
            UART3R::DISABLE => false,
            UART3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART3R {
        match value {
            false => UART3R::DISABLE,
            true => UART3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UART3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UART3R::ENABLE
    }
}
#[doc = "Possible values of the field `UART4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART4R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART4R {
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
            UART4R::DISABLE => false,
            UART4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART4R {
        match value {
            false => UART4R::DISABLE,
            true => UART4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == UART4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == UART4R::ENABLE
    }
}
#[doc = r" Proxy"]
pub struct _SYSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSW<'a> {
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
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl ROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROMW::DISABLE => false,
            ROMW::ENABLE => true,
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
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROMW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROMW::ENABLE)
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
#[doc = "Values that can be written to the field `RAM0_1`"]
pub enum RAM0_1W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl RAM0_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM0_1W::DISABLE => false,
            RAM0_1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM0_1W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM0_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM0_1W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM0_1W::ENABLE)
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
#[doc = "Values that can be written to the field `FLASH`"]
pub enum FLASHW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl FLASHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHW::DISABLE => false,
            FLASHW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASHW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLASHW::ENABLE)
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
#[doc = "Values that can be written to the field `I2C0`"]
pub enum I2C0W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0W::DISABLE => false,
            I2C0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C0W<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C0W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C0W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO0`"]
pub enum GPIO0W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl GPIO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO0W::DISABLE => false,
            GPIO0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO0W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO0W::ENABLE)
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
#[doc = "Values that can be written to the field `SWM`"]
pub enum SWMW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SWMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWMW::DISABLE => false,
            SWMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWMW<'a> {
    w: &'a mut W,
}
impl<'a> _SWMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWMW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWMW::ENABLE)
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
#[doc = "Values that can be written to the field `SCT`"]
pub enum SCTW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SCTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCTW::DISABLE => false,
            SCTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCTW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCTW::ENABLE)
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
#[doc = "Values that can be written to the field `WKT`"]
pub enum WKTW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl WKTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKTW::DISABLE => false,
            WKTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKTW<'a> {
    w: &'a mut W,
}
impl<'a> _WKTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKTW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKTW::ENABLE)
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
#[doc = "Values that can be written to the field `MRT`"]
pub enum MRTW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl MRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRTW::DISABLE => false,
            MRTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRTW<'a> {
    w: &'a mut W,
}
impl<'a> _MRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRTW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRTW::ENABLE)
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
#[doc = "Values that can be written to the field `SPI0`"]
pub enum SPI0W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SPI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI0W::DISABLE => false,
            SPI0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI0W<'a> {
    w: &'a mut W,
}
impl<'a> _SPI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPI0W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPI0W::ENABLE)
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
#[doc = "Values that can be written to the field `SPI1`"]
pub enum SPI1W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl SPI1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI1W::DISABLE => false,
            SPI1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI1W<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPI1W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPI1W::ENABLE)
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
#[doc = "Values that can be written to the field `CRC`"]
pub enum CRCW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl CRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCW::DISABLE => false,
            CRCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCW::ENABLE)
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
#[doc = "Values that can be written to the field `UART0`"]
pub enum UART0W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART0W::DISABLE => false,
            UART0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0W<'a> {
    w: &'a mut W,
}
impl<'a> _UART0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART0W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART0W::ENABLE)
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
#[doc = "Values that can be written to the field `UART1`"]
pub enum UART1W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART1W::DISABLE => false,
            UART1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1W<'a> {
    w: &'a mut W,
}
impl<'a> _UART1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART1W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART1W::ENABLE)
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
#[doc = "Values that can be written to the field `UART2`"]
pub enum UART2W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART2W::DISABLE => false,
            UART2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART2W<'a> {
    w: &'a mut W,
}
impl<'a> _UART2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART2W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART2W::ENABLE)
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
#[doc = "Values that can be written to the field `WWDT`"]
pub enum WWDTW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl WWDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDTW::DISABLE => false,
            WWDTW::ENABLE => true,
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
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDTW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDTW::ENABLE)
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
#[doc = "Values that can be written to the field `IOCON`"]
pub enum IOCONW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl IOCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCONW::DISABLE => false,
            IOCONW::ENABLE => true,
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
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCONW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCONW::ENABLE)
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
#[doc = "Values that can be written to the field `ACMP`"]
pub enum ACMPW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl ACMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMPW::DISABLE => false,
            ACMPW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMPW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACMPW::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO1`"]
pub enum GPIO1W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl GPIO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO1W::DISABLE => false,
            GPIO1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO1W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO1W::ENABLE)
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
#[doc = "Values that can be written to the field `I2C1`"]
pub enum I2C1W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C1W::DISABLE => false,
            I2C1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C1W<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C1W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C1W::ENABLE)
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
#[doc = "Values that can be written to the field `I2C2`"]
pub enum I2C2W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C2W::DISABLE => false,
            I2C2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C2W<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C2W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C2W::ENABLE)
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
#[doc = "Values that can be written to the field `I2C3`"]
pub enum I2C3W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl I2C3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C3W::DISABLE => false,
            I2C3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C3W<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C3W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C3W::ENABLE)
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
#[doc = "Values that can be written to the field `ADC`"]
pub enum ADCW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl ADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCW::DISABLE => false,
            ADCW::ENABLE => true,
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
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADCW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADCW::ENABLE)
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
#[doc = "Values that can be written to the field `CTIMER`"]
pub enum CTIMERW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl CTIMERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMERW::DISABLE => false,
            CTIMERW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMERW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMERW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMERW::ENABLE)
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
#[doc = "Values that can be written to the field `MTB`"]
pub enum MTBW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl MTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTBW::DISABLE => false,
            MTBW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTBW<'a> {
    w: &'a mut W,
}
impl<'a> _MTBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MTBW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MTBW::ENABLE)
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
#[doc = "Values that can be written to the field `DAC0`"]
pub enum DAC0W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl DAC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAC0W::DISABLE => false,
            DAC0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC0W<'a> {
    w: &'a mut W,
}
impl<'a> _DAC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DAC0W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DAC0W::ENABLE)
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
#[doc = "Values that can be written to the field `GPIO_INT`"]
pub enum GPIO_INTW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl GPIO_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INTW::DISABLE => false,
            GPIO_INTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INTW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INTW::ENABLE)
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
#[doc = "Values that can be written to the field `DMA`"]
pub enum DMAW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAW::DISABLE => false,
            DMAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAW::ENABLE)
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
#[doc = "Values that can be written to the field `UART3`"]
pub enum UART3W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART3W::DISABLE => false,
            UART3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART3W<'a> {
    w: &'a mut W,
}
impl<'a> _UART3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART3W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART3W::ENABLE)
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
#[doc = "Values that can be written to the field `UART4`"]
pub enum UART4W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl UART4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART4W::DISABLE => false,
            UART4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART4W<'a> {
    w: &'a mut W,
}
impl<'a> _UART4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART4W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART4W::ENABLE)
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
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
    #[inline]
    pub fn sys(&self) -> SYSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSR { bits }
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
    #[doc = "Bit 2 - Enables clock for SRAM0 and SRAM1."]
    #[inline]
    pub fn ram0_1(&self) -> RAM0_1R {
        RAM0_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enables clock for flash."]
    #[inline]
    pub fn flash(&self) -> FLASHR {
        FLASHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enables clock for I2C0."]
    #[inline]
    pub fn i2c0(&self) -> I2C0R {
        I2C0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enables clock for GPIO0 port registers."]
    #[inline]
    pub fn gpio0(&self) -> GPIO0R {
        GPIO0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enables clock for switch matrix."]
    #[inline]
    pub fn swm(&self) -> SWMR {
        SWMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables clock for state configurable timer SCTimer/PWM."]
    #[inline]
    pub fn sct(&self) -> SCTR {
        SCTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables clock for self-wake-up timer."]
    #[inline]
    pub fn wkt(&self) -> WKTR {
        WKTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enables clock for multi-rate timer."]
    #[inline]
    pub fn mrt(&self) -> MRTR {
        MRTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline]
    pub fn spi0(&self) -> SPI0R {
        SPI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enables clock for SPI1."]
    #[inline]
    pub fn spi1(&self) -> SPI1R {
        SPI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enables clock for CRC."]
    #[inline]
    pub fn crc(&self) -> CRCR {
        CRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enables clock for UART0."]
    #[inline]
    pub fn uart0(&self) -> UART0R {
        UART0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enables clock for UART1."]
    #[inline]
    pub fn uart1(&self) -> UART1R {
        UART1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enables clock for UART2."]
    #[inline]
    pub fn uart2(&self) -> UART2R {
        UART2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline]
    pub fn wwdt(&self) -> WWDTR {
        WWDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enables clock for IOCON."]
    #[inline]
    pub fn iocon(&self) -> IOCONR {
        IOCONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enables clock for analog comparator."]
    #[inline]
    pub fn acmp(&self) -> ACMPR {
        ACMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enables clock for GPIO1 port registers."]
    #[inline]
    pub fn gpio1(&self) -> GPIO1R {
        GPIO1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enables clock for I2C1."]
    #[inline]
    pub fn i2c1(&self) -> I2C1R {
        I2C1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enables clock for I2C2."]
    #[inline]
    pub fn i2c2(&self) -> I2C2R {
        I2C2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enables clock for I2C3."]
    #[inline]
    pub fn i2c3(&self) -> I2C3R {
        I2C3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enables clock for ADC."]
    #[inline]
    pub fn adc(&self) -> ADCR {
        ADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enables clock for CTIMER."]
    #[inline]
    pub fn ctimer(&self) -> CTIMERR {
        CTIMERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enables clock to micro-trace buffer control registers. Turn on this clock when using the micro-trace buffer for debug purposes."]
    #[inline]
    pub fn mtb(&self) -> MTBR {
        MTBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enables clock for DAC0."]
    #[inline]
    pub fn dac0(&self) -> DAC0R {
        DAC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable clock for GPIO pin interrupt registers"]
    #[inline]
    pub fn gpio_int(&self) -> GPIO_INTR {
        GPIO_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enables clock for DMA."]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enables clock for UART3."]
    #[inline]
    pub fn uart3(&self) -> UART3R {
        UART3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enables clock for UART4."]
    #[inline]
    pub fn uart4(&self) -> UART4R {
        UART4R::_from({
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
        W { bits: 23 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
    #[inline]
    pub fn sys(&mut self) -> _SYSW {
        _SYSW { w: self }
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline]
    pub fn rom(&mut self) -> _ROMW {
        _ROMW { w: self }
    }
    #[doc = "Bit 2 - Enables clock for SRAM0 and SRAM1."]
    #[inline]
    pub fn ram0_1(&mut self) -> _RAM0_1W {
        _RAM0_1W { w: self }
    }
    #[doc = "Bit 4 - Enables clock for flash."]
    #[inline]
    pub fn flash(&mut self) -> _FLASHW {
        _FLASHW { w: self }
    }
    #[doc = "Bit 5 - Enables clock for I2C0."]
    #[inline]
    pub fn i2c0(&mut self) -> _I2C0W {
        _I2C0W { w: self }
    }
    #[doc = "Bit 6 - Enables clock for GPIO0 port registers."]
    #[inline]
    pub fn gpio0(&mut self) -> _GPIO0W {
        _GPIO0W { w: self }
    }
    #[doc = "Bit 7 - Enables clock for switch matrix."]
    #[inline]
    pub fn swm(&mut self) -> _SWMW {
        _SWMW { w: self }
    }
    #[doc = "Bit 8 - Enables clock for state configurable timer SCTimer/PWM."]
    #[inline]
    pub fn sct(&mut self) -> _SCTW {
        _SCTW { w: self }
    }
    #[doc = "Bit 9 - Enables clock for self-wake-up timer."]
    #[inline]
    pub fn wkt(&mut self) -> _WKTW {
        _WKTW { w: self }
    }
    #[doc = "Bit 10 - Enables clock for multi-rate timer."]
    #[inline]
    pub fn mrt(&mut self) -> _MRTW {
        _MRTW { w: self }
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline]
    pub fn spi0(&mut self) -> _SPI0W {
        _SPI0W { w: self }
    }
    #[doc = "Bit 12 - Enables clock for SPI1."]
    #[inline]
    pub fn spi1(&mut self) -> _SPI1W {
        _SPI1W { w: self }
    }
    #[doc = "Bit 13 - Enables clock for CRC."]
    #[inline]
    pub fn crc(&mut self) -> _CRCW {
        _CRCW { w: self }
    }
    #[doc = "Bit 14 - Enables clock for UART0."]
    #[inline]
    pub fn uart0(&mut self) -> _UART0W {
        _UART0W { w: self }
    }
    #[doc = "Bit 15 - Enables clock for UART1."]
    #[inline]
    pub fn uart1(&mut self) -> _UART1W {
        _UART1W { w: self }
    }
    #[doc = "Bit 16 - Enables clock for UART2."]
    #[inline]
    pub fn uart2(&mut self) -> _UART2W {
        _UART2W { w: self }
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline]
    pub fn wwdt(&mut self) -> _WWDTW {
        _WWDTW { w: self }
    }
    #[doc = "Bit 18 - Enables clock for IOCON."]
    #[inline]
    pub fn iocon(&mut self) -> _IOCONW {
        _IOCONW { w: self }
    }
    #[doc = "Bit 19 - Enables clock for analog comparator."]
    #[inline]
    pub fn acmp(&mut self) -> _ACMPW {
        _ACMPW { w: self }
    }
    #[doc = "Bit 20 - Enables clock for GPIO1 port registers."]
    #[inline]
    pub fn gpio1(&mut self) -> _GPIO1W {
        _GPIO1W { w: self }
    }
    #[doc = "Bit 21 - Enables clock for I2C1."]
    #[inline]
    pub fn i2c1(&mut self) -> _I2C1W {
        _I2C1W { w: self }
    }
    #[doc = "Bit 22 - Enables clock for I2C2."]
    #[inline]
    pub fn i2c2(&mut self) -> _I2C2W {
        _I2C2W { w: self }
    }
    #[doc = "Bit 23 - Enables clock for I2C3."]
    #[inline]
    pub fn i2c3(&mut self) -> _I2C3W {
        _I2C3W { w: self }
    }
    #[doc = "Bit 24 - Enables clock for ADC."]
    #[inline]
    pub fn adc(&mut self) -> _ADCW {
        _ADCW { w: self }
    }
    #[doc = "Bit 25 - Enables clock for CTIMER."]
    #[inline]
    pub fn ctimer(&mut self) -> _CTIMERW {
        _CTIMERW { w: self }
    }
    #[doc = "Bit 26 - Enables clock to micro-trace buffer control registers. Turn on this clock when using the micro-trace buffer for debug purposes."]
    #[inline]
    pub fn mtb(&mut self) -> _MTBW {
        _MTBW { w: self }
    }
    #[doc = "Bit 27 - Enables clock for DAC0."]
    #[inline]
    pub fn dac0(&mut self) -> _DAC0W {
        _DAC0W { w: self }
    }
    #[doc = "Bit 28 - Enable clock for GPIO pin interrupt registers"]
    #[inline]
    pub fn gpio_int(&mut self) -> _GPIO_INTW {
        _GPIO_INTW { w: self }
    }
    #[doc = "Bit 29 - Enables clock for DMA."]
    #[inline]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 30 - Enables clock for UART3."]
    #[inline]
    pub fn uart3(&mut self) -> _UART3W {
        _UART3W { w: self }
    }
    #[doc = "Bit 31 - Enables clock for UART4."]
    #[inline]
    pub fn uart4(&mut self) -> _UART4W {
        _UART4W { w: self }
    }
}
