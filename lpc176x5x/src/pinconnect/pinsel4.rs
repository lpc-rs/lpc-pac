#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL4 {
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
#[doc = "Possible values of the field `P2_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_0R {
    #[doc = "GPIO P2.0"]
    GPIO_P2,
    #[doc = "PWM1.1"]
    PWM1,
    #[doc = "TXD1"]
    TXD1,
}
impl P2_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_0R::GPIO_P2 => 0,
            P2_0R::PWM1 => 1,
            P2_0R::TXD1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_0R {
        match value {
            0 => P2_0R::GPIO_P2,
            1 => P2_0R::PWM1,
            2 => P2_0R::TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_0R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_0R::PWM1
    }
    #[doc = "Checks if the value of the field is `TXD1`"]
    #[inline]
    pub fn is_txd1(&self) -> bool {
        *self == P2_0R::TXD1
    }
}
#[doc = "Possible values of the field `P2_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_1R {
    #[doc = "GPIO P2.1"]
    GPIO_P2,
    #[doc = "PWM1.2"]
    PWM1,
    #[doc = "RXD1"]
    RXD1,
}
impl P2_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_1R::GPIO_P2 => 0,
            P2_1R::PWM1 => 1,
            P2_1R::RXD1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_1R {
        match value {
            0 => P2_1R::GPIO_P2,
            1 => P2_1R::PWM1,
            2 => P2_1R::RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_1R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_1R::PWM1
    }
    #[doc = "Checks if the value of the field is `RXD1`"]
    #[inline]
    pub fn is_rxd1(&self) -> bool {
        *self == P2_1R::RXD1
    }
}
#[doc = "Possible values of the field `P2_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_2R {
    #[doc = "GPIO P2.2"]
    GPIO_P2,
    #[doc = "PWM1.3"]
    PWM1,
    #[doc = "CTS1"]
    CTS1,
}
impl P2_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_2R::GPIO_P2 => 0,
            P2_2R::PWM1 => 1,
            P2_2R::CTS1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_2R {
        match value {
            0 => P2_2R::GPIO_P2,
            1 => P2_2R::PWM1,
            2 => P2_2R::CTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_2R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_2R::PWM1
    }
    #[doc = "Checks if the value of the field is `CTS1`"]
    #[inline]
    pub fn is_cts1(&self) -> bool {
        *self == P2_2R::CTS1
    }
}
#[doc = "Possible values of the field `P2_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_3R {
    #[doc = "GPIO P2.3."]
    GPIO_P2,
    #[doc = "PWM1.4"]
    PWM1,
    #[doc = "DCD1"]
    DCD1,
}
impl P2_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_3R::GPIO_P2 => 0,
            P2_3R::PWM1 => 1,
            P2_3R::DCD1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_3R {
        match value {
            0 => P2_3R::GPIO_P2,
            1 => P2_3R::PWM1,
            2 => P2_3R::DCD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_3R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_3R::PWM1
    }
    #[doc = "Checks if the value of the field is `DCD1`"]
    #[inline]
    pub fn is_dcd1(&self) -> bool {
        *self == P2_3R::DCD1
    }
}
#[doc = "Possible values of the field `P2_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_4R {
    #[doc = "GPIO P2.4."]
    GPIO_P2,
    #[doc = "PWM1.5"]
    PWM1,
    #[doc = "DSR1"]
    DSR1,
}
impl P2_4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_4R::GPIO_P2 => 0,
            P2_4R::PWM1 => 1,
            P2_4R::DSR1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_4R {
        match value {
            0 => P2_4R::GPIO_P2,
            1 => P2_4R::PWM1,
            2 => P2_4R::DSR1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_4R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_4R::PWM1
    }
    #[doc = "Checks if the value of the field is `DSR1`"]
    #[inline]
    pub fn is_dsr1(&self) -> bool {
        *self == P2_4R::DSR1
    }
}
#[doc = "Possible values of the field `P2_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_5R {
    #[doc = "GPIO P2.5."]
    GPIO_P2,
    #[doc = "PWM1.6"]
    PWM1,
    #[doc = "DTR1"]
    DTR1,
}
impl P2_5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_5R::GPIO_P2 => 0,
            P2_5R::PWM1 => 1,
            P2_5R::DTR1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_5R {
        match value {
            0 => P2_5R::GPIO_P2,
            1 => P2_5R::PWM1,
            2 => P2_5R::DTR1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_5R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_5R::PWM1
    }
    #[doc = "Checks if the value of the field is `DTR1`"]
    #[inline]
    pub fn is_dtr1(&self) -> bool {
        *self == P2_5R::DTR1
    }
}
#[doc = "Possible values of the field `P2_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_6R {
    #[doc = "GPIO P2.6."]
    GPIO_P2,
    #[doc = "PCAP1.0"]
    PCAP1,
    #[doc = "RI1"]
    RI1,
}
impl P2_6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_6R::GPIO_P2 => 0,
            P2_6R::PCAP1 => 1,
            P2_6R::RI1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_6R {
        match value {
            0 => P2_6R::GPIO_P2,
            1 => P2_6R::PCAP1,
            2 => P2_6R::RI1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_6R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline]
    pub fn is_pcap1(&self) -> bool {
        *self == P2_6R::PCAP1
    }
    #[doc = "Checks if the value of the field is `RI1`"]
    #[inline]
    pub fn is_ri1(&self) -> bool {
        *self == P2_6R::RI1
    }
}
#[doc = "Possible values of the field `P2_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_7R {
    #[doc = "GPIO P2.7."]
    GPIO_P2,
    #[doc = "RD2"]
    RD2,
    #[doc = "RTS1"]
    RTS1,
}
impl P2_7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_7R::GPIO_P2 => 0,
            P2_7R::RD2 => 1,
            P2_7R::RTS1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_7R {
        match value {
            0 => P2_7R::GPIO_P2,
            1 => P2_7R::RD2,
            2 => P2_7R::RTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_7R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `RD2`"]
    #[inline]
    pub fn is_rd2(&self) -> bool {
        *self == P2_7R::RD2
    }
    #[doc = "Checks if the value of the field is `RTS1`"]
    #[inline]
    pub fn is_rts1(&self) -> bool {
        *self == P2_7R::RTS1
    }
}
#[doc = "Possible values of the field `P2_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_8R {
    #[doc = "GPIO P2.8."]
    GPIO_P2,
    #[doc = "TD2"]
    TD2,
    #[doc = "TXD2"]
    TXD2,
    #[doc = "ENET_MDC"]
    ENET_MDC,
}
impl P2_8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_8R::GPIO_P2 => 0,
            P2_8R::TD2 => 1,
            P2_8R::TXD2 => 2,
            P2_8R::ENET_MDC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_8R {
        match value {
            0 => P2_8R::GPIO_P2,
            1 => P2_8R::TD2,
            2 => P2_8R::TXD2,
            3 => P2_8R::ENET_MDC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_8R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `TD2`"]
    #[inline]
    pub fn is_td2(&self) -> bool {
        *self == P2_8R::TD2
    }
    #[doc = "Checks if the value of the field is `TXD2`"]
    #[inline]
    pub fn is_txd2(&self) -> bool {
        *self == P2_8R::TXD2
    }
    #[doc = "Checks if the value of the field is `ENET_MDC`"]
    #[inline]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P2_8R::ENET_MDC
    }
}
#[doc = "Possible values of the field `P2_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_9R {
    #[doc = "GPIO P2.9"]
    GPIO_P2,
    #[doc = "USB_CONNECT"]
    USB_CONNECT,
    #[doc = "RXD2"]
    RXD2,
    #[doc = "ENET_MDIO"]
    ENET_MDIO,
}
impl P2_9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_9R::GPIO_P2 => 0,
            P2_9R::USB_CONNECT => 1,
            P2_9R::RXD2 => 2,
            P2_9R::ENET_MDIO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_9R {
        match value {
            0 => P2_9R::GPIO_P2,
            1 => P2_9R::USB_CONNECT,
            2 => P2_9R::RXD2,
            3 => P2_9R::ENET_MDIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_9R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `USB_CONNECT`"]
    #[inline]
    pub fn is_usb_connect(&self) -> bool {
        *self == P2_9R::USB_CONNECT
    }
    #[doc = "Checks if the value of the field is `RXD2`"]
    #[inline]
    pub fn is_rxd2(&self) -> bool {
        *self == P2_9R::RXD2
    }
    #[doc = "Checks if the value of the field is `ENET_MDIO`"]
    #[inline]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P2_9R::ENET_MDIO
    }
}
#[doc = "Possible values of the field `P2_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10R {
    #[doc = "GPIO P2.10"]
    GPIO_P2,
    #[doc = "EINT0"]
    EINT0,
    #[doc = "NMI"]
    NMI,
}
impl P2_10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_10R::GPIO_P2 => 0,
            P2_10R::EINT0 => 1,
            P2_10R::NMI => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_10R {
        match value {
            0 => P2_10R::GPIO_P2,
            1 => P2_10R::EINT0,
            2 => P2_10R::NMI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_10R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT0`"]
    #[inline]
    pub fn is_eint0(&self) -> bool {
        *self == P2_10R::EINT0
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline]
    pub fn is_nmi(&self) -> bool {
        *self == P2_10R::NMI
    }
}
#[doc = "Possible values of the field `P2_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11R {
    #[doc = "GPIO P2.11"]
    GPIO_P2,
    #[doc = "EINT1"]
    EINT1,
    #[doc = "I2STX_CLK"]
    I2STX_CLK,
}
impl P2_11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_11R::GPIO_P2 => 0,
            P2_11R::EINT1 => 1,
            P2_11R::I2STX_CLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_11R {
        match value {
            0 => P2_11R::GPIO_P2,
            1 => P2_11R::EINT1,
            3 => P2_11R::I2STX_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_11R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT1`"]
    #[inline]
    pub fn is_eint1(&self) -> bool {
        *self == P2_11R::EINT1
    }
    #[doc = "Checks if the value of the field is `I2STX_CLK`"]
    #[inline]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P2_11R::I2STX_CLK
    }
}
#[doc = "Possible values of the field `P2_12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12R {
    #[doc = "GPIO P2.12"]
    GPIO_P2,
    #[doc = "EINT2"]
    EINT2,
    #[doc = "I2STX_WS"]
    I2STX_WS,
}
impl P2_12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_12R::GPIO_P2 => 0,
            P2_12R::EINT2 => 1,
            P2_12R::I2STX_WS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_12R {
        match value {
            0 => P2_12R::GPIO_P2,
            1 => P2_12R::EINT2,
            3 => P2_12R::I2STX_WS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_12R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT2`"]
    #[inline]
    pub fn is_eint2(&self) -> bool {
        *self == P2_12R::EINT2
    }
    #[doc = "Checks if the value of the field is `I2STX_WS`"]
    #[inline]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P2_12R::I2STX_WS
    }
}
#[doc = "Possible values of the field `P2_13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13R {
    #[doc = "GPIO P2.13"]
    GPIO_P2,
    #[doc = "EINT3"]
    EINT3,
    #[doc = "I2STX_SDA"]
    I2STX_SDA,
}
impl P2_13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_13R::GPIO_P2 => 0,
            P2_13R::EINT3 => 1,
            P2_13R::I2STX_SDA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_13R {
        match value {
            0 => P2_13R::GPIO_P2,
            1 => P2_13R::EINT3,
            3 => P2_13R::I2STX_SDA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_13R::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT3`"]
    #[inline]
    pub fn is_eint3(&self) -> bool {
        *self == P2_13R::EINT3
    }
    #[doc = "Checks if the value of the field is `I2STX_SDA`"]
    #[inline]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P2_13R::I2STX_SDA
    }
}
#[doc = "Values that can be written to the field `P2_0`"]
pub enum P2_0W {
    #[doc = "GPIO P2.0"]
    GPIO_P2,
    #[doc = "PWM1.1"]
    PWM1,
    #[doc = "TXD1"]
    TXD1,
}
impl P2_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_0W::GPIO_P2 => 0,
            P2_0W::PWM1 => 1,
            P2_0W::TXD1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_0W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.0"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_0W::GPIO_P2)
    }
    #[doc = "PWM1.1"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_0W::PWM1)
    }
    #[doc = "TXD1"]
    #[inline]
    pub fn txd1(self) -> &'a mut W {
        self.variant(P2_0W::TXD1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_1`"]
pub enum P2_1W {
    #[doc = "GPIO P2.1"]
    GPIO_P2,
    #[doc = "PWM1.2"]
    PWM1,
    #[doc = "RXD1"]
    RXD1,
}
impl P2_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_1W::GPIO_P2 => 0,
            P2_1W::PWM1 => 1,
            P2_1W::RXD1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_1W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.1"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_1W::GPIO_P2)
    }
    #[doc = "PWM1.2"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_1W::PWM1)
    }
    #[doc = "RXD1"]
    #[inline]
    pub fn rxd1(self) -> &'a mut W {
        self.variant(P2_1W::RXD1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_2`"]
pub enum P2_2W {
    #[doc = "GPIO P2.2"]
    GPIO_P2,
    #[doc = "PWM1.3"]
    PWM1,
    #[doc = "CTS1"]
    CTS1,
}
impl P2_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_2W::GPIO_P2 => 0,
            P2_2W::PWM1 => 1,
            P2_2W::CTS1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_2W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.2"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_2W::GPIO_P2)
    }
    #[doc = "PWM1.3"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_2W::PWM1)
    }
    #[doc = "CTS1"]
    #[inline]
    pub fn cts1(self) -> &'a mut W {
        self.variant(P2_2W::CTS1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_3`"]
pub enum P2_3W {
    #[doc = "GPIO P2.3."]
    GPIO_P2,
    #[doc = "PWM1.4"]
    PWM1,
    #[doc = "DCD1"]
    DCD1,
}
impl P2_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_3W::GPIO_P2 => 0,
            P2_3W::PWM1 => 1,
            P2_3W::DCD1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_3W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.3."]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_3W::GPIO_P2)
    }
    #[doc = "PWM1.4"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_3W::PWM1)
    }
    #[doc = "DCD1"]
    #[inline]
    pub fn dcd1(self) -> &'a mut W {
        self.variant(P2_3W::DCD1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_4`"]
pub enum P2_4W {
    #[doc = "GPIO P2.4."]
    GPIO_P2,
    #[doc = "PWM1.5"]
    PWM1,
    #[doc = "DSR1"]
    DSR1,
}
impl P2_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_4W::GPIO_P2 => 0,
            P2_4W::PWM1 => 1,
            P2_4W::DSR1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_4W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.4."]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_4W::GPIO_P2)
    }
    #[doc = "PWM1.5"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_4W::PWM1)
    }
    #[doc = "DSR1"]
    #[inline]
    pub fn dsr1(self) -> &'a mut W {
        self.variant(P2_4W::DSR1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_5`"]
pub enum P2_5W {
    #[doc = "GPIO P2.5."]
    GPIO_P2,
    #[doc = "PWM1.6"]
    PWM1,
    #[doc = "DTR1"]
    DTR1,
}
impl P2_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_5W::GPIO_P2 => 0,
            P2_5W::PWM1 => 1,
            P2_5W::DTR1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_5W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.5."]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_5W::GPIO_P2)
    }
    #[doc = "PWM1.6"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_5W::PWM1)
    }
    #[doc = "DTR1"]
    #[inline]
    pub fn dtr1(self) -> &'a mut W {
        self.variant(P2_5W::DTR1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_6`"]
pub enum P2_6W {
    #[doc = "GPIO P2.6."]
    GPIO_P2,
    #[doc = "PCAP1.0"]
    PCAP1,
    #[doc = "RI1"]
    RI1,
}
impl P2_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_6W::GPIO_P2 => 0,
            P2_6W::PCAP1 => 1,
            P2_6W::RI1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_6W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.6."]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_6W::GPIO_P2)
    }
    #[doc = "PCAP1.0"]
    #[inline]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P2_6W::PCAP1)
    }
    #[doc = "RI1"]
    #[inline]
    pub fn ri1(self) -> &'a mut W {
        self.variant(P2_6W::RI1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_7`"]
pub enum P2_7W {
    #[doc = "GPIO P2.7."]
    GPIO_P2,
    #[doc = "RD2"]
    RD2,
    #[doc = "RTS1"]
    RTS1,
}
impl P2_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_7W::GPIO_P2 => 0,
            P2_7W::RD2 => 1,
            P2_7W::RTS1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_7W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.7."]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_7W::GPIO_P2)
    }
    #[doc = "RD2"]
    #[inline]
    pub fn rd2(self) -> &'a mut W {
        self.variant(P2_7W::RD2)
    }
    #[doc = "RTS1"]
    #[inline]
    pub fn rts1(self) -> &'a mut W {
        self.variant(P2_7W::RTS1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_8`"]
pub enum P2_8W {
    #[doc = "GPIO P2.8."]
    GPIO_P2,
    #[doc = "TD2"]
    TD2,
    #[doc = "TXD2"]
    TXD2,
    #[doc = "ENET_MDC"]
    ENET_MDC,
}
impl P2_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_8W::GPIO_P2 => 0,
            P2_8W::TD2 => 1,
            P2_8W::TXD2 => 2,
            P2_8W::ENET_MDC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_8W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P2.8."]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_8W::GPIO_P2)
    }
    #[doc = "TD2"]
    #[inline]
    pub fn td2(self) -> &'a mut W {
        self.variant(P2_8W::TD2)
    }
    #[doc = "TXD2"]
    #[inline]
    pub fn txd2(self) -> &'a mut W {
        self.variant(P2_8W::TXD2)
    }
    #[doc = "ENET_MDC"]
    #[inline]
    pub fn enet_mdc(self) -> &'a mut W {
        self.variant(P2_8W::ENET_MDC)
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
#[doc = "Values that can be written to the field `P2_9`"]
pub enum P2_9W {
    #[doc = "GPIO P2.9"]
    GPIO_P2,
    #[doc = "USB_CONNECT"]
    USB_CONNECT,
    #[doc = "RXD2"]
    RXD2,
    #[doc = "ENET_MDIO"]
    ENET_MDIO,
}
impl P2_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_9W::GPIO_P2 => 0,
            P2_9W::USB_CONNECT => 1,
            P2_9W::RXD2 => 2,
            P2_9W::ENET_MDIO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_9W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P2.9"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_9W::GPIO_P2)
    }
    #[doc = "USB_CONNECT"]
    #[inline]
    pub fn usb_connect(self) -> &'a mut W {
        self.variant(P2_9W::USB_CONNECT)
    }
    #[doc = "RXD2"]
    #[inline]
    pub fn rxd2(self) -> &'a mut W {
        self.variant(P2_9W::RXD2)
    }
    #[doc = "ENET_MDIO"]
    #[inline]
    pub fn enet_mdio(self) -> &'a mut W {
        self.variant(P2_9W::ENET_MDIO)
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
#[doc = "Values that can be written to the field `P2_10`"]
pub enum P2_10W {
    #[doc = "GPIO P2.10"]
    GPIO_P2,
    #[doc = "EINT0"]
    EINT0,
    #[doc = "NMI"]
    NMI,
}
impl P2_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_10W::GPIO_P2 => 0,
            P2_10W::EINT0 => 1,
            P2_10W::NMI => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_10W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.10"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_10W::GPIO_P2)
    }
    #[doc = "EINT0"]
    #[inline]
    pub fn eint0(self) -> &'a mut W {
        self.variant(P2_10W::EINT0)
    }
    #[doc = "NMI"]
    #[inline]
    pub fn nmi(self) -> &'a mut W {
        self.variant(P2_10W::NMI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_11`"]
pub enum P2_11W {
    #[doc = "GPIO P2.11"]
    GPIO_P2,
    #[doc = "EINT1"]
    EINT1,
    #[doc = "I2STX_CLK"]
    I2STX_CLK,
}
impl P2_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_11W::GPIO_P2 => 0,
            P2_11W::EINT1 => 1,
            P2_11W::I2STX_CLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_11W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.11"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_11W::GPIO_P2)
    }
    #[doc = "EINT1"]
    #[inline]
    pub fn eint1(self) -> &'a mut W {
        self.variant(P2_11W::EINT1)
    }
    #[doc = "I2STX_CLK"]
    #[inline]
    pub fn i2stx_clk(self) -> &'a mut W {
        self.variant(P2_11W::I2STX_CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_12`"]
pub enum P2_12W {
    #[doc = "GPIO P2.12"]
    GPIO_P2,
    #[doc = "EINT2"]
    EINT2,
    #[doc = "I2STX_WS"]
    I2STX_WS,
}
impl P2_12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_12W::GPIO_P2 => 0,
            P2_12W::EINT2 => 1,
            P2_12W::I2STX_WS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_12W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.12"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_12W::GPIO_P2)
    }
    #[doc = "EINT2"]
    #[inline]
    pub fn eint2(self) -> &'a mut W {
        self.variant(P2_12W::EINT2)
    }
    #[doc = "I2STX_WS"]
    #[inline]
    pub fn i2stx_ws(self) -> &'a mut W {
        self.variant(P2_12W::I2STX_WS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_13`"]
pub enum P2_13W {
    #[doc = "GPIO P2.13"]
    GPIO_P2,
    #[doc = "EINT3"]
    EINT3,
    #[doc = "I2STX_SDA"]
    I2STX_SDA,
}
impl P2_13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_13W::GPIO_P2 => 0,
            P2_13W::EINT3 => 1,
            P2_13W::I2STX_SDA => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_13W<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P2.13"]
    #[inline]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_13W::GPIO_P2)
    }
    #[doc = "EINT3"]
    #[inline]
    pub fn eint3(self) -> &'a mut W {
        self.variant(P2_13W::EINT3)
    }
    #[doc = "I2STX_SDA"]
    #[inline]
    pub fn i2stx_sda(self) -> &'a mut W {
        self.variant(P2_13W::I2STX_SDA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline]
    pub fn p2_0(&self) -> P2_0R {
        P2_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline]
    pub fn p2_1(&self) -> P2_1R {
        P2_1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline]
    pub fn p2_2(&self) -> P2_2R {
        P2_2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline]
    pub fn p2_3(&self) -> P2_3R {
        P2_3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline]
    pub fn p2_4(&self) -> P2_4R {
        P2_4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline]
    pub fn p2_5(&self) -> P2_5R {
        P2_5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline]
    pub fn p2_6(&self) -> P2_6R {
        P2_6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline]
    pub fn p2_7(&self) -> P2_7R {
        P2_7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline]
    pub fn p2_8(&self) -> P2_8R {
        P2_8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline]
    pub fn p2_9(&self) -> P2_9R {
        P2_9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline]
    pub fn p2_10(&self) -> P2_10R {
        P2_10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline]
    pub fn p2_11(&self) -> P2_11R {
        P2_11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline]
    pub fn p2_12(&self) -> P2_12R {
        P2_12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline]
    pub fn p2_13(&self) -> P2_13R {
        P2_13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline]
    pub fn p2_0(&mut self) -> _P2_0W {
        _P2_0W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline]
    pub fn p2_1(&mut self) -> _P2_1W {
        _P2_1W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline]
    pub fn p2_2(&mut self) -> _P2_2W {
        _P2_2W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline]
    pub fn p2_3(&mut self) -> _P2_3W {
        _P2_3W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline]
    pub fn p2_4(&mut self) -> _P2_4W {
        _P2_4W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline]
    pub fn p2_5(&mut self) -> _P2_5W {
        _P2_5W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline]
    pub fn p2_6(&mut self) -> _P2_6W {
        _P2_6W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline]
    pub fn p2_7(&mut self) -> _P2_7W {
        _P2_7W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline]
    pub fn p2_8(&mut self) -> _P2_8W {
        _P2_8W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline]
    pub fn p2_9(&mut self) -> _P2_9W {
        _P2_9W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline]
    pub fn p2_10(&mut self) -> _P2_10W {
        _P2_10W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline]
    pub fn p2_11(&mut self) -> _P2_11W {
        _P2_11W { w: self }
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline]
    pub fn p2_12(&mut self) -> _P2_12W {
        _P2_12W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline]
    pub fn p2_13(&mut self) -> _P2_13W {
        _P2_13W { w: self }
    }
}
