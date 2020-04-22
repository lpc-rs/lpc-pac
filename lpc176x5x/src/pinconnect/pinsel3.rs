#[doc = "Reader of register PINSEL3"]
pub type R = crate::R<u32, super::PINSEL3>;
#[doc = "Writer for register PINSEL3"]
pub type W = crate::W<u32, super::PINSEL3>;
#[doc = "Register PINSEL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin function select P1.16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_16_A {
    #[doc = "0: GPIO P1.16"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_MDC"]
    ENET_MDC = 1,
}
impl From<P1_16_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_16_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_16`"]
pub type P1_16_R = crate::R<u8, P1_16_A>;
impl P1_16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_16_A {
        match self.bits {
            0 => P1_16_A::GPIO_P1,
            1 => P1_16_A::ENET_MDC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_16_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_MDC`"]
    #[inline(always)]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P1_16_A::ENET_MDC
    }
}
#[doc = "Write proxy for field `P1_16`"]
pub struct P1_16_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_16_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.16"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_16_A::GPIO_P1)
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn enet_mdc(self) -> &'a mut W {
        self.variant(P1_16_A::ENET_MDC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Pin function select P1.17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_17_A {
    #[doc = "0: GPIO P1.17"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_MDIO"]
    ENET_MDIO = 1,
}
impl From<P1_17_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_17_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_17`"]
pub type P1_17_R = crate::R<u8, P1_17_A>;
impl P1_17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_17_A {
        match self.bits {
            0 => P1_17_A::GPIO_P1,
            1 => P1_17_A::ENET_MDIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_17_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_MDIO`"]
    #[inline(always)]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P1_17_A::ENET_MDIO
    }
}
#[doc = "Write proxy for field `P1_17`"]
pub struct P1_17_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_17_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.17"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_17_A::GPIO_P1)
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn enet_mdio(self) -> &'a mut W {
        self.variant(P1_17_A::ENET_MDIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Pin function select P1.18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_18_A {
    #[doc = "0: GPIO P1.18"]
    GPIO_P1 = 0,
    #[doc = "1: USB_UP_LED"]
    USB_UP_LED = 1,
    #[doc = "2: PWM1.1"]
    PWM1 = 2,
    #[doc = "3: CAP1.0"]
    CAP1 = 3,
}
impl From<P1_18_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_18_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_18`"]
pub type P1_18_R = crate::R<u8, P1_18_A>;
impl P1_18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_18_A {
        match self.bits {
            0 => P1_18_A::GPIO_P1,
            1 => P1_18_A::USB_UP_LED,
            2 => P1_18_A::PWM1,
            3 => P1_18_A::CAP1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_18_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `USB_UP_LED`"]
    #[inline(always)]
    pub fn is_usb_up_led(&self) -> bool {
        *self == P1_18_A::USB_UP_LED
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_18_A::PWM1
    }
    #[doc = "Checks if the value of the field is `CAP1`"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_18_A::CAP1
    }
}
#[doc = "Write proxy for field `P1_18`"]
pub struct P1_18_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_18_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.18"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_18_A::GPIO_P1)
    }
    #[doc = "USB_UP_LED"]
    #[inline(always)]
    pub fn usb_up_led(self) -> &'a mut W {
        self.variant(P1_18_A::USB_UP_LED)
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_18_A::PWM1)
    }
    #[doc = "CAP1.0"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut W {
        self.variant(P1_18_A::CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Pin function select P1.19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_19_A {
    #[doc = "0: GPIO P1.19."]
    GPIO_P1 = 0,
    #[doc = "1: MCOA0"]
    MCOA0 = 1,
    #[doc = "2: USB_PPWR"]
    USB_PPWR = 2,
    #[doc = "3: CAP1.1"]
    CAP1 = 3,
}
impl From<P1_19_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_19_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_19`"]
pub type P1_19_R = crate::R<u8, P1_19_A>;
impl P1_19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_19_A {
        match self.bits {
            0 => P1_19_A::GPIO_P1,
            1 => P1_19_A::MCOA0,
            2 => P1_19_A::USB_PPWR,
            3 => P1_19_A::CAP1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_19_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA0`"]
    #[inline(always)]
    pub fn is_mcoa0(&self) -> bool {
        *self == P1_19_A::MCOA0
    }
    #[doc = "Checks if the value of the field is `USB_PPWR`"]
    #[inline(always)]
    pub fn is_usb_ppwr(&self) -> bool {
        *self == P1_19_A::USB_PPWR
    }
    #[doc = "Checks if the value of the field is `CAP1`"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_19_A::CAP1
    }
}
#[doc = "Write proxy for field `P1_19`"]
pub struct P1_19_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_19_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.19."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_19_A::GPIO_P1)
    }
    #[doc = "MCOA0"]
    #[inline(always)]
    pub fn mcoa0(self) -> &'a mut W {
        self.variant(P1_19_A::MCOA0)
    }
    #[doc = "USB_PPWR"]
    #[inline(always)]
    pub fn usb_ppwr(self) -> &'a mut W {
        self.variant(P1_19_A::USB_PPWR)
    }
    #[doc = "CAP1.1"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut W {
        self.variant(P1_19_A::CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pin function select P1.20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_20_A {
    #[doc = "0: GPIO P1.20."]
    GPIO_P1 = 0,
    #[doc = "1: MCI0"]
    MCI0 = 1,
    #[doc = "2: PWM1.2"]
    PWM1 = 2,
    #[doc = "3: SCK0"]
    SCK0 = 3,
}
impl From<P1_20_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_20_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_20`"]
pub type P1_20_R = crate::R<u8, P1_20_A>;
impl P1_20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_20_A {
        match self.bits {
            0 => P1_20_A::GPIO_P1,
            1 => P1_20_A::MCI0,
            2 => P1_20_A::PWM1,
            3 => P1_20_A::SCK0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_20_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI0`"]
    #[inline(always)]
    pub fn is_mci0(&self) -> bool {
        *self == P1_20_A::MCI0
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_20_A::PWM1
    }
    #[doc = "Checks if the value of the field is `SCK0`"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P1_20_A::SCK0
    }
}
#[doc = "Write proxy for field `P1_20`"]
pub struct P1_20_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_20_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.20."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_20_A::GPIO_P1)
    }
    #[doc = "MCI0"]
    #[inline(always)]
    pub fn mci0(self) -> &'a mut W {
        self.variant(P1_20_A::MCI0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_20_A::PWM1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut W {
        self.variant(P1_20_A::SCK0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Pin function select P1.21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_21_A {
    #[doc = "0: GPIO P1.21."]
    GPIO_P1 = 0,
    #[doc = "1: MCABORT"]
    MCABORT = 1,
    #[doc = "2: PWM1.3"]
    PWM1 = 2,
    #[doc = "3: SSEL0"]
    SSEL0 = 3,
}
impl From<P1_21_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_21_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_21`"]
pub type P1_21_R = crate::R<u8, P1_21_A>;
impl P1_21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_21_A {
        match self.bits {
            0 => P1_21_A::GPIO_P1,
            1 => P1_21_A::MCABORT,
            2 => P1_21_A::PWM1,
            3 => P1_21_A::SSEL0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_21_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCABORT`"]
    #[inline(always)]
    pub fn is_mcabort(&self) -> bool {
        *self == P1_21_A::MCABORT
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_21_A::PWM1
    }
    #[doc = "Checks if the value of the field is `SSEL0`"]
    #[inline(always)]
    pub fn is_ssel0(&self) -> bool {
        *self == P1_21_A::SSEL0
    }
}
#[doc = "Write proxy for field `P1_21`"]
pub struct P1_21_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_21_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.21."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_21_A::GPIO_P1)
    }
    #[doc = "MCABORT"]
    #[inline(always)]
    pub fn mcabort(self) -> &'a mut W {
        self.variant(P1_21_A::MCABORT)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_21_A::PWM1)
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn ssel0(self) -> &'a mut W {
        self.variant(P1_21_A::SSEL0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Pin function select P1.22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_22_A {
    #[doc = "0: GPIO P1.22."]
    GPIO_P1 = 0,
    #[doc = "1: MCOB0"]
    MCOB0 = 1,
    #[doc = "2: USB_PWRD"]
    USB_PWRD = 2,
    #[doc = "3: MAT1.0"]
    MAT1 = 3,
}
impl From<P1_22_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_22_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_22`"]
pub type P1_22_R = crate::R<u8, P1_22_A>;
impl P1_22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_22_A {
        match self.bits {
            0 => P1_22_A::GPIO_P1,
            1 => P1_22_A::MCOB0,
            2 => P1_22_A::USB_PWRD,
            3 => P1_22_A::MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_22_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB0`"]
    #[inline(always)]
    pub fn is_mcob0(&self) -> bool {
        *self == P1_22_A::MCOB0
    }
    #[doc = "Checks if the value of the field is `USB_PWRD`"]
    #[inline(always)]
    pub fn is_usb_pwrd(&self) -> bool {
        *self == P1_22_A::USB_PWRD
    }
    #[doc = "Checks if the value of the field is `MAT1`"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_22_A::MAT1
    }
}
#[doc = "Write proxy for field `P1_22`"]
pub struct P1_22_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_22_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.22."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_22_A::GPIO_P1)
    }
    #[doc = "MCOB0"]
    #[inline(always)]
    pub fn mcob0(self) -> &'a mut W {
        self.variant(P1_22_A::MCOB0)
    }
    #[doc = "USB_PWRD"]
    #[inline(always)]
    pub fn usb_pwrd(self) -> &'a mut W {
        self.variant(P1_22_A::USB_PWRD)
    }
    #[doc = "MAT1.0"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut W {
        self.variant(P1_22_A::MAT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Pin function select P1.23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_23_A {
    #[doc = "0: GPIO P1.23."]
    GPIO_P1 = 0,
    #[doc = "1: MCI1"]
    MCI1 = 1,
    #[doc = "2: PWM1.4"]
    PWM1 = 2,
    #[doc = "3: MISO0"]
    MISO0 = 3,
}
impl From<P1_23_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_23`"]
pub type P1_23_R = crate::R<u8, P1_23_A>;
impl P1_23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_23_A {
        match self.bits {
            0 => P1_23_A::GPIO_P1,
            1 => P1_23_A::MCI1,
            2 => P1_23_A::PWM1,
            3 => P1_23_A::MISO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_23_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI1`"]
    #[inline(always)]
    pub fn is_mci1(&self) -> bool {
        *self == P1_23_A::MCI1
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_23_A::PWM1
    }
    #[doc = "Checks if the value of the field is `MISO0`"]
    #[inline(always)]
    pub fn is_miso0(&self) -> bool {
        *self == P1_23_A::MISO0
    }
}
#[doc = "Write proxy for field `P1_23`"]
pub struct P1_23_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.23."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_23_A::GPIO_P1)
    }
    #[doc = "MCI1"]
    #[inline(always)]
    pub fn mci1(self) -> &'a mut W {
        self.variant(P1_23_A::MCI1)
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_23_A::PWM1)
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn miso0(self) -> &'a mut W {
        self.variant(P1_23_A::MISO0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pin function select P1.24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_24_A {
    #[doc = "0: GPIO P1.24."]
    GPIO_P1 = 0,
    #[doc = "1: MCI2"]
    MCI2 = 1,
    #[doc = "2: PWM1.5"]
    PWM1 = 2,
    #[doc = "3: MOSI0"]
    MOSI0 = 3,
}
impl From<P1_24_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_24_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_24`"]
pub type P1_24_R = crate::R<u8, P1_24_A>;
impl P1_24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_24_A {
        match self.bits {
            0 => P1_24_A::GPIO_P1,
            1 => P1_24_A::MCI2,
            2 => P1_24_A::PWM1,
            3 => P1_24_A::MOSI0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_24_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCI2`"]
    #[inline(always)]
    pub fn is_mci2(&self) -> bool {
        *self == P1_24_A::MCI2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_24_A::PWM1
    }
    #[doc = "Checks if the value of the field is `MOSI0`"]
    #[inline(always)]
    pub fn is_mosi0(&self) -> bool {
        *self == P1_24_A::MOSI0
    }
}
#[doc = "Write proxy for field `P1_24`"]
pub struct P1_24_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_24_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.24."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_24_A::GPIO_P1)
    }
    #[doc = "MCI2"]
    #[inline(always)]
    pub fn mci2(self) -> &'a mut W {
        self.variant(P1_24_A::MCI2)
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_24_A::PWM1)
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn mosi0(self) -> &'a mut W {
        self.variant(P1_24_A::MOSI0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Pin function select P1.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_25_A {
    #[doc = "0: GPIO P1.25"]
    GPIO_P1 = 0,
    #[doc = "1: MCOA1"]
    MCOA1 = 1,
    #[doc = "3: MAT1.1"]
    MAT1 = 3,
}
impl From<P1_25_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_25_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_25`"]
pub type P1_25_R = crate::R<u8, P1_25_A>;
impl P1_25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_25_A {
        match self.bits {
            0 => P1_25_A::GPIO_P1,
            1 => P1_25_A::MCOA1,
            3 => P1_25_A::MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_25_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA1`"]
    #[inline(always)]
    pub fn is_mcoa1(&self) -> bool {
        *self == P1_25_A::MCOA1
    }
    #[doc = "Checks if the value of the field is `MAT1`"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_25_A::MAT1
    }
}
#[doc = "Write proxy for field `P1_25`"]
pub struct P1_25_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_25_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.25"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_25_A::GPIO_P1)
    }
    #[doc = "MCOA1"]
    #[inline(always)]
    pub fn mcoa1(self) -> &'a mut W {
        self.variant(P1_25_A::MCOA1)
    }
    #[doc = "MAT1.1"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut W {
        self.variant(P1_25_A::MAT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Pin function select P1.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_26_A {
    #[doc = "0: GPIO P1.26"]
    GPIO_P1 = 0,
    #[doc = "1: MCOB1"]
    MCOB1 = 1,
    #[doc = "2: PWM1.6"]
    PWM1 = 2,
    #[doc = "3: CAP0.0"]
    CAP0 = 3,
}
impl From<P1_26_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_26_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_26`"]
pub type P1_26_R = crate::R<u8, P1_26_A>;
impl P1_26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_26_A {
        match self.bits {
            0 => P1_26_A::GPIO_P1,
            1 => P1_26_A::MCOB1,
            2 => P1_26_A::PWM1,
            3 => P1_26_A::CAP0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_26_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB1`"]
    #[inline(always)]
    pub fn is_mcob1(&self) -> bool {
        *self == P1_26_A::MCOB1
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_26_A::PWM1
    }
    #[doc = "Checks if the value of the field is `CAP0`"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_26_A::CAP0
    }
}
#[doc = "Write proxy for field `P1_26`"]
pub struct P1_26_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_26_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.26"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_26_A::GPIO_P1)
    }
    #[doc = "MCOB1"]
    #[inline(always)]
    pub fn mcob1(self) -> &'a mut W {
        self.variant(P1_26_A::MCOB1)
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P1_26_A::PWM1)
    }
    #[doc = "CAP0.0"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut W {
        self.variant(P1_26_A::CAP0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Pin function select P1.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_27_A {
    #[doc = "0: GPIO P1.27"]
    GPIO_P1 = 0,
    #[doc = "1: CLKOUT"]
    CLKOUT = 1,
    #[doc = "2: USB_OVRCR"]
    USB_OVRCR = 2,
    #[doc = "3: CAP0.1"]
    CAP0 = 3,
}
impl From<P1_27_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_27_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_27`"]
pub type P1_27_R = crate::R<u8, P1_27_A>;
impl P1_27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_27_A {
        match self.bits {
            0 => P1_27_A::GPIO_P1,
            1 => P1_27_A::CLKOUT,
            2 => P1_27_A::USB_OVRCR,
            3 => P1_27_A::CAP0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_27_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == P1_27_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `USB_OVRCR`"]
    #[inline(always)]
    pub fn is_usb_ovrcr(&self) -> bool {
        *self == P1_27_A::USB_OVRCR
    }
    #[doc = "Checks if the value of the field is `CAP0`"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_27_A::CAP0
    }
}
#[doc = "Write proxy for field `P1_27`"]
pub struct P1_27_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_27_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.27"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_27_A::GPIO_P1)
    }
    #[doc = "CLKOUT"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(P1_27_A::CLKOUT)
    }
    #[doc = "USB_OVRCR"]
    #[inline(always)]
    pub fn usb_ovrcr(self) -> &'a mut W {
        self.variant(P1_27_A::USB_OVRCR)
    }
    #[doc = "CAP0.1"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut W {
        self.variant(P1_27_A::CAP0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pin function select P1.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_28_A {
    #[doc = "0: GPIO P1.28"]
    GPIO_P1 = 0,
    #[doc = "1: MCOA2"]
    MCOA2 = 1,
    #[doc = "2: PCAP1.0"]
    PCAP1 = 2,
    #[doc = "3: MAT0.0"]
    MAT0 = 3,
}
impl From<P1_28_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_28_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_28`"]
pub type P1_28_R = crate::R<u8, P1_28_A>;
impl P1_28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_28_A {
        match self.bits {
            0 => P1_28_A::GPIO_P1,
            1 => P1_28_A::MCOA2,
            2 => P1_28_A::PCAP1,
            3 => P1_28_A::MAT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_28_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOA2`"]
    #[inline(always)]
    pub fn is_mcoa2(&self) -> bool {
        *self == P1_28_A::MCOA2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_28_A::PCAP1
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_28_A::MAT0
    }
}
#[doc = "Write proxy for field `P1_28`"]
pub struct P1_28_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_28_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.28"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_28_A::GPIO_P1)
    }
    #[doc = "MCOA2"]
    #[inline(always)]
    pub fn mcoa2(self) -> &'a mut W {
        self.variant(P1_28_A::MCOA2)
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P1_28_A::PCAP1)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P1_28_A::MAT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Pin function select P1.29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_29_A {
    #[doc = "0: GPIO P1.29"]
    GPIO_P1 = 0,
    #[doc = "1: MCOB2"]
    MCOB2 = 1,
    #[doc = "2: PCAP1.1"]
    PCAP1 = 2,
    #[doc = "3: MAT0.1"]
    MAT0 = 3,
}
impl From<P1_29_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_29_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_29`"]
pub type P1_29_R = crate::R<u8, P1_29_A>;
impl P1_29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_29_A {
        match self.bits {
            0 => P1_29_A::GPIO_P1,
            1 => P1_29_A::MCOB2,
            2 => P1_29_A::PCAP1,
            3 => P1_29_A::MAT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_29_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `MCOB2`"]
    #[inline(always)]
    pub fn is_mcob2(&self) -> bool {
        *self == P1_29_A::MCOB2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_29_A::PCAP1
    }
    #[doc = "Checks if the value of the field is `MAT0`"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_29_A::MAT0
    }
}
#[doc = "Write proxy for field `P1_29`"]
pub struct P1_29_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_29_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P1.29"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_29_A::GPIO_P1)
    }
    #[doc = "MCOB2"]
    #[inline(always)]
    pub fn mcob2(self) -> &'a mut W {
        self.variant(P1_29_A::MCOB2)
    }
    #[doc = "PCAP1.1"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P1_29_A::PCAP1)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut W {
        self.variant(P1_29_A::MAT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Pin function select P1.30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_30_A {
    #[doc = "0: GPIO P1.30"]
    GPIO_P1 = 0,
    #[doc = "2: VBUS"]
    VBUS = 2,
    #[doc = "3: AD0.4"]
    AD0 = 3,
}
impl From<P1_30_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_30_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_30`"]
pub type P1_30_R = crate::R<u8, P1_30_A>;
impl P1_30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_30_A {
        match self.bits {
            0 => P1_30_A::GPIO_P1,
            2 => P1_30_A::VBUS,
            3 => P1_30_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_30_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `VBUS`"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == P1_30_A::VBUS
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_30_A::AD0
    }
}
#[doc = "Write proxy for field `P1_30`"]
pub struct P1_30_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_30_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.30"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_30_A::GPIO_P1)
    }
    #[doc = "VBUS"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut W {
        self.variant(P1_30_A::VBUS)
    }
    #[doc = "AD0.4"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P1_30_A::AD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Pin function select P1.31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_31_A {
    #[doc = "0: GPIO Port 1.31"]
    GPIO_PORT_1 = 0,
    #[doc = "2: SCK1"]
    SCK1 = 2,
    #[doc = "3: AD0.5"]
    AD0 = 3,
}
impl From<P1_31_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_31_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_31`"]
pub type P1_31_R = crate::R<u8, P1_31_A>;
impl P1_31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_31_A {
        match self.bits {
            0 => P1_31_A::GPIO_PORT_1,
            2 => P1_31_A::SCK1,
            3 => P1_31_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline(always)]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_31_A::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `SCK1`"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P1_31_A::SCK1
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_31_A::AD0
    }
}
#[doc = "Write proxy for field `P1_31`"]
pub struct P1_31_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_31_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO Port 1.31"]
    #[inline(always)]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(P1_31_A::GPIO_PORT_1)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut W {
        self.variant(P1_31_A::SCK1)
    }
    #[doc = "AD0.5"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P1_31_A::AD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    pub fn p1_16(&self) -> P1_16_R {
        P1_16_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    pub fn p1_17(&self) -> P1_17_R {
        P1_17_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    pub fn p1_18(&self) -> P1_18_R {
        P1_18_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    pub fn p1_19(&self) -> P1_19_R {
        P1_19_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    pub fn p1_20(&self) -> P1_20_R {
        P1_20_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    pub fn p1_21(&self) -> P1_21_R {
        P1_21_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    pub fn p1_22(&self) -> P1_22_R {
        P1_22_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    pub fn p1_23(&self) -> P1_23_R {
        P1_23_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    pub fn p1_24(&self) -> P1_24_R {
        P1_24_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    pub fn p1_25(&self) -> P1_25_R {
        P1_25_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    pub fn p1_26(&self) -> P1_26_R {
        P1_26_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    pub fn p1_27(&self) -> P1_27_R {
        P1_27_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    pub fn p1_28(&self) -> P1_28_R {
        P1_28_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    pub fn p1_29(&self) -> P1_29_R {
        P1_29_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    pub fn p1_30(&self) -> P1_30_R {
        P1_30_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    pub fn p1_31(&self) -> P1_31_R {
        P1_31_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    pub fn p1_16(&mut self) -> P1_16_W {
        P1_16_W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    pub fn p1_17(&mut self) -> P1_17_W {
        P1_17_W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    pub fn p1_18(&mut self) -> P1_18_W {
        P1_18_W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    pub fn p1_19(&mut self) -> P1_19_W {
        P1_19_W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    pub fn p1_20(&mut self) -> P1_20_W {
        P1_20_W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    pub fn p1_21(&mut self) -> P1_21_W {
        P1_21_W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    pub fn p1_22(&mut self) -> P1_22_W {
        P1_22_W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    pub fn p1_23(&mut self) -> P1_23_W {
        P1_23_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    pub fn p1_24(&mut self) -> P1_24_W {
        P1_24_W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    pub fn p1_25(&mut self) -> P1_25_W {
        P1_25_W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    pub fn p1_26(&mut self) -> P1_26_W {
        P1_26_W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    pub fn p1_27(&mut self) -> P1_27_W {
        P1_27_W { w: self }
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    pub fn p1_28(&mut self) -> P1_28_W {
        P1_28_W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    pub fn p1_29(&mut self) -> P1_29_W {
        P1_29_W { w: self }
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    pub fn p1_30(&mut self) -> P1_30_W {
        P1_30_W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    pub fn p1_31(&mut self) -> P1_31_W {
        P1_31_W { w: self }
    }
}
