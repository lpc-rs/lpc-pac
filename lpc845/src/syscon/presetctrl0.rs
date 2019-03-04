#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL0 {
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
#[doc = "Possible values of the field `FLASH_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RST_NR {
    #[doc = "Assert the flash controller reset."]
    ASSERT,
    #[doc = "Clear the flash controller reset."]
    CLEAR,
}
impl FLASH_RST_NR {
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
            FLASH_RST_NR::ASSERT => false,
            FLASH_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_RST_NR {
        match value {
            false => FLASH_RST_NR::ASSERT,
            true => FLASH_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == FLASH_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == FLASH_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `I2C0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_RST_NR {
    #[doc = "Assert the I2C0 reset."]
    ASSERT,
    #[doc = "Clear the I2C0 reset."]
    CLEAR,
}
impl I2C0_RST_NR {
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
            I2C0_RST_NR::ASSERT => false,
            I2C0_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C0_RST_NR {
        match value {
            false => I2C0_RST_NR::ASSERT,
            true => I2C0_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == I2C0_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == I2C0_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `GPIO0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_RST_NR {
    #[doc = "Assert the GPIO0 reset."]
    ASSERT,
    #[doc = "Clear the GPIO0 reset."]
    CLEAR,
}
impl GPIO0_RST_NR {
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
            GPIO0_RST_NR::ASSERT => false,
            GPIO0_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO0_RST_NR {
        match value {
            false => GPIO0_RST_NR::ASSERT,
            true => GPIO0_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == GPIO0_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == GPIO0_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `SWM_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWM_RST_NR {
    #[doc = "Assert the SWM reset."]
    ASSERT,
    #[doc = "Clear the SWM reset."]
    CLEAR,
}
impl SWM_RST_NR {
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
            SWM_RST_NR::ASSERT => false,
            SWM_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWM_RST_NR {
        match value {
            false => SWM_RST_NR::ASSERT,
            true => SWM_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == SWM_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == SWM_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `SCT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RST_NR {
    #[doc = "Assert the SCT reset."]
    ASSERT,
    #[doc = "Clear the SCT reset."]
    CLEAR,
}
impl SCT_RST_NR {
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
            SCT_RST_NR::ASSERT => false,
            SCT_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCT_RST_NR {
        match value {
            false => SCT_RST_NR::ASSERT,
            true => SCT_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == SCT_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == SCT_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `WKT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_RST_NR {
    #[doc = "Assert the WKT reset."]
    ASSERT,
    #[doc = "Clear the WKT reset."]
    CLEAR,
}
impl WKT_RST_NR {
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
            WKT_RST_NR::ASSERT => false,
            WKT_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKT_RST_NR {
        match value {
            false => WKT_RST_NR::ASSERT,
            true => WKT_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == WKT_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == WKT_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `MRT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RST_NR {
    #[doc = "Assert the MRT reset."]
    ASSERT,
    #[doc = "Clear the MRT reset."]
    CLEAR,
}
impl MRT_RST_NR {
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
            MRT_RST_NR::ASSERT => false,
            MRT_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRT_RST_NR {
        match value {
            false => MRT_RST_NR::ASSERT,
            true => MRT_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == MRT_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == MRT_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `SPI0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_RST_NR {
    #[doc = "Assert the SPI0 reset."]
    ASSERT,
    #[doc = "Clear the SPI0 reset."]
    CLEAR,
}
impl SPI0_RST_NR {
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
            SPI0_RST_NR::ASSERT => false,
            SPI0_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI0_RST_NR {
        match value {
            false => SPI0_RST_NR::ASSERT,
            true => SPI0_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == SPI0_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == SPI0_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `SPI1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_RST_NR {
    #[doc = "Assert the SPI1 reset."]
    ASSERT,
    #[doc = "Clear the SPI1 reset."]
    CLEAR,
}
impl SPI1_RST_NR {
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
            SPI1_RST_NR::ASSERT => false,
            SPI1_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI1_RST_NR {
        match value {
            false => SPI1_RST_NR::ASSERT,
            true => SPI1_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == SPI1_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == SPI1_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `CRC_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_RST_NR {
    #[doc = "Assert the CRC reset."]
    ASSERT,
    #[doc = "Clear the CRC reset."]
    CLEAR,
}
impl CRC_RST_NR {
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
            CRC_RST_NR::ASSERT => false,
            CRC_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_RST_NR {
        match value {
            false => CRC_RST_NR::ASSERT,
            true => CRC_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == CRC_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CRC_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `UART0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_RST_NR {
    #[doc = "Assert the UART0 reset."]
    ASSERT,
    #[doc = "Clear the UART0 reset."]
    CLEAR,
}
impl UART0_RST_NR {
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
            UART0_RST_NR::ASSERT => false,
            UART0_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART0_RST_NR {
        match value {
            false => UART0_RST_NR::ASSERT,
            true => UART0_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == UART0_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_cl_ear(&self) -> bool {
        *self == UART0_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `UART1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_RST_NR {
    #[doc = "Assert the UART1 reset."]
    ASSERT,
    #[doc = "Clear the UART1 reset."]
    CLEAR,
}
impl UART1_RST_NR {
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
            UART1_RST_NR::ASSERT => false,
            UART1_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART1_RST_NR {
        match value {
            false => UART1_RST_NR::ASSERT,
            true => UART1_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == UART1_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UART1_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `UART2_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_RST_NR {
    #[doc = "Assert the UART2 reset."]
    ASSERT,
    #[doc = "Clear the UART2 reset."]
    CLEAR,
}
impl UART2_RST_NR {
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
            UART2_RST_NR::ASSERT => false,
            UART2_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART2_RST_NR {
        match value {
            false => UART2_RST_NR::ASSERT,
            true => UART2_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == UART2_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UART2_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `IOCON_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_RST_NR {
    #[doc = "Assert the IOCON reset."]
    ASSERT,
    #[doc = "Clear the IOCON reset."]
    CLEAR,
}
impl IOCON_RST_NR {
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
            IOCON_RST_NR::ASSERT => false,
            IOCON_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IOCON_RST_NR {
        match value {
            false => IOCON_RST_NR::ASSERT,
            true => IOCON_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == IOCON_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == IOCON_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `ACMP_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_RST_NR {
    #[doc = "Assert the analog comparator reset."]
    ASSERT,
    #[doc = "Clear the analog comparator reset."]
    CLEAR,
}
impl ACMP_RST_NR {
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
            ACMP_RST_NR::ASSERT => false,
            ACMP_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_RST_NR {
        match value {
            false => ACMP_RST_NR::ASSERT,
            true => ACMP_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == ACMP_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ACMP_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `GPIO1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_RST_NR {
    #[doc = "Assert the GPIO1 reset."]
    ASSERT,
    #[doc = "Clear the GPIO1 reset."]
    CLEAR,
}
impl GPIO1_RST_NR {
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
            GPIO1_RST_NR::ASSERT => false,
            GPIO1_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO1_RST_NR {
        match value {
            false => GPIO1_RST_NR::ASSERT,
            true => GPIO1_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == GPIO1_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == GPIO1_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `I2C1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_RST_NR {
    #[doc = "Assert the I2C1 reset."]
    ASSERT,
    #[doc = "Clear the I2C1 reset."]
    CLEAR,
}
impl I2C1_RST_NR {
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
            I2C1_RST_NR::ASSERT => false,
            I2C1_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C1_RST_NR {
        match value {
            false => I2C1_RST_NR::ASSERT,
            true => I2C1_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == I2C1_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == I2C1_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `I2C2_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_RST_NR {
    #[doc = "Assert the I2C2 reset."]
    ASSERT,
    #[doc = "Clear the I2C2 reset."]
    CLEAR,
}
impl I2C2_RST_NR {
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
            I2C2_RST_NR::ASSERT => false,
            I2C2_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C2_RST_NR {
        match value {
            false => I2C2_RST_NR::ASSERT,
            true => I2C2_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == I2C2_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == I2C2_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `I2C3_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_RST_NR {
    #[doc = "Assert the I2C3 reset."]
    ASSERT,
    #[doc = "Clear the I2C3 reset."]
    CLEAR,
}
impl I2C3_RST_NR {
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
            I2C3_RST_NR::ASSERT => false,
            I2C3_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C3_RST_NR {
        match value {
            false => I2C3_RST_NR::ASSERT,
            true => I2C3_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == I2C3_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == I2C3_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `ADC_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_RST_NR {
    #[doc = "Assert the ADC reset."]
    ASSERT,
    #[doc = "Clear the ADC reset."]
    CLEAR,
}
impl ADC_RST_NR {
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
            ADC_RST_NR::ASSERT => false,
            ADC_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_RST_NR {
        match value {
            false => ADC_RST_NR::ASSERT,
            true => ADC_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == ADC_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ADC_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `CTIMER_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER_RST_NR {
    #[doc = "Assert the CTIMER reset."]
    ASSERT,
    #[doc = "Clear the CTIMER reset."]
    CLEAR,
}
impl CTIMER_RST_NR {
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
            CTIMER_RST_NR::ASSERT => false,
            CTIMER_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER_RST_NR {
        match value {
            false => CTIMER_RST_NR::ASSERT,
            true => CTIMER_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == CTIMER_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CTIMER_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `DAC0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_RST_NR {
    #[doc = "Assert the DAC0 reset."]
    ASSERT,
    #[doc = "Clear the DAC0 reset."]
    CLEAR,
}
impl DAC0_RST_NR {
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
            DAC0_RST_NR::ASSERT => false,
            DAC0_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAC0_RST_NR {
        match value {
            false => DAC0_RST_NR::ASSERT,
            true => DAC0_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == DAC0_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == DAC0_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `GPIOINT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOINT_RST_NR {
    #[doc = "Assert the GPIOINT reset."]
    ASSERT,
    #[doc = "Clear the GPIOINT reset."]
    CLEAR,
}
impl GPIOINT_RST_NR {
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
            GPIOINT_RST_NR::ASSERT => false,
            GPIOINT_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIOINT_RST_NR {
        match value {
            false => GPIOINT_RST_NR::ASSERT,
            true => GPIOINT_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == GPIOINT_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == GPIOINT_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `DMA_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_RST_NR {
    #[doc = "Assert the DMA reset."]
    ASSERT,
    #[doc = "Clear the DMA reset."]
    CLEAR,
}
impl DMA_RST_NR {
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
            DMA_RST_NR::ASSERT => false,
            DMA_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_RST_NR {
        match value {
            false => DMA_RST_NR::ASSERT,
            true => DMA_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == DMA_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == DMA_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `UART3_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3_RST_NR {
    #[doc = "Assert the UART3 reset."]
    ASSERT,
    #[doc = "Clear the UART3 reset."]
    CLEAR,
}
impl UART3_RST_NR {
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
            UART3_RST_NR::ASSERT => false,
            UART3_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART3_RST_NR {
        match value {
            false => UART3_RST_NR::ASSERT,
            true => UART3_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == UART3_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UART3_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `UART4_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART4_RST_NR {
    #[doc = "Assert the UART4 reset."]
    ASSERT,
    #[doc = "Clear the UART4 reset."]
    CLEAR,
}
impl UART4_RST_NR {
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
            UART4_RST_NR::ASSERT => false,
            UART4_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART4_RST_NR {
        match value {
            false => UART4_RST_NR::ASSERT,
            true => UART4_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == UART4_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UART4_RST_NR::CLEAR
    }
}
#[doc = "Values that can be written to the field `FLASH_RST_N`"]
pub enum FLASH_RST_NW {
    #[doc = "Assert the flash controller reset."]
    ASSERT,
    #[doc = "Clear the flash controller reset."]
    CLEAR,
}
impl FLASH_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH_RST_NW::ASSERT => false,
            FLASH_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the flash controller reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(FLASH_RST_NW::ASSERT)
    }
    #[doc = "Clear the flash controller reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLASH_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `I2C0_RST_N`"]
pub enum I2C0_RST_NW {
    #[doc = "Assert the I2C0 reset."]
    ASSERT,
    #[doc = "Clear the I2C0 reset."]
    CLEAR,
}
impl I2C0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0_RST_NW::ASSERT => false,
            I2C0_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the I2C0 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C0_RST_NW::ASSERT)
    }
    #[doc = "Clear the I2C0 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C0_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `GPIO0_RST_N`"]
pub enum GPIO0_RST_NW {
    #[doc = "Assert the GPIO0 reset."]
    ASSERT,
    #[doc = "Clear the GPIO0 reset."]
    CLEAR,
}
impl GPIO0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO0_RST_NW::ASSERT => false,
            GPIO0_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the GPIO0 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(GPIO0_RST_NW::ASSERT)
    }
    #[doc = "Clear the GPIO0 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO0_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `SWM_RST_N`"]
pub enum SWM_RST_NW {
    #[doc = "Assert the SWM reset."]
    ASSERT,
    #[doc = "Clear the SWM reset."]
    CLEAR,
}
impl SWM_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWM_RST_NW::ASSERT => false,
            SWM_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWM_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SWM_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWM_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the SWM reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(SWM_RST_NW::ASSERT)
    }
    #[doc = "Clear the SWM reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SWM_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `SCT_RST_N`"]
pub enum SCT_RST_NW {
    #[doc = "Assert the SCT reset."]
    ASSERT,
    #[doc = "Clear the SCT reset."]
    CLEAR,
}
impl SCT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT_RST_NW::ASSERT => false,
            SCT_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the SCT reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(SCT_RST_NW::ASSERT)
    }
    #[doc = "Clear the SCT reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SCT_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `WKT_RST_N`"]
pub enum WKT_RST_NW {
    #[doc = "Assert the WKT reset."]
    ASSERT,
    #[doc = "Clear the WKT reset."]
    CLEAR,
}
impl WKT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKT_RST_NW::ASSERT => false,
            WKT_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _WKT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the WKT reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(WKT_RST_NW::ASSERT)
    }
    #[doc = "Clear the WKT reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(WKT_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `MRT_RST_N`"]
pub enum MRT_RST_NW {
    #[doc = "Assert the MRT reset."]
    ASSERT,
    #[doc = "Clear the MRT reset."]
    CLEAR,
}
impl MRT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRT_RST_NW::ASSERT => false,
            MRT_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _MRT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the MRT reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(MRT_RST_NW::ASSERT)
    }
    #[doc = "Clear the MRT reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(MRT_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `SPI0_RST_N`"]
pub enum SPI0_RST_NW {
    #[doc = "Assert the SPI0 reset."]
    ASSERT,
    #[doc = "Clear the SPI0 reset."]
    CLEAR,
}
impl SPI0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI0_RST_NW::ASSERT => false,
            SPI0_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the SPI0 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(SPI0_RST_NW::ASSERT)
    }
    #[doc = "Clear the SPI0 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SPI0_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `SPI1_RST_N`"]
pub enum SPI1_RST_NW {
    #[doc = "Assert the SPI1 reset."]
    ASSERT,
    #[doc = "Clear the SPI1 reset."]
    CLEAR,
}
impl SPI1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI1_RST_NW::ASSERT => false,
            SPI1_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the SPI1 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(SPI1_RST_NW::ASSERT)
    }
    #[doc = "Clear the SPI1 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SPI1_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `CRC_RST_N`"]
pub enum CRC_RST_NW {
    #[doc = "Assert the CRC reset."]
    ASSERT,
    #[doc = "Clear the CRC reset."]
    CLEAR,
}
impl CRC_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_RST_NW::ASSERT => false,
            CRC_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the CRC reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(CRC_RST_NW::ASSERT)
    }
    #[doc = "Clear the CRC reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRC_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `UART0_RST_N`"]
pub enum UART0_RST_NW {
    #[doc = "Assert the UART0 reset."]
    ASSERT,
    #[doc = "Clear the UART0 reset."]
    CLEAR,
}
impl UART0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART0_RST_NW::ASSERT => false,
            UART0_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UART0 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART0_RST_NW::ASSERT)
    }
    #[doc = "Clear the UART0 reset."]
    #[inline]
    pub fn cl_ear(self) -> &'a mut W {
        self.variant(UART0_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `UART1_RST_N`"]
pub enum UART1_RST_NW {
    #[doc = "Assert the UART1 reset."]
    ASSERT,
    #[doc = "Clear the UART1 reset."]
    CLEAR,
}
impl UART1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART1_RST_NW::ASSERT => false,
            UART1_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UART1 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART1_RST_NW::ASSERT)
    }
    #[doc = "Clear the UART1 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART1_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `UART2_RST_N`"]
pub enum UART2_RST_NW {
    #[doc = "Assert the UART2 reset."]
    ASSERT,
    #[doc = "Clear the UART2 reset."]
    CLEAR,
}
impl UART2_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART2_RST_NW::ASSERT => false,
            UART2_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART2_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UART2_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART2_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UART2 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART2_RST_NW::ASSERT)
    }
    #[doc = "Clear the UART2 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART2_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `IOCON_RST_N`"]
pub enum IOCON_RST_NW {
    #[doc = "Assert the IOCON reset."]
    ASSERT,
    #[doc = "Clear the IOCON reset."]
    CLEAR,
}
impl IOCON_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCON_RST_NW::ASSERT => false,
            IOCON_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCON_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCON_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCON_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the IOCON reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(IOCON_RST_NW::ASSERT)
    }
    #[doc = "Clear the IOCON reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(IOCON_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `ACMP_RST_N`"]
pub enum ACMP_RST_NW {
    #[doc = "Assert the analog comparator reset."]
    ASSERT,
    #[doc = "Clear the analog comparator reset."]
    CLEAR,
}
impl ACMP_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_RST_NW::ASSERT => false,
            ACMP_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the analog comparator reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(ACMP_RST_NW::ASSERT)
    }
    #[doc = "Clear the analog comparator reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACMP_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `GPIO1_RST_N`"]
pub enum GPIO1_RST_NW {
    #[doc = "Assert the GPIO1 reset."]
    ASSERT,
    #[doc = "Clear the GPIO1 reset."]
    CLEAR,
}
impl GPIO1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO1_RST_NW::ASSERT => false,
            GPIO1_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the GPIO1 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(GPIO1_RST_NW::ASSERT)
    }
    #[doc = "Clear the GPIO1 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO1_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `I2C1_RST_N`"]
pub enum I2C1_RST_NW {
    #[doc = "Assert the I2C1 reset."]
    ASSERT,
    #[doc = "Clear the I2C1 reset."]
    CLEAR,
}
impl I2C1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C1_RST_NW::ASSERT => false,
            I2C1_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the I2C1 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C1_RST_NW::ASSERT)
    }
    #[doc = "Clear the I2C1 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C1_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `I2C2_RST_N`"]
pub enum I2C2_RST_NW {
    #[doc = "Assert the I2C2 reset."]
    ASSERT,
    #[doc = "Clear the I2C2 reset."]
    CLEAR,
}
impl I2C2_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C2_RST_NW::ASSERT => false,
            I2C2_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C2_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the I2C2 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C2_RST_NW::ASSERT)
    }
    #[doc = "Clear the I2C2 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C2_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `I2C3_RST_N`"]
pub enum I2C3_RST_NW {
    #[doc = "Assert the I2C3 reset."]
    ASSERT,
    #[doc = "Clear the I2C3 reset."]
    CLEAR,
}
impl I2C3_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C3_RST_NW::ASSERT => false,
            I2C3_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C3_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C3_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the I2C3 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C3_RST_NW::ASSERT)
    }
    #[doc = "Clear the I2C3 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C3_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `ADC_RST_N`"]
pub enum ADC_RST_NW {
    #[doc = "Assert the ADC reset."]
    ASSERT,
    #[doc = "Clear the ADC reset."]
    CLEAR,
}
impl ADC_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_RST_NW::ASSERT => false,
            ADC_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the ADC reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(ADC_RST_NW::ASSERT)
    }
    #[doc = "Clear the ADC reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADC_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `CTIMER_RST_N`"]
pub enum CTIMER_RST_NW {
    #[doc = "Assert the CTIMER reset."]
    ASSERT,
    #[doc = "Clear the CTIMER reset."]
    CLEAR,
}
impl CTIMER_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER_RST_NW::ASSERT => false,
            CTIMER_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the CTIMER reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(CTIMER_RST_NW::ASSERT)
    }
    #[doc = "Clear the CTIMER reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTIMER_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `DAC0_RST_N`"]
pub enum DAC0_RST_NW {
    #[doc = "Assert the DAC0 reset."]
    ASSERT,
    #[doc = "Clear the DAC0 reset."]
    CLEAR,
}
impl DAC0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAC0_RST_NW::ASSERT => false,
            DAC0_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the DAC0 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(DAC0_RST_NW::ASSERT)
    }
    #[doc = "Clear the DAC0 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DAC0_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `GPIOINT_RST_N`"]
pub enum GPIOINT_RST_NW {
    #[doc = "Assert the GPIOINT reset."]
    ASSERT,
    #[doc = "Clear the GPIOINT reset."]
    CLEAR,
}
impl GPIOINT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOINT_RST_NW::ASSERT => false,
            GPIOINT_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIOINT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOINT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOINT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the GPIOINT reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(GPIOINT_RST_NW::ASSERT)
    }
    #[doc = "Clear the GPIOINT reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIOINT_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `DMA_RST_N`"]
pub enum DMA_RST_NW {
    #[doc = "Assert the DMA reset."]
    ASSERT,
    #[doc = "Clear the DMA reset."]
    CLEAR,
}
impl DMA_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_RST_NW::ASSERT => false,
            DMA_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the DMA reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(DMA_RST_NW::ASSERT)
    }
    #[doc = "Clear the DMA reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DMA_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `UART3_RST_N`"]
pub enum UART3_RST_NW {
    #[doc = "Assert the UART3 reset."]
    ASSERT,
    #[doc = "Clear the UART3 reset."]
    CLEAR,
}
impl UART3_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART3_RST_NW::ASSERT => false,
            UART3_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART3_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UART3_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART3_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UART3 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART3_RST_NW::ASSERT)
    }
    #[doc = "Clear the UART3 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART3_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `UART4_RST_N`"]
pub enum UART4_RST_NW {
    #[doc = "Assert the UART4 reset."]
    ASSERT,
    #[doc = "Clear the UART4 reset."]
    CLEAR,
}
impl UART4_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UART4_RST_NW::ASSERT => false,
            UART4_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UART4_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _UART4_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART4_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the UART4 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART4_RST_NW::ASSERT)
    }
    #[doc = "Clear the UART4 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART4_RST_NW::CLEAR)
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
    #[doc = "Bit 4 - flash controller reset control"]
    #[inline]
    pub fn flash_rst_n(&self) -> FLASH_RST_NR {
        FLASH_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - I2C0 reset control"]
    #[inline]
    pub fn i2c0_rst_n(&self) -> I2C0_RST_NR {
        I2C0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - GPIO0 reset control"]
    #[inline]
    pub fn gpio0_rst_n(&self) -> GPIO0_RST_NR {
        GPIO0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SWM reset control"]
    #[inline]
    pub fn swm_rst_n(&self) -> SWM_RST_NR {
        SWM_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SCT reset control"]
    #[inline]
    pub fn sct_rst_n(&self) -> SCT_RST_NR {
        SCT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control"]
    #[inline]
    pub fn wkt_rst_n(&self) -> WKT_RST_NR {
        WKT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Multi-rate timer (MRT) reset control"]
    #[inline]
    pub fn mrt_rst_n(&self) -> MRT_RST_NR {
        MRT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SPI0 reset control"]
    #[inline]
    pub fn spi0_rst_n(&self) -> SPI0_RST_NR {
        SPI0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI1 reset control"]
    #[inline]
    pub fn spi1_rst_n(&self) -> SPI1_RST_NR {
        SPI1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - CRC engine reset control"]
    #[inline]
    pub fn crc_rst_n(&self) -> CRC_RST_NR {
        CRC_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - UART0 reset control"]
    #[inline]
    pub fn uart0_rst_n(&self) -> UART0_RST_NR {
        UART0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - UART1 reset control"]
    #[inline]
    pub fn uart1_rst_n(&self) -> UART1_RST_NR {
        UART1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - UART2 reset control"]
    #[inline]
    pub fn uart2_rst_n(&self) -> UART2_RST_NR {
        UART2_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - IOCON reset control"]
    #[inline]
    pub fn iocon_rst_n(&self) -> IOCON_RST_NR {
        IOCON_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Analog comparator reset control"]
    #[inline]
    pub fn acmp_rst_n(&self) -> ACMP_RST_NR {
        ACMP_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - GPIO1 reset control"]
    #[inline]
    pub fn gpio1_rst_n(&self) -> GPIO1_RST_NR {
        GPIO1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - I2C1 reset control"]
    #[inline]
    pub fn i2c1_rst_n(&self) -> I2C1_RST_NR {
        I2C1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I2C2 reset control"]
    #[inline]
    pub fn i2c2_rst_n(&self) -> I2C2_RST_NR {
        I2C2_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - I2C3 reset control"]
    #[inline]
    pub fn i2c3_rst_n(&self) -> I2C3_RST_NR {
        I2C3_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - ADC reset control"]
    #[inline]
    pub fn adc_rst_n(&self) -> ADC_RST_NR {
        ADC_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CTIMER reset control"]
    #[inline]
    pub fn ctimer_rst_n(&self) -> CTIMER_RST_NR {
        CTIMER_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - DAC0 reset control"]
    #[inline]
    pub fn dac0_rst_n(&self) -> DAC0_RST_NR {
        DAC0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - GPIOINT reset control"]
    #[inline]
    pub fn gpioint_rst_n(&self) -> GPIOINT_RST_NR {
        GPIOINT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - DMA reset control"]
    #[inline]
    pub fn dma_rst_n(&self) -> DMA_RST_NR {
        DMA_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - UART3 reset control"]
    #[inline]
    pub fn uart3_rst_n(&self) -> UART3_RST_NR {
        UART3_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - UART4 reset control"]
    #[inline]
    pub fn uart4_rst_n(&self) -> UART4_RST_NR {
        UART4_RST_NR::_from({
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
        W { bits: 4227727344 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - flash controller reset control"]
    #[inline]
    pub fn flash_rst_n(&mut self) -> _FLASH_RST_NW {
        _FLASH_RST_NW { w: self }
    }
    #[doc = "Bit 5 - I2C0 reset control"]
    #[inline]
    pub fn i2c0_rst_n(&mut self) -> _I2C0_RST_NW {
        _I2C0_RST_NW { w: self }
    }
    #[doc = "Bit 6 - GPIO0 reset control"]
    #[inline]
    pub fn gpio0_rst_n(&mut self) -> _GPIO0_RST_NW {
        _GPIO0_RST_NW { w: self }
    }
    #[doc = "Bit 7 - SWM reset control"]
    #[inline]
    pub fn swm_rst_n(&mut self) -> _SWM_RST_NW {
        _SWM_RST_NW { w: self }
    }
    #[doc = "Bit 8 - SCT reset control"]
    #[inline]
    pub fn sct_rst_n(&mut self) -> _SCT_RST_NW {
        _SCT_RST_NW { w: self }
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control"]
    #[inline]
    pub fn wkt_rst_n(&mut self) -> _WKT_RST_NW {
        _WKT_RST_NW { w: self }
    }
    #[doc = "Bit 10 - Multi-rate timer (MRT) reset control"]
    #[inline]
    pub fn mrt_rst_n(&mut self) -> _MRT_RST_NW {
        _MRT_RST_NW { w: self }
    }
    #[doc = "Bit 11 - SPI0 reset control"]
    #[inline]
    pub fn spi0_rst_n(&mut self) -> _SPI0_RST_NW {
        _SPI0_RST_NW { w: self }
    }
    #[doc = "Bit 12 - SPI1 reset control"]
    #[inline]
    pub fn spi1_rst_n(&mut self) -> _SPI1_RST_NW {
        _SPI1_RST_NW { w: self }
    }
    #[doc = "Bit 13 - CRC engine reset control"]
    #[inline]
    pub fn crc_rst_n(&mut self) -> _CRC_RST_NW {
        _CRC_RST_NW { w: self }
    }
    #[doc = "Bit 14 - UART0 reset control"]
    #[inline]
    pub fn uart0_rst_n(&mut self) -> _UART0_RST_NW {
        _UART0_RST_NW { w: self }
    }
    #[doc = "Bit 15 - UART1 reset control"]
    #[inline]
    pub fn uart1_rst_n(&mut self) -> _UART1_RST_NW {
        _UART1_RST_NW { w: self }
    }
    #[doc = "Bit 16 - UART2 reset control"]
    #[inline]
    pub fn uart2_rst_n(&mut self) -> _UART2_RST_NW {
        _UART2_RST_NW { w: self }
    }
    #[doc = "Bit 18 - IOCON reset control"]
    #[inline]
    pub fn iocon_rst_n(&mut self) -> _IOCON_RST_NW {
        _IOCON_RST_NW { w: self }
    }
    #[doc = "Bit 19 - Analog comparator reset control"]
    #[inline]
    pub fn acmp_rst_n(&mut self) -> _ACMP_RST_NW {
        _ACMP_RST_NW { w: self }
    }
    #[doc = "Bit 20 - GPIO1 reset control"]
    #[inline]
    pub fn gpio1_rst_n(&mut self) -> _GPIO1_RST_NW {
        _GPIO1_RST_NW { w: self }
    }
    #[doc = "Bit 21 - I2C1 reset control"]
    #[inline]
    pub fn i2c1_rst_n(&mut self) -> _I2C1_RST_NW {
        _I2C1_RST_NW { w: self }
    }
    #[doc = "Bit 22 - I2C2 reset control"]
    #[inline]
    pub fn i2c2_rst_n(&mut self) -> _I2C2_RST_NW {
        _I2C2_RST_NW { w: self }
    }
    #[doc = "Bit 23 - I2C3 reset control"]
    #[inline]
    pub fn i2c3_rst_n(&mut self) -> _I2C3_RST_NW {
        _I2C3_RST_NW { w: self }
    }
    #[doc = "Bit 24 - ADC reset control"]
    #[inline]
    pub fn adc_rst_n(&mut self) -> _ADC_RST_NW {
        _ADC_RST_NW { w: self }
    }
    #[doc = "Bit 25 - CTIMER reset control"]
    #[inline]
    pub fn ctimer_rst_n(&mut self) -> _CTIMER_RST_NW {
        _CTIMER_RST_NW { w: self }
    }
    #[doc = "Bit 27 - DAC0 reset control"]
    #[inline]
    pub fn dac0_rst_n(&mut self) -> _DAC0_RST_NW {
        _DAC0_RST_NW { w: self }
    }
    #[doc = "Bit 28 - GPIOINT reset control"]
    #[inline]
    pub fn gpioint_rst_n(&mut self) -> _GPIOINT_RST_NW {
        _GPIOINT_RST_NW { w: self }
    }
    #[doc = "Bit 29 - DMA reset control"]
    #[inline]
    pub fn dma_rst_n(&mut self) -> _DMA_RST_NW {
        _DMA_RST_NW { w: self }
    }
    #[doc = "Bit 30 - UART3 reset control"]
    #[inline]
    pub fn uart3_rst_n(&mut self) -> _UART3_RST_NW {
        _UART3_RST_NW { w: self }
    }
    #[doc = "Bit 31 - UART4 reset control"]
    #[inline]
    pub fn uart4_rst_n(&mut self) -> _UART4_RST_NW {
        _UART4_RST_NW { w: self }
    }
}
