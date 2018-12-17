#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL3 {
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
#[doc = "Possible values of the field `P1_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_16R {
    #[doc = "GPIO P1.16"]
    GPIO_P1,
    #[doc = "ENET_MDC"]
    ENET_MDC,
}
impl P1_16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_16R::GPIO_P1 => 0,
            P1_16R::ENET_MDC => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_16R {
        match value {
            0 => P1_16R::GPIO_P1,
            1 => P1_16R::ENET_MDC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_16R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_MDC`"]
    #[inline]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P1_16R::ENET_MDC
    }
}
#[doc = "Possible values of the field `P1_17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_17R {
    #[doc = "GPIO P1.17"]
    GPIO_P1,
    #[doc = "ENET_MDIO"]
    ENET_MDIO,
}
impl P1_17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_17R::GPIO_P1 => 0,
            P1_17R::ENET_MDIO => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_17R {
        match value {
            0 => P1_17R::GPIO_P1,
            1 => P1_17R::ENET_MDIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_17R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_MDIO`"]
    #[inline]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P1_17R::ENET_MDIO
    }
}
#[doc = "Possible values of the field `P1_18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_18R {
    #[doc = "GPIO P1.18"]
    GPIO_P1,
    #[doc = "USB_UP_LED"]
    USB_UP_LED,
    #[doc = "PWM1.1"]
    PWM1,
    #[doc = "CAP1.0"]
    CAP1,
}
impl P1_18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_18R::GPIO_P1 => 0,
            P1_18R::USB_UP_LED => 1,
            P1_18R::PWM1 => 2,
            P1_18R::CAP1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_18R {
        match value {
            0 => P1_18R::GPIO_P1,
            1 => P1_18R::USB_UP_LED,
            2 => P1_18R::PWM1,
            3 => P1_18R::CAP1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_18R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `USB_UP_LED`"]
    #[inline]
    pub fn is_usb_up_led(&self) -> bool {
        *self == P1_18R::USB_UP_LED
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_18R::PWM1
    }
    #[doc = "Checks if the value of the field is `CAP1`"]
    #[inline]
    pub fn is_cap1(&self) -> bool {
        *self == P1_18R::CAP1
    }
}
#[doc = "Possible values of the field `P1_19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_19R {
    #[doc = "GPIO P1.19."]
    GPIO_P1,
    #[doc = "MCOA0"]
    MCOA0,
    #[doc = "USB_PPWR"]
    USB_PPWR,
    #[doc = "CAP1.1"]
    CAP1,
}
impl P1_19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_19R::GPIO_P1 => 0,
            P1_19R::MCOA0 => 1,
            P1_19R::USB_PPWR => 2,
            P1_19R::CAP1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_19R {
        match value {
            0 => P1_19R::GPIO_P1,
            1 => P1_19R::MCOA0,
            2 => P1_19R::USB_PPWR,
            3 => P1_19R::CAP1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_19R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA0`"]
    #[inline]
    pub fn is_mcoa0(&self) -> bool {
        *self == P1_19R::MCOA0
    }
    #[doc = "Checks if the value of the field is `USB_PPWR`"]
    #[inline]
    pub fn is_usb_ppwr(&self) -> bool {
        *self == P1_19R::USB_PPWR
    }
    #[doc = "Checks if the value of the field is `CAP1`"]
    #[inline]
    pub fn is_cap1(&self) -> bool {
        *self == P1_19R::CAP1
    }
}
#[doc = "Possible values of the field `P1_20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_20R {
    #[doc = "GPIO P1.20."]
    GPIO_P1,
    #[doc = "MCI0"]
    MCI0,
    #[doc = "PWM1.2"]
    PWM1,
    #[doc = "SCK0"]
    SCK0,
}
impl P1_20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_20R::GPIO_P1 => 0,
            P1_20R::MCI0 => 1,
            P1_20R::PWM1 => 2,
            P1_20R::SCK0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_20R {
        match value {
            0 => P1_20R::GPIO_P1,
            1 => P1_20R::MCI0,
            2 => P1_20R::PWM1,
            3 => P1_20R::SCK0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_20R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI0`"]
    #[inline]
    pub fn is_mci0(&self) -> bool {
        *self == P1_20R::MCI0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_20R::PWM1
    }
    #[doc = "Checks if the value of the field is `SCK0`"]
    #[inline]
    pub fn is_sck0(&self) -> bool {
        *self == P1_20R::SCK0
    }
}
#[doc = "Possible values of the field `P1_21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_21R {
    #[doc = "GPIO P1.21."]
    GPIO_P1,
    #[doc = "MCABORT"]
    MCABORT,
    #[doc = "PWM1.3"]
    PWM1,
    #[doc = "SSEL0"]
    SSEL0,
}
impl P1_21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_21R::GPIO_P1 => 0,
            P1_21R::MCABORT => 1,
            P1_21R::PWM1 => 2,
            P1_21R::SSEL0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_21R {
        match value {
            0 => P1_21R::GPIO_P1,
            1 => P1_21R::MCABORT,
            2 => P1_21R::PWM1,
            3 => P1_21R::SSEL0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_21R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCABORT`"]
    #[inline]
    pub fn is_mcabort(&self) -> bool {
        *self == P1_21R::MCABORT
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_21R::PWM1
    }
    #[doc = "Checks if the value of the field is `SSEL0`"]
    #[inline]
    pub fn is_ssel0(&self) -> bool {
        *self == P1_21R::SSEL0
    }
}
#[doc = "Possible values of the field `P1_22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_22R {
    #[doc = "GPIO P1.22."]
    GPIO_P1,
    #[doc = "MCOB0"]
    MCOB0,
    #[doc = "USB_PWRD"]
    USB_PWRD,
    #[doc = "MAT1.0"]
    MAT1,
}
impl P1_22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_22R::GPIO_P1 => 0,
            P1_22R::MCOB0 => 1,
            P1_22R::USB_PWRD => 2,
            P1_22R::MAT1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_22R {
        match value {
            0 => P1_22R::GPIO_P1,
            1 => P1_22R::MCOB0,
            2 => P1_22R::USB_PWRD,
            3 => P1_22R::MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_22R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB0`"]
    #[inline]
    pub fn is_mcob0(&self) -> bool {
        *self == P1_22R::MCOB0
    }
    #[doc = "Checks if the value of the field is `USB_PWRD`"]
    #[inline]
    pub fn is_usb_pwrd(&self) -> bool {
        *self == P1_22R::USB_PWRD
    }
    #[doc = "Checks if the value of the field is `MAT1`"]
    #[inline]
    pub fn is_mat1(&self) -> bool {
        *self == P1_22R::MAT1
    }
}
#[doc = "Possible values of the field `P1_23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_23R {
    #[doc = "GPIO P1.23."]
    GPIO_P1,
    #[doc = "MCI1"]
    MCI1,
    #[doc = "PWM1.4"]
    PWM1,
    #[doc = "MISO0"]
    MISO0,
}
impl P1_23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_23R::GPIO_P1 => 0,
            P1_23R::MCI1 => 1,
            P1_23R::PWM1 => 2,
            P1_23R::MISO0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_23R {
        match value {
            0 => P1_23R::GPIO_P1,
            1 => P1_23R::MCI1,
            2 => P1_23R::PWM1,
            3 => P1_23R::MISO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_23R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI1`"]
    #[inline]
    pub fn is_mci1(&self) -> bool {
        *self == P1_23R::MCI1
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_23R::PWM1
    }
    #[doc = "Checks if the value of the field is `MISO0`"]
    #[inline]
    pub fn is_miso0(&self) -> bool {
        *self == P1_23R::MISO0
    }
}
#[doc = "Possible values of the field `P1_24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_24R {
    #[doc = "GPIO P1.24."]
    GPIO_P1,
    #[doc = "MCI2"]
    MCI2,
    #[doc = "PWM1.5"]
    PWM1,
    #[doc = "MOSI0"]
    MOSI0,
}
impl P1_24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_24R::GPIO_P1 => 0,
            P1_24R::MCI2 => 1,
            P1_24R::PWM1 => 2,
            P1_24R::MOSI0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_24R {
        match value {
            0 => P1_24R::GPIO_P1,
            1 => P1_24R::MCI2,
            2 => P1_24R::PWM1,
            3 => P1_24R::MOSI0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_24R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI2`"]
    #[inline]
    pub fn is_mci2(&self) -> bool {
        *self == P1_24R::MCI2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_24R::PWM1
    }
    #[doc = "Checks if the value of the field is `MOSI0`"]
    #[inline]
    pub fn is_mosi0(&self) -> bool {
        *self == P1_24R::MOSI0
    }
}
#[doc = "Possible values of the field `P1_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_25R {
    #[doc = "GPIO P1.25"]
    GPIO_P1,
    #[doc = "MCOA1"]
    MCOA1,
    #[doc = "MAT1.1"]
    MAT1,
}
impl P1_25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_25R::GPIO_P1 => 0,
            P1_25R::MCOA1 => 1,
            P1_25R::MAT1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_25R {
        match value {
            0 => P1_25R::GPIO_P1,
            1 => P1_25R::MCOA1,
            3 => P1_25R::MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_25R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA1`"]
    #[inline]
    pub fn is_mcoa1(&self) -> bool {
        *self == P1_25R::MCOA1
    }
    #[doc = "Checks if the value of the field is `MAT1`"]
    #[inline]
    pub fn is_mat1(&self) -> bool {
        *self == P1_25R::MAT1
    }
}
#[doc = "Possible values of the field `P1_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_26R {
    #[doc = "GPIO P1.26"]
    GPIO_P1,
    #[doc = "MCOB1"]
    MCOB1,
    #[doc = "PWM1.6"]
    PWM1,
    #[doc = "CAP0.0"]
    CAP0,
}
impl P1_26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_26R::GPIO_P1 => 0,
            P1_26R::MCOB1 => 1,
            P1_26R::PWM1 => 2,
            P1_26R::CAP0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_26R {
        match value {
            0 => P1_26R::GPIO_P1,
            1 => P1_26R::MCOB1,
            2 => P1_26R::PWM1,
            3 => P1_26R::CAP0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_26R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB1`"]
    #[inline]
    pub fn is_mcob1(&self) -> bool {
        *self == P1_26R::MCOB1
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_26R::PWM1
    }
    #[doc = "Checks if the value of the field is `CAP0`"]
    #[inline]
    pub fn is_cap0(&self) -> bool {
        *self == P1_26R::CAP0
    }
}
#[doc = "Possible values of the field `P1_27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_27R {
    #[doc = "GPIO P1.27"]
    GPIO_P1,
    #[doc = "CLKOUT"]
    CLKOUT,
    #[doc = "USB_OVRCR"]
    USB_OVRCR,
    #[doc = "CAP0.1"]
    CAP0,
}
impl P1_27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_27R::GPIO_P1 => 0,
            P1_27R::CLKOUT => 1,
            P1_27R::USB_OVRCR => 2,
            P1_27R::CAP0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_27R {
        match value {
            0 => P1_27R::GPIO_P1,
            1 => P1_27R::CLKOUT,
            2 => P1_27R::USB_OVRCR,
            3 => P1_27R::CAP0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_27R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline]
    pub fn is_clkout(&self) -> bool {
        *self == P1_27R::CLKOUT
    }
    #[doc = "Checks if the value of the field is `USB_OVRCR`"]
    #[inline]
    pub fn is_usb_ovrcr(&self) -> bool {
        *self == P1_27R::USB_OVRCR
    }
    #[doc = "Checks if the value of the field is `CAP0`"]
    #[inline]
    pub fn is_cap0(&self) -> bool {
        *self == P1_27R::CAP0
    }
}
#[doc = "Possible values of the field `P1_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_28R {
    #[doc = "GPIO P1.28"]
    GPIO_P1,
    #[doc = "MCOA2"]
    MCOA2,
    #[doc = "PCAP1.0"]
    PCAP1,
    #[doc = "MAT0.0"]
    MAT0,
}
impl P1_28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_28R::GPIO_P1 => 0,
            P1_28R::MCOA2 => 1,
            P1_28R::PCAP1 => 2,
            P1_28R::MAT0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_28R {
        match value {
            0 => P1_28R::GPIO_P1,
            1 => P1_28R::MCOA2,
            2 => P1_28R::PCAP1,
            3 => P1_28R::MAT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_28R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA2`"]
    #[inline]
    pub fn is_mcoa2(&self) -> bool {
        *self == P1_28R::MCOA2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_28R::PCAP1
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline]
    pub fn is_mat0(&self) -> bool {
        *self == P1_28R::MAT0
    }
}
#[doc = "Possible values of the field `P1_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_29R {
    #[doc = "GPIO P1.29"]
    GPIO_P1,
    #[doc = "MCOB2"]
    MCOB2,
    #[doc = "PCAP1.1"]
    PCAP1,
    #[doc = "MAT0.1"]
    MAT0,
}
impl P1_29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_29R::GPIO_P1 => 0,
            P1_29R::MCOB2 => 1,
            P1_29R::PCAP1 => 2,
            P1_29R::MAT0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_29R {
        match value {
            0 => P1_29R::GPIO_P1,
            1 => P1_29R::MCOB2,
            2 => P1_29R::PCAP1,
            3 => P1_29R::MAT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_29R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB2`"]
    #[inline]
    pub fn is_mcob2(&self) -> bool {
        *self == P1_29R::MCOB2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_29R::PCAP1
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline]
    pub fn is_mat0(&self) -> bool {
        *self == P1_29R::MAT0
    }
}
#[doc = "Possible values of the field `P1_30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_30R {
    #[doc = "GPIO P1.30"]
    GPIO_P1,
    #[doc = "VBUS"]
    VBUS,
    #[doc = "AD0.4"]
    AD0,
}
impl P1_30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_30R::GPIO_P1 => 0,
            P1_30R::VBUS => 2,
            P1_30R::AD0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_30R {
        match value {
            0 => P1_30R::GPIO_P1,
            2 => P1_30R::VBUS,
            3 => P1_30R::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_30R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `VBUS`"]
    #[inline]
    pub fn is_vbus(&self) -> bool {
        *self == P1_30R::VBUS
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline]
    pub fn is_ad0(&self) -> bool {
        *self == P1_30R::AD0
    }
}
#[doc = "Possible values of the field `P1_31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_31R {
    #[doc = "GPIO Port 1.31"]
    GPIO_PORT_1,
    #[doc = "SCK1"]
    SCK1,
    #[doc = "AD0.5"]
    AD0,
}
impl P1_31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_31R::GPIO_PORT_1 => 0,
            P1_31R::SCK1 => 2,
            P1_31R::AD0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_31R {
        match value {
            0 => P1_31R::GPIO_PORT_1,
            2 => P1_31R::SCK1,
            3 => P1_31R::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_31R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `SCK1`"]
    #[inline]
    pub fn is_sck1(&self) -> bool {
        *self == P1_31R::SCK1
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline]
    pub fn is_ad0(&self) -> bool {
        *self == P1_31R::AD0
    }
}
#[doc = "Values that can be written to the field `P1_16`"]
pub enum P1_16W {
    #[doc = "GPIO P1.16"]
    GPIO_P1,
    #[doc = "ENET_MDC"]
    ENET_MDC,
}
impl P1_16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_16W::GPIO_P1 => 0,
            P1_16W::ENET_MDC => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_16W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_16W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.16"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_16W::GPIO_P1)
    }
    #[doc = "ENET_MDC"]
    #[inline]
    pub fn enet_mdc(self) -> &'a mut W {
        self.variant(P1_16W::ENET_MDC)
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
#[doc = "Values that can be written to the field `P1_17`"]
pub enum P1_17W {
    #[doc = "GPIO P1.17"]
    GPIO_P1,
    #[doc = "ENET_MDIO"]
    ENET_MDIO,
}
impl P1_17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_17W::GPIO_P1 => 0,
            P1_17W::ENET_MDIO => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_17W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_17W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.17"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_17W::GPIO_P1)
    }
    #[doc = "ENET_MDIO"]
    #[inline]
    pub fn enet_mdio(self) -> &'a mut W {
        self.variant(P1_17W::ENET_MDIO)
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
#[doc = "Values that can be written to the field `P1_18`"]
pub enum P1_18W {
    #[doc = "GPIO P1.18"]
    GPIO_P1,
    #[doc = "USB_UP_LED"]
    USB_UP_LED,
    #[doc = "PWM1.1"]
    PWM1,
    #[doc = "CAP1.0"]
    CAP1,
}
impl P1_18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_18W::GPIO_P1 => 0,
            P1_18W::USB_UP_LED => 1,
            P1_18W::PWM1 => 2,
            P1_18W::CAP1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_18W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_18W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.18"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_18W::GPIO_P1)
    }
    #[doc = "USB_UP_LED"]
    #[inline]
    pub fn usb_up_led(self) -> &'a mut W {
        self.variant(P1_18W::USB_UP_LED)
    }
    #[doc = "PWM1.1"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_18W::PWM1)
    }
    #[doc = "CAP1.0"]
    #[inline]
    pub fn cap1(self) -> &'a mut W {
        self.variant(P1_18W::CAP1)
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
#[doc = "Values that can be written to the field `P1_19`"]
pub enum P1_19W {
    #[doc = "GPIO P1.19."]
    GPIO_P1,
    #[doc = "MCOA0"]
    MCOA0,
    #[doc = "USB_PPWR"]
    USB_PPWR,
    #[doc = "CAP1.1"]
    CAP1,
}
impl P1_19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_19W::GPIO_P1 => 0,
            P1_19W::MCOA0 => 1,
            P1_19W::USB_PPWR => 2,
            P1_19W::CAP1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_19W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_19W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.19."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_19W::GPIO_P1)
    }
    #[doc = "MCOA0"]
    #[inline]
    pub fn mcoa0(self) -> &'a mut W {
        self.variant(P1_19W::MCOA0)
    }
    #[doc = "USB_PPWR"]
    #[inline]
    pub fn usb_ppwr(self) -> &'a mut W {
        self.variant(P1_19W::USB_PPWR)
    }
    #[doc = "CAP1.1"]
    #[inline]
    pub fn cap1(self) -> &'a mut W {
        self.variant(P1_19W::CAP1)
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
#[doc = "Values that can be written to the field `P1_20`"]
pub enum P1_20W {
    #[doc = "GPIO P1.20."]
    GPIO_P1,
    #[doc = "MCI0"]
    MCI0,
    #[doc = "PWM1.2"]
    PWM1,
    #[doc = "SCK0"]
    SCK0,
}
impl P1_20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_20W::GPIO_P1 => 0,
            P1_20W::MCI0 => 1,
            P1_20W::PWM1 => 2,
            P1_20W::SCK0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_20W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_20W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.20."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_20W::GPIO_P1)
    }
    #[doc = "MCI0"]
    #[inline]
    pub fn mci0(self) -> &'a mut W {
        self.variant(P1_20W::MCI0)
    }
    #[doc = "PWM1.2"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_20W::PWM1)
    }
    #[doc = "SCK0"]
    #[inline]
    pub fn sck0(self) -> &'a mut W {
        self.variant(P1_20W::SCK0)
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
#[doc = "Values that can be written to the field `P1_21`"]
pub enum P1_21W {
    #[doc = "GPIO P1.21."]
    GPIO_P1,
    #[doc = "MCABORT"]
    MCABORT,
    #[doc = "PWM1.3"]
    PWM1,
    #[doc = "SSEL0"]
    SSEL0,
}
impl P1_21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_21W::GPIO_P1 => 0,
            P1_21W::MCABORT => 1,
            P1_21W::PWM1 => 2,
            P1_21W::SSEL0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_21W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_21W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.21."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_21W::GPIO_P1)
    }
    #[doc = "MCABORT"]
    #[inline]
    pub fn mcabort(self) -> &'a mut W {
        self.variant(P1_21W::MCABORT)
    }
    #[doc = "PWM1.3"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_21W::PWM1)
    }
    #[doc = "SSEL0"]
    #[inline]
    pub fn ssel0(self) -> &'a mut W {
        self.variant(P1_21W::SSEL0)
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
#[doc = "Values that can be written to the field `P1_22`"]
pub enum P1_22W {
    #[doc = "GPIO P1.22."]
    GPIO_P1,
    #[doc = "MCOB0"]
    MCOB0,
    #[doc = "USB_PWRD"]
    USB_PWRD,
    #[doc = "MAT1.0"]
    MAT1,
}
impl P1_22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_22W::GPIO_P1 => 0,
            P1_22W::MCOB0 => 1,
            P1_22W::USB_PWRD => 2,
            P1_22W::MAT1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_22W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_22W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.22."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_22W::GPIO_P1)
    }
    #[doc = "MCOB0"]
    #[inline]
    pub fn mcob0(self) -> &'a mut W {
        self.variant(P1_22W::MCOB0)
    }
    #[doc = "USB_PWRD"]
    #[inline]
    pub fn usb_pwrd(self) -> &'a mut W {
        self.variant(P1_22W::USB_PWRD)
    }
    #[doc = "MAT1.0"]
    #[inline]
    pub fn mat1(self) -> &'a mut W {
        self.variant(P1_22W::MAT1)
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
#[doc = "Values that can be written to the field `P1_23`"]
pub enum P1_23W {
    #[doc = "GPIO P1.23."]
    GPIO_P1,
    #[doc = "MCI1"]
    MCI1,
    #[doc = "PWM1.4"]
    PWM1,
    #[doc = "MISO0"]
    MISO0,
}
impl P1_23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_23W::GPIO_P1 => 0,
            P1_23W::MCI1 => 1,
            P1_23W::PWM1 => 2,
            P1_23W::MISO0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_23W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.23."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_23W::GPIO_P1)
    }
    #[doc = "MCI1"]
    #[inline]
    pub fn mci1(self) -> &'a mut W {
        self.variant(P1_23W::MCI1)
    }
    #[doc = "PWM1.4"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_23W::PWM1)
    }
    #[doc = "MISO0"]
    #[inline]
    pub fn miso0(self) -> &'a mut W {
        self.variant(P1_23W::MISO0)
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
#[doc = "Values that can be written to the field `P1_24`"]
pub enum P1_24W {
    #[doc = "GPIO P1.24."]
    GPIO_P1,
    #[doc = "MCI2"]
    MCI2,
    #[doc = "PWM1.5"]
    PWM1,
    #[doc = "MOSI0"]
    MOSI0,
}
impl P1_24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_24W::GPIO_P1 => 0,
            P1_24W::MCI2 => 1,
            P1_24W::PWM1 => 2,
            P1_24W::MOSI0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_24W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_24W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.24."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_24W::GPIO_P1)
    }
    #[doc = "MCI2"]
    #[inline]
    pub fn mci2(self) -> &'a mut W {
        self.variant(P1_24W::MCI2)
    }
    #[doc = "PWM1.5"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_24W::PWM1)
    }
    #[doc = "MOSI0"]
    #[inline]
    pub fn mosi0(self) -> &'a mut W {
        self.variant(P1_24W::MOSI0)
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
#[doc = "Values that can be written to the field `P1_25`"]
pub enum P1_25W {
    #[doc = "GPIO P1.25"]
    GPIO_P1,
    #[doc = "MCOA1"]
    MCOA1,
    #[doc = "MAT1.1"]
    MAT1,
}
impl P1_25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_25W::GPIO_P1 => 0,
            P1_25W::MCOA1 => 1,
            P1_25W::MAT1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_25W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_25W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.25"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_25W::GPIO_P1)
    }
    #[doc = "MCOA1"]
    #[inline]
    pub fn mcoa1(self) -> &'a mut W {
        self.variant(P1_25W::MCOA1)
    }
    #[doc = "MAT1.1"]
    #[inline]
    pub fn mat1(self) -> &'a mut W {
        self.variant(P1_25W::MAT1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_26`"]
pub enum P1_26W {
    #[doc = "GPIO P1.26"]
    GPIO_P1,
    #[doc = "MCOB1"]
    MCOB1,
    #[doc = "PWM1.6"]
    PWM1,
    #[doc = "CAP0.0"]
    CAP0,
}
impl P1_26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_26W::GPIO_P1 => 0,
            P1_26W::MCOB1 => 1,
            P1_26W::PWM1 => 2,
            P1_26W::CAP0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_26W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.26"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_26W::GPIO_P1)
    }
    #[doc = "MCOB1"]
    #[inline]
    pub fn mcob1(self) -> &'a mut W {
        self.variant(P1_26W::MCOB1)
    }
    #[doc = "PWM1.6"]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_26W::PWM1)
    }
    #[doc = "CAP0.0"]
    #[inline]
    pub fn cap0(self) -> &'a mut W {
        self.variant(P1_26W::CAP0)
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
#[doc = "Values that can be written to the field `P1_27`"]
pub enum P1_27W {
    #[doc = "GPIO P1.27"]
    GPIO_P1,
    #[doc = "CLKOUT"]
    CLKOUT,
    #[doc = "USB_OVRCR"]
    USB_OVRCR,
    #[doc = "CAP0.1"]
    CAP0,
}
impl P1_27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_27W::GPIO_P1 => 0,
            P1_27W::CLKOUT => 1,
            P1_27W::USB_OVRCR => 2,
            P1_27W::CAP0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_27W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_27W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.27"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_27W::GPIO_P1)
    }
    #[doc = "CLKOUT"]
    #[inline]
    pub fn clkout(self) -> &'a mut W {
        self.variant(P1_27W::CLKOUT)
    }
    #[doc = "USB_OVRCR"]
    #[inline]
    pub fn usb_ovrcr(self) -> &'a mut W {
        self.variant(P1_27W::USB_OVRCR)
    }
    #[doc = "CAP0.1"]
    #[inline]
    pub fn cap0(self) -> &'a mut W {
        self.variant(P1_27W::CAP0)
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
#[doc = "Values that can be written to the field `P1_28`"]
pub enum P1_28W {
    #[doc = "GPIO P1.28"]
    GPIO_P1,
    #[doc = "MCOA2"]
    MCOA2,
    #[doc = "PCAP1.0"]
    PCAP1,
    #[doc = "MAT0.0"]
    MAT0,
}
impl P1_28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_28W::GPIO_P1 => 0,
            P1_28W::MCOA2 => 1,
            P1_28W::PCAP1 => 2,
            P1_28W::MAT0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_28W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_28W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.28"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_28W::GPIO_P1)
    }
    #[doc = "MCOA2"]
    #[inline]
    pub fn mcoa2(self) -> &'a mut W {
        self.variant(P1_28W::MCOA2)
    }
    #[doc = "PCAP1.0"]
    #[inline]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P1_28W::PCAP1)
    }
    #[doc = "MAT0.0"]
    #[inline]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P1_28W::MAT0)
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
#[doc = "Values that can be written to the field `P1_29`"]
pub enum P1_29W {
    #[doc = "GPIO P1.29"]
    GPIO_P1,
    #[doc = "MCOB2"]
    MCOB2,
    #[doc = "PCAP1.1"]
    PCAP1,
    #[doc = "MAT0.1"]
    MAT0,
}
impl P1_29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_29W::GPIO_P1 => 0,
            P1_29W::MCOB2 => 1,
            P1_29W::PCAP1 => 2,
            P1_29W::MAT0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_29W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_29W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P1.29"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_29W::GPIO_P1)
    }
    #[doc = "MCOB2"]
    #[inline]
    pub fn mcob2(self) -> &'a mut W {
        self.variant(P1_29W::MCOB2)
    }
    #[doc = "PCAP1.1"]
    #[inline]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P1_29W::PCAP1)
    }
    #[doc = "MAT0.1"]
    #[inline]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P1_29W::MAT0)
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
#[doc = "Values that can be written to the field `P1_30`"]
pub enum P1_30W {
    #[doc = "GPIO P1.30"]
    GPIO_P1,
    #[doc = "VBUS"]
    VBUS,
    #[doc = "AD0.4"]
    AD0,
}
impl P1_30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_30W::GPIO_P1 => 0,
            P1_30W::VBUS => 2,
            P1_30W::AD0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_30W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_30W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.30"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_30W::GPIO_P1)
    }
    #[doc = "VBUS"]
    #[inline]
    pub fn vbus(self) -> &'a mut W {
        self.variant(P1_30W::VBUS)
    }
    #[doc = "AD0.4"]
    #[inline]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P1_30W::AD0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_31`"]
pub enum P1_31W {
    #[doc = "GPIO Port 1.31"]
    GPIO_PORT_1,
    #[doc = "SCK1"]
    SCK1,
    #[doc = "AD0.5"]
    AD0,
}
impl P1_31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_31W::GPIO_PORT_1 => 0,
            P1_31W::SCK1 => 2,
            P1_31W::AD0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_31W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_31W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO Port 1.31"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(P1_31W::GPIO_PORT_1)
    }
    #[doc = "SCK1"]
    #[inline]
    pub fn sck1(self) -> &'a mut W {
        self.variant(P1_31W::SCK1)
    }
    #[doc = "AD0.5"]
    #[inline]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P1_31W::AD0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline]
    pub fn p1_16(&self) -> P1_16R {
        P1_16R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline]
    pub fn p1_17(&self) -> P1_17R {
        P1_17R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline]
    pub fn p1_18(&self) -> P1_18R {
        P1_18R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline]
    pub fn p1_19(&self) -> P1_19R {
        P1_19R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline]
    pub fn p1_20(&self) -> P1_20R {
        P1_20R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline]
    pub fn p1_21(&self) -> P1_21R {
        P1_21R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline]
    pub fn p1_22(&self) -> P1_22R {
        P1_22R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline]
    pub fn p1_23(&self) -> P1_23R {
        P1_23R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline]
    pub fn p1_24(&self) -> P1_24R {
        P1_24R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline]
    pub fn p1_25(&self) -> P1_25R {
        P1_25R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline]
    pub fn p1_26(&self) -> P1_26R {
        P1_26R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline]
    pub fn p1_27(&self) -> P1_27R {
        P1_27R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline]
    pub fn p1_28(&self) -> P1_28R {
        P1_28R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline]
    pub fn p1_29(&self) -> P1_29R {
        P1_29R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline]
    pub fn p1_30(&self) -> P1_30R {
        P1_30R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline]
    pub fn p1_31(&self) -> P1_31R {
        P1_31R::_from({
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
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline]
    pub fn p1_16(&mut self) -> _P1_16W {
        _P1_16W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline]
    pub fn p1_17(&mut self) -> _P1_17W {
        _P1_17W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline]
    pub fn p1_18(&mut self) -> _P1_18W {
        _P1_18W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline]
    pub fn p1_19(&mut self) -> _P1_19W {
        _P1_19W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline]
    pub fn p1_20(&mut self) -> _P1_20W {
        _P1_20W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline]
    pub fn p1_21(&mut self) -> _P1_21W {
        _P1_21W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline]
    pub fn p1_22(&mut self) -> _P1_22W {
        _P1_22W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline]
    pub fn p1_23(&mut self) -> _P1_23W {
        _P1_23W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline]
    pub fn p1_24(&mut self) -> _P1_24W {
        _P1_24W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline]
    pub fn p1_25(&mut self) -> _P1_25W {
        _P1_25W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline]
    pub fn p1_26(&mut self) -> _P1_26W {
        _P1_26W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline]
    pub fn p1_27(&mut self) -> _P1_27W {
        _P1_27W { w: self }
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline]
    pub fn p1_28(&mut self) -> _P1_28W {
        _P1_28W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline]
    pub fn p1_29(&mut self) -> _P1_29W {
        _P1_29W { w: self }
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline]
    pub fn p1_30(&mut self) -> _P1_30W {
        _P1_30W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline]
    pub fn p1_31(&mut self) -> _P1_31W {
        _P1_31W { w: self }
    }
}
