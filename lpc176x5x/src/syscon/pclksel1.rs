#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCLKSEL1 {
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
#[doc = "Possible values of the field `PCLK_QEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_QEIR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_QEIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_QEIR::CCLK_DIV_4 => 0,
            PCLK_QEIR::CCLK => 1,
            PCLK_QEIR::CCLK_DIV_2 => 2,
            PCLK_QEIR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_QEIR {
        match value {
            0 => PCLK_QEIR::CCLK_DIV_4,
            1 => PCLK_QEIR::CCLK,
            2 => PCLK_QEIR::CCLK_DIV_2,
            3 => PCLK_QEIR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_QEIR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_QEIR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_QEIR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_QEIR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_GPIOINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_GPIOINTR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_GPIOINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_GPIOINTR::CCLK_DIV_4 => 0,
            PCLK_GPIOINTR::CCLK => 1,
            PCLK_GPIOINTR::CCLK_DIV_2 => 2,
            PCLK_GPIOINTR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_GPIOINTR {
        match value {
            0 => PCLK_GPIOINTR::CCLK_DIV_4,
            1 => PCLK_GPIOINTR::CCLK,
            2 => PCLK_GPIOINTR::CCLK_DIV_2,
            3 => PCLK_GPIOINTR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_GPIOINTR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_GPIOINTR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_GPIOINTR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_GPIOINTR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_PCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_PCBR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_PCBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_PCBR::CCLK_DIV_4 => 0,
            PCLK_PCBR::CCLK => 1,
            PCLK_PCBR::CCLK_DIV_2 => 2,
            PCLK_PCBR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_PCBR {
        match value {
            0 => PCLK_PCBR::CCLK_DIV_4,
            1 => PCLK_PCBR::CCLK,
            2 => PCLK_PCBR::CCLK_DIV_2,
            3 => PCLK_PCBR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_PCBR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_PCBR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_PCBR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_PCBR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_I2C1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_I2C1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2C1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_I2C1R::CCLK_DIV_4 => 0,
            PCLK_I2C1R::CCLK => 1,
            PCLK_I2C1R::CCLK_DIV_2 => 2,
            PCLK_I2C1R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_I2C1R {
        match value {
            0 => PCLK_I2C1R::CCLK_DIV_4,
            1 => PCLK_I2C1R::CCLK,
            2 => PCLK_I2C1R::CCLK_DIV_2,
            3 => PCLK_I2C1R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C1R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_SSP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SSP0R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SSP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_SSP0R::CCLK_DIV_4 => 0,
            PCLK_SSP0R::CCLK => 1,
            PCLK_SSP0R::CCLK_DIV_2 => 2,
            PCLK_SSP0R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_SSP0R {
        match value {
            0 => PCLK_SSP0R::CCLK_DIV_4,
            1 => PCLK_SSP0R::CCLK,
            2 => PCLK_SSP0R::CCLK_DIV_2,
            3 => PCLK_SSP0R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SSP0R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SSP0R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SSP0R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SSP0R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_TIMER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER2R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_TIMER2R::CCLK_DIV_4 => 0,
            PCLK_TIMER2R::CCLK => 1,
            PCLK_TIMER2R::CCLK_DIV_2 => 2,
            PCLK_TIMER2R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_TIMER2R {
        match value {
            0 => PCLK_TIMER2R::CCLK_DIV_4,
            1 => PCLK_TIMER2R::CCLK,
            2 => PCLK_TIMER2R::CCLK_DIV_2,
            3 => PCLK_TIMER2R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER2R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER2R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER2R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER2R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_TIMER3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER3R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_TIMER3R::CCLK_DIV_4 => 0,
            PCLK_TIMER3R::CCLK => 1,
            PCLK_TIMER3R::CCLK_DIV_2 => 2,
            PCLK_TIMER3R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_TIMER3R {
        match value {
            0 => PCLK_TIMER3R::CCLK_DIV_4,
            1 => PCLK_TIMER3R::CCLK,
            2 => PCLK_TIMER3R::CCLK_DIV_2,
            3 => PCLK_TIMER3R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER3R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER3R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER3R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER3R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_UART2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART2R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_UART2R::CCLK_DIV_4 => 0,
            PCLK_UART2R::CCLK => 1,
            PCLK_UART2R::CCLK_DIV_2 => 2,
            PCLK_UART2R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_UART2R {
        match value {
            0 => PCLK_UART2R::CCLK_DIV_4,
            1 => PCLK_UART2R::CCLK,
            2 => PCLK_UART2R::CCLK_DIV_2,
            3 => PCLK_UART2R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART2R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART2R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART2R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART2R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_UART3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART3R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_UART3R::CCLK_DIV_4 => 0,
            PCLK_UART3R::CCLK => 1,
            PCLK_UART3R::CCLK_DIV_2 => 2,
            PCLK_UART3R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_UART3R {
        match value {
            0 => PCLK_UART3R::CCLK_DIV_4,
            1 => PCLK_UART3R::CCLK,
            2 => PCLK_UART3R::CCLK_DIV_2,
            3 => PCLK_UART3R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART3R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART3R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART3R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART3R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_I2C2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_I2C2R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2C2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_I2C2R::CCLK_DIV_4 => 0,
            PCLK_I2C2R::CCLK => 1,
            PCLK_I2C2R::CCLK_DIV_2 => 2,
            PCLK_I2C2R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_I2C2R {
        match value {
            0 => PCLK_I2C2R::CCLK_DIV_4,
            1 => PCLK_I2C2R::CCLK,
            2 => PCLK_I2C2R::CCLK_DIV_2,
            3 => PCLK_I2C2R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C2R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C2R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C2R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C2R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_I2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_I2SR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_I2SR::CCLK_DIV_4 => 0,
            PCLK_I2SR::CCLK => 1,
            PCLK_I2SR::CCLK_DIV_2 => 2,
            PCLK_I2SR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_I2SR {
        match value {
            0 => PCLK_I2SR::CCLK_DIV_4,
            1 => PCLK_I2SR::CCLK,
            2 => PCLK_I2SR::CCLK_DIV_2,
            3 => PCLK_I2SR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2SR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2SR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2SR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2SR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_RIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_RITR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_RITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_RITR::CCLK_DIV_4 => 0,
            PCLK_RITR::CCLK => 1,
            PCLK_RITR::CCLK_DIV_2 => 2,
            PCLK_RITR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_RITR {
        match value {
            0 => PCLK_RITR::CCLK_DIV_4,
            1 => PCLK_RITR::CCLK,
            2 => PCLK_RITR::CCLK_DIV_2,
            3 => PCLK_RITR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_RITR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_RITR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_RITR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_RITR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_SYSCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SYSCONR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SYSCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_SYSCONR::CCLK_DIV_4 => 0,
            PCLK_SYSCONR::CCLK => 1,
            PCLK_SYSCONR::CCLK_DIV_2 => 2,
            PCLK_SYSCONR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_SYSCONR {
        match value {
            0 => PCLK_SYSCONR::CCLK_DIV_4,
            1 => PCLK_SYSCONR::CCLK,
            2 => PCLK_SYSCONR::CCLK_DIV_2,
            3 => PCLK_SYSCONR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SYSCONR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SYSCONR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SYSCONR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SYSCONR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_MC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_MCR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_MCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_MCR::CCLK_DIV_4 => 0,
            PCLK_MCR::CCLK => 1,
            PCLK_MCR::CCLK_DIV_2 => 2,
            PCLK_MCR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_MCR {
        match value {
            0 => PCLK_MCR::CCLK_DIV_4,
            1 => PCLK_MCR::CCLK,
            2 => PCLK_MCR::CCLK_DIV_2,
            3 => PCLK_MCR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_MCR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_MCR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_MCR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_MCR::CCLK_DIV_8
    }
}
#[doc = "Values that can be written to the field `PCLK_QEI`"]
pub enum PCLK_QEIW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_QEIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_QEIW::CCLK_DIV_4 => 0,
            PCLK_QEIW::CCLK => 1,
            PCLK_QEIW::CCLK_DIV_2 => 2,
            PCLK_QEIW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_QEIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_QEIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_QEIW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_QEIW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_QEIW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_QEIW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_QEIW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_GPIOINT`"]
pub enum PCLK_GPIOINTW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_GPIOINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_GPIOINTW::CCLK_DIV_4 => 0,
            PCLK_GPIOINTW::CCLK => 1,
            PCLK_GPIOINTW::CCLK_DIV_2 => 2,
            PCLK_GPIOINTW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_GPIOINTW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_GPIOINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_GPIOINTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_GPIOINTW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_GPIOINTW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_GPIOINTW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_GPIOINTW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_PCB`"]
pub enum PCLK_PCBW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_PCBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_PCBW::CCLK_DIV_4 => 0,
            PCLK_PCBW::CCLK => 1,
            PCLK_PCBW::CCLK_DIV_2 => 2,
            PCLK_PCBW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_PCBW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_PCBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_PCBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_PCBW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_PCBW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_PCBW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_PCBW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_I2C1`"]
pub enum PCLK_I2C1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2C1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_I2C1W::CCLK_DIV_4 => 0,
            PCLK_I2C1W::CCLK => 1,
            PCLK_I2C1W::CCLK_DIV_2 => 2,
            PCLK_I2C1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_I2C1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_I2C1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_I2C1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2C1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2C1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2C1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2C1W::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_SSP0`"]
pub enum PCLK_SSP0W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SSP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_SSP0W::CCLK_DIV_4 => 0,
            PCLK_SSP0W::CCLK => 1,
            PCLK_SSP0W::CCLK_DIV_2 => 2,
            PCLK_SSP0W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_SSP0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_SSP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_SSP0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SSP0W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SSP0W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SSP0W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SSP0W::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_TIMER2`"]
pub enum PCLK_TIMER2W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER2W::CCLK_DIV_4 => 0,
            PCLK_TIMER2W::CCLK => 1,
            PCLK_TIMER2W::CCLK_DIV_2 => 2,
            PCLK_TIMER2W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_TIMER2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_TIMER2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_TIMER2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER2W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER2W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER2W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER2W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_TIMER3`"]
pub enum PCLK_TIMER3W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER3W::CCLK_DIV_4 => 0,
            PCLK_TIMER3W::CCLK => 1,
            PCLK_TIMER3W::CCLK_DIV_2 => 2,
            PCLK_TIMER3W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_TIMER3W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_TIMER3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_TIMER3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER3W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER3W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER3W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER3W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_UART2`"]
pub enum PCLK_UART2W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART2W::CCLK_DIV_4 => 0,
            PCLK_UART2W::CCLK => 1,
            PCLK_UART2W::CCLK_DIV_2 => 2,
            PCLK_UART2W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_UART2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_UART2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_UART2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART2W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART2W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART2W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART2W::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_UART3`"]
pub enum PCLK_UART3W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART3W::CCLK_DIV_4 => 0,
            PCLK_UART3W::CCLK => 1,
            PCLK_UART3W::CCLK_DIV_2 => 2,
            PCLK_UART3W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_UART3W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_UART3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_UART3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART3W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART3W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART3W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART3W::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_I2C2`"]
pub enum PCLK_I2C2W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2C2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_I2C2W::CCLK_DIV_4 => 0,
            PCLK_I2C2W::CCLK => 1,
            PCLK_I2C2W::CCLK_DIV_2 => 2,
            PCLK_I2C2W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_I2C2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_I2C2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_I2C2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2C2W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2C2W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2C2W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2C2W::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_I2S`"]
pub enum PCLK_I2SW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_I2SW::CCLK_DIV_4 => 0,
            PCLK_I2SW::CCLK => 1,
            PCLK_I2SW::CCLK_DIV_2 => 2,
            PCLK_I2SW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_I2SW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_I2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_I2SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2SW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2SW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2SW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2SW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_RIT`"]
pub enum PCLK_RITW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_RITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_RITW::CCLK_DIV_4 => 0,
            PCLK_RITW::CCLK => 1,
            PCLK_RITW::CCLK_DIV_2 => 2,
            PCLK_RITW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_RITW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_RITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_RITW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_RITW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_RITW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_RITW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_RITW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_SYSCON`"]
pub enum PCLK_SYSCONW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SYSCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_SYSCONW::CCLK_DIV_4 => 0,
            PCLK_SYSCONW::CCLK => 1,
            PCLK_SYSCONW::CCLK_DIV_2 => 2,
            PCLK_SYSCONW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_SYSCONW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_SYSCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_SYSCONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SYSCONW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SYSCONW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SYSCONW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SYSCONW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_MC`"]
pub enum PCLK_MCW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_MCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_MCW::CCLK_DIV_4 => 0,
            PCLK_MCW::CCLK => 1,
            PCLK_MCW::CCLK_DIV_2 => 2,
            PCLK_MCW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_MCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_MCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_MCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_MCW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_MCW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_MCW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_MCW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline]
    pub fn pclk_qei(&self) -> PCLK_QEIR {
        PCLK_QEIR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline]
    pub fn pclk_gpioint(&self) -> PCLK_GPIOINTR {
        PCLK_GPIOINTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline]
    pub fn pclk_pcb(&self) -> PCLK_PCBR {
        PCLK_PCBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline]
    pub fn pclk_i2c1(&self) -> PCLK_I2C1R {
        PCLK_I2C1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline]
    pub fn pclk_ssp0(&self) -> PCLK_SSP0R {
        PCLK_SSP0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline]
    pub fn pclk_timer2(&self) -> PCLK_TIMER2R {
        PCLK_TIMER2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline]
    pub fn pclk_timer3(&self) -> PCLK_TIMER3R {
        PCLK_TIMER3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline]
    pub fn pclk_uart2(&self) -> PCLK_UART2R {
        PCLK_UART2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline]
    pub fn pclk_uart3(&self) -> PCLK_UART3R {
        PCLK_UART3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline]
    pub fn pclk_i2c2(&self) -> PCLK_I2C2R {
        PCLK_I2C2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline]
    pub fn pclk_i2s(&self) -> PCLK_I2SR {
        PCLK_I2SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline]
    pub fn pclk_rit(&self) -> PCLK_RITR {
        PCLK_RITR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline]
    pub fn pclk_syscon(&self) -> PCLK_SYSCONR {
        PCLK_SYSCONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline]
    pub fn pclk_mc(&self) -> PCLK_MCR {
        PCLK_MCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline]
    pub fn pclk_qei(&mut self) -> _PCLK_QEIW {
        _PCLK_QEIW { w: self }
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline]
    pub fn pclk_gpioint(&mut self) -> _PCLK_GPIOINTW {
        _PCLK_GPIOINTW { w: self }
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline]
    pub fn pclk_pcb(&mut self) -> _PCLK_PCBW {
        _PCLK_PCBW { w: self }
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline]
    pub fn pclk_i2c1(&mut self) -> _PCLK_I2C1W {
        _PCLK_I2C1W { w: self }
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline]
    pub fn pclk_ssp0(&mut self) -> _PCLK_SSP0W {
        _PCLK_SSP0W { w: self }
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline]
    pub fn pclk_timer2(&mut self) -> _PCLK_TIMER2W {
        _PCLK_TIMER2W { w: self }
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline]
    pub fn pclk_timer3(&mut self) -> _PCLK_TIMER3W {
        _PCLK_TIMER3W { w: self }
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline]
    pub fn pclk_uart2(&mut self) -> _PCLK_UART2W {
        _PCLK_UART2W { w: self }
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline]
    pub fn pclk_uart3(&mut self) -> _PCLK_UART3W {
        _PCLK_UART3W { w: self }
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline]
    pub fn pclk_i2c2(&mut self) -> _PCLK_I2C2W {
        _PCLK_I2C2W { w: self }
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline]
    pub fn pclk_i2s(&mut self) -> _PCLK_I2SW {
        _PCLK_I2SW { w: self }
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline]
    pub fn pclk_rit(&mut self) -> _PCLK_RITW {
        _PCLK_RITW { w: self }
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline]
    pub fn pclk_syscon(&mut self) -> _PCLK_SYSCONW {
        _PCLK_SYSCONW { w: self }
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline]
    pub fn pclk_mc(&mut self) -> _PCLK_MCW {
        _PCLK_MCW { w: self }
    }
}
