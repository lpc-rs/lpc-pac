#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCLKSEL0 {
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
#[doc = "Possible values of the field `PCLK_WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_WDTR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_WDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_WDTR::CCLK_DIV_4 => 0,
            PCLK_WDTR::CCLK => 1,
            PCLK_WDTR::CCLK_DIV_2 => 2,
            PCLK_WDTR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_WDTR {
        match value {
            0 => PCLK_WDTR::CCLK_DIV_4,
            1 => PCLK_WDTR::CCLK,
            2 => PCLK_WDTR::CCLK_DIV_2,
            3 => PCLK_WDTR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_WDTR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_WDTR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_WDTR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_WDTR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_TIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER0R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_TIMER0R::CCLK_DIV_4 => 0,
            PCLK_TIMER0R::CCLK => 1,
            PCLK_TIMER0R::CCLK_DIV_2 => 2,
            PCLK_TIMER0R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_TIMER0R {
        match value {
            0 => PCLK_TIMER0R::CCLK_DIV_4,
            1 => PCLK_TIMER0R::CCLK,
            2 => PCLK_TIMER0R::CCLK_DIV_2,
            3 => PCLK_TIMER0R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER0R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_TIMER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_TIMER1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_TIMER1R::CCLK_DIV_4 => 0,
            PCLK_TIMER1R::CCLK => 1,
            PCLK_TIMER1R::CCLK_DIV_2 => 2,
            PCLK_TIMER1R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_TIMER1R {
        match value {
            0 => PCLK_TIMER1R::CCLK_DIV_4,
            1 => PCLK_TIMER1R::CCLK,
            2 => PCLK_TIMER1R::CCLK_DIV_2,
            3 => PCLK_TIMER1R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER1R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_UART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART0R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_UART0R::CCLK_DIV_4 => 0,
            PCLK_UART0R::CCLK => 1,
            PCLK_UART0R::CCLK_DIV_2 => 2,
            PCLK_UART0R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_UART0R {
        match value {
            0 => PCLK_UART0R::CCLK_DIV_4,
            1 => PCLK_UART0R::CCLK,
            2 => PCLK_UART0R::CCLK_DIV_2,
            3 => PCLK_UART0R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART0R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART0R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART0R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART0R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_UART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_UART1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_UART1R::CCLK_DIV_4 => 0,
            PCLK_UART1R::CCLK => 1,
            PCLK_UART1R::CCLK_DIV_2 => 2,
            PCLK_UART1R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_UART1R {
        match value {
            0 => PCLK_UART1R::CCLK_DIV_4,
            1 => PCLK_UART1R::CCLK,
            2 => PCLK_UART1R::CCLK_DIV_2,
            3 => PCLK_UART1R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART1R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_PWM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_PWM1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_PWM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_PWM1R::CCLK_DIV_4 => 0,
            PCLK_PWM1R::CCLK => 1,
            PCLK_PWM1R::CCLK_DIV_2 => 2,
            PCLK_PWM1R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_PWM1R {
        match value {
            0 => PCLK_PWM1R::CCLK_DIV_4,
            1 => PCLK_PWM1R::CCLK,
            2 => PCLK_PWM1R::CCLK_DIV_2,
            3 => PCLK_PWM1R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_PWM1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_PWM1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_PWM1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_PWM1R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_I2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_I2C0R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2C0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_I2C0R::CCLK_DIV_4 => 0,
            PCLK_I2C0R::CCLK => 1,
            PCLK_I2C0R::CCLK_DIV_2 => 2,
            PCLK_I2C0R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_I2C0R {
        match value {
            0 => PCLK_I2C0R::CCLK_DIV_4,
            1 => PCLK_I2C0R::CCLK,
            2 => PCLK_I2C0R::CCLK_DIV_2,
            3 => PCLK_I2C0R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C0R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C0R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C0R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C0R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_SPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SPIR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SPIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_SPIR::CCLK_DIV_4 => 0,
            PCLK_SPIR::CCLK => 1,
            PCLK_SPIR::CCLK_DIV_2 => 2,
            PCLK_SPIR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_SPIR {
        match value {
            0 => PCLK_SPIR::CCLK_DIV_4,
            1 => PCLK_SPIR::CCLK,
            2 => PCLK_SPIR::CCLK_DIV_2,
            3 => PCLK_SPIR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SPIR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SPIR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SPIR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SPIR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_SSP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_SSP1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SSP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_SSP1R::CCLK_DIV_4 => 0,
            PCLK_SSP1R::CCLK => 1,
            PCLK_SSP1R::CCLK_DIV_2 => 2,
            PCLK_SSP1R::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_SSP1R {
        match value {
            0 => PCLK_SSP1R::CCLK_DIV_4,
            1 => PCLK_SSP1R::CCLK,
            2 => PCLK_SSP1R::CCLK_DIV_2,
            3 => PCLK_SSP1R::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SSP1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SSP1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SSP1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SSP1R::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_DAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_DACR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_DACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_DACR::CCLK_DIV_4 => 0,
            PCLK_DACR::CCLK => 1,
            PCLK_DACR::CCLK_DIV_2 => 2,
            PCLK_DACR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_DACR {
        match value {
            0 => PCLK_DACR::CCLK_DIV_4,
            1 => PCLK_DACR::CCLK,
            2 => PCLK_DACR::CCLK_DIV_2,
            3 => PCLK_DACR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_DACR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_DACR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_DACR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_DACR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_ADCR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_ADCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_ADCR::CCLK_DIV_4 => 0,
            PCLK_ADCR::CCLK => 1,
            PCLK_ADCR::CCLK_DIV_2 => 2,
            PCLK_ADCR::CCLK_DIV_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_ADCR {
        match value {
            0 => PCLK_ADCR::CCLK_DIV_4,
            1 => PCLK_ADCR::CCLK,
            2 => PCLK_ADCR::CCLK_DIV_2,
            3 => PCLK_ADCR::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ADCR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ADCR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ADCR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_8`"]
    #[inline]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_ADCR::CCLK_DIV_8
    }
}
#[doc = "Possible values of the field `PCLK_CAN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_CAN1R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    CCLK_DIV_6,
}
impl PCLK_CAN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_CAN1R::CCLK_DIV_4 => 0,
            PCLK_CAN1R::CCLK => 1,
            PCLK_CAN1R::CCLK_DIV_2 => 2,
            PCLK_CAN1R::CCLK_DIV_6 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_CAN1R {
        match value {
            0 => PCLK_CAN1R::CCLK_DIV_4,
            1 => PCLK_CAN1R::CCLK,
            2 => PCLK_CAN1R::CCLK_DIV_2,
            3 => PCLK_CAN1R::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN1R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN1R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN1R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN1R::CCLK_DIV_6
    }
}
#[doc = "Possible values of the field `PCLK_CAN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_CAN2R {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CCLK_DIV_6,
}
impl PCLK_CAN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_CAN2R::CCLK_DIV_4 => 0,
            PCLK_CAN2R::CCLK => 1,
            PCLK_CAN2R::CCLK_DIV_2 => 2,
            PCLK_CAN2R::CCLK_DIV_6 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_CAN2R {
        match value {
            0 => PCLK_CAN2R::CCLK_DIV_4,
            1 => PCLK_CAN2R::CCLK,
            2 => PCLK_CAN2R::CCLK_DIV_2,
            3 => PCLK_CAN2R::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN2R::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN2R::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN2R::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN2R::CCLK_DIV_6
    }
}
#[doc = "Possible values of the field `PCLK_ACF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK_ACFR {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    CCLK_DIV_6,
}
impl PCLK_ACFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCLK_ACFR::CCLK_DIV_4 => 0,
            PCLK_ACFR::CCLK => 1,
            PCLK_ACFR::CCLK_DIV_2 => 2,
            PCLK_ACFR::CCLK_DIV_6 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCLK_ACFR {
        match value {
            0 => PCLK_ACFR::CCLK_DIV_4,
            1 => PCLK_ACFR::CCLK,
            2 => PCLK_ACFR::CCLK_DIV_2,
            3 => PCLK_ACFR::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_4`"]
    #[inline]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ACFR::CCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ACFR::CCLK
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_2`"]
    #[inline]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ACFR::CCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CCLK_DIV_6`"]
    #[inline]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_ACFR::CCLK_DIV_6
    }
}
#[doc = "Values that can be written to the field `PCLK_WDT`"]
pub enum PCLK_WDTW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_WDTW::CCLK_DIV_4 => 0,
            PCLK_WDTW::CCLK => 1,
            PCLK_WDTW::CCLK_DIV_2 => 2,
            PCLK_WDTW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_WDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_WDTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_WDTW::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_TIMER0`"]
pub enum PCLK_TIMER0W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER0W::CCLK_DIV_4 => 0,
            PCLK_TIMER0W::CCLK => 1,
            PCLK_TIMER0W::CCLK_DIV_2 => 2,
            PCLK_TIMER0W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_TIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_TIMER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_TIMER0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER0W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_TIMER1`"]
pub enum PCLK_TIMER1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_TIMER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_TIMER1W::CCLK_DIV_4 => 0,
            PCLK_TIMER1W::CCLK => 1,
            PCLK_TIMER1W::CCLK_DIV_2 => 2,
            PCLK_TIMER1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_TIMER1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_TIMER1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_TIMER1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_TIMER1W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_UART0`"]
pub enum PCLK_UART0W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART0W::CCLK_DIV_4 => 0,
            PCLK_UART0W::CCLK => 1,
            PCLK_UART0W::CCLK_DIV_2 => 2,
            PCLK_UART0W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_UART0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_UART0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_UART0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART0W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_UART1`"]
pub enum PCLK_UART1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_UART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_UART1W::CCLK_DIV_4 => 0,
            PCLK_UART1W::CCLK => 1,
            PCLK_UART1W::CCLK_DIV_2 => 2,
            PCLK_UART1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_UART1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_UART1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_UART1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_UART1W::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_PWM1`"]
pub enum PCLK_PWM1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_PWM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_PWM1W::CCLK_DIV_4 => 0,
            PCLK_PWM1W::CCLK => 1,
            PCLK_PWM1W::CCLK_DIV_2 => 2,
            PCLK_PWM1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_PWM1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_PWM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_PWM1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_PWM1W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_I2C0`"]
pub enum PCLK_I2C0W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_I2C0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_I2C0W::CCLK_DIV_4 => 0,
            PCLK_I2C0W::CCLK => 1,
            PCLK_I2C0W::CCLK_DIV_2 => 2,
            PCLK_I2C0W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_I2C0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_I2C0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_I2C0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_I2C0W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_SPI`"]
pub enum PCLK_SPIW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SPIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_SPIW::CCLK_DIV_4 => 0,
            PCLK_SPIW::CCLK => 1,
            PCLK_SPIW::CCLK_DIV_2 => 2,
            PCLK_SPIW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_SPIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_SPIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_SPIW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SPIW::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_SSP1`"]
pub enum PCLK_SSP1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_SSP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_SSP1W::CCLK_DIV_4 => 0,
            PCLK_SSP1W::CCLK => 1,
            PCLK_SSP1W::CCLK_DIV_2 => 2,
            PCLK_SSP1W::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_SSP1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_SSP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_SSP1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_SSP1W::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_DAC`"]
pub enum PCLK_DACW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_DACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_DACW::CCLK_DIV_4 => 0,
            PCLK_DACW::CCLK => 1,
            PCLK_DACW::CCLK_DIV_2 => 2,
            PCLK_DACW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_DACW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_DACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_DACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_DACW::CCLK_DIV_8)
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
#[doc = "Values that can be written to the field `PCLK_ADC`"]
pub enum PCLK_ADCW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8,
}
impl PCLK_ADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_ADCW::CCLK_DIV_4 => 0,
            PCLK_ADCW::CCLK => 1,
            PCLK_ADCW::CCLK_DIV_2 => 2,
            PCLK_ADCW::CCLK_DIV_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_ADCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_ADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_ADCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline]
    pub fn cclk_div_8(self) -> &'a mut W {
        self.variant(PCLK_ADCW::CCLK_DIV_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCLK_CAN1`"]
pub enum PCLK_CAN1W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    CCLK_DIV_6,
}
impl PCLK_CAN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_CAN1W::CCLK_DIV_4 => 0,
            PCLK_CAN1W::CCLK => 1,
            PCLK_CAN1W::CCLK_DIV_2 => 2,
            PCLK_CAN1W::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_CAN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_CAN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_CAN1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_CAN1W::CCLK_DIV_6)
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
#[doc = "Values that can be written to the field `PCLK_CAN2`"]
pub enum PCLK_CAN2W {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CCLK_DIV_6,
}
impl PCLK_CAN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_CAN2W::CCLK_DIV_4 => 0,
            PCLK_CAN2W::CCLK => 1,
            PCLK_CAN2W::CCLK_DIV_2 => 2,
            PCLK_CAN2W::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_CAN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_CAN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_CAN2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_CAN2W::CCLK_DIV_6)
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
#[doc = "Values that can be written to the field `PCLK_ACF`"]
pub enum PCLK_ACFW {
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4,
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    CCLK,
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2,
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    CCLK_DIV_6,
}
impl PCLK_ACFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCLK_ACFW::CCLK_DIV_4 => 0,
            PCLK_ACFW::CCLK => 1,
            PCLK_ACFW::CCLK_DIV_2 => 2,
            PCLK_ACFW::CCLK_DIV_6 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCLK_ACFW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLK_ACFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCLK_ACFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline]
    pub fn cclk_div_4(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline]
    pub fn cclk(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline]
    pub fn cclk_div_2(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline]
    pub fn cclk_div_6(self) -> &'a mut W {
        self.variant(PCLK_ACFW::CCLK_DIV_6)
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
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline]
    pub fn pclk_wdt(&self) -> PCLK_WDTR {
        PCLK_WDTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline]
    pub fn pclk_timer0(&self) -> PCLK_TIMER0R {
        PCLK_TIMER0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline]
    pub fn pclk_timer1(&self) -> PCLK_TIMER1R {
        PCLK_TIMER1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline]
    pub fn pclk_uart0(&self) -> PCLK_UART0R {
        PCLK_UART0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline]
    pub fn pclk_uart1(&self) -> PCLK_UART1R {
        PCLK_UART1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline]
    pub fn pclk_pwm1(&self) -> PCLK_PWM1R {
        PCLK_PWM1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline]
    pub fn pclk_i2c0(&self) -> PCLK_I2C0R {
        PCLK_I2C0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline]
    pub fn pclk_spi(&self) -> PCLK_SPIR {
        PCLK_SPIR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline]
    pub fn pclk_ssp1(&self) -> PCLK_SSP1R {
        PCLK_SSP1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline]
    pub fn pclk_dac(&self) -> PCLK_DACR {
        PCLK_DACR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline]
    pub fn pclk_adc(&self) -> PCLK_ADCR {
        PCLK_ADCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline]
    pub fn pclk_can1(&self) -> PCLK_CAN1R {
        PCLK_CAN1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline]
    pub fn pclk_can2(&self) -> PCLK_CAN2R {
        PCLK_CAN2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline]
    pub fn pclk_acf(&self) -> PCLK_ACFR {
        PCLK_ACFR::_from({
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
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline]
    pub fn pclk_wdt(&mut self) -> _PCLK_WDTW {
        _PCLK_WDTW { w: self }
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline]
    pub fn pclk_timer0(&mut self) -> _PCLK_TIMER0W {
        _PCLK_TIMER0W { w: self }
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline]
    pub fn pclk_timer1(&mut self) -> _PCLK_TIMER1W {
        _PCLK_TIMER1W { w: self }
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline]
    pub fn pclk_uart0(&mut self) -> _PCLK_UART0W {
        _PCLK_UART0W { w: self }
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline]
    pub fn pclk_uart1(&mut self) -> _PCLK_UART1W {
        _PCLK_UART1W { w: self }
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline]
    pub fn pclk_pwm1(&mut self) -> _PCLK_PWM1W {
        _PCLK_PWM1W { w: self }
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline]
    pub fn pclk_i2c0(&mut self) -> _PCLK_I2C0W {
        _PCLK_I2C0W { w: self }
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline]
    pub fn pclk_spi(&mut self) -> _PCLK_SPIW {
        _PCLK_SPIW { w: self }
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline]
    pub fn pclk_ssp1(&mut self) -> _PCLK_SSP1W {
        _PCLK_SSP1W { w: self }
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline]
    pub fn pclk_dac(&mut self) -> _PCLK_DACW {
        _PCLK_DACW { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline]
    pub fn pclk_adc(&mut self) -> _PCLK_ADCW {
        _PCLK_ADCW { w: self }
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline]
    pub fn pclk_can1(&mut self) -> _PCLK_CAN1W {
        _PCLK_CAN1W { w: self }
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline]
    pub fn pclk_can2(&mut self) -> _PCLK_CAN2W {
        _PCLK_CAN2W { w: self }
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline]
    pub fn pclk_acf(&mut self) -> _PCLK_ACFW {
        _PCLK_ACFW { w: self }
    }
}
