#[doc = "Reader of register PINSEL4"]
pub type R = crate::R<u32, super::PINSEL4>;
#[doc = "Writer for register PINSEL4"]
pub type W = crate::W<u32, super::PINSEL4>;
#[doc = "Register PINSEL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin function select P2.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_0_A {
    #[doc = "0: GPIO P2.0"]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.1"]
    PWM1 = 1,
    #[doc = "2: TXD1"]
    TXD1 = 2,
}
impl From<P2_0_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_0`"]
pub type P2_0_R = crate::R<u8, P2_0_A>;
impl P2_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_0_A {
        match self.bits {
            0 => P2_0_A::GPIO_P2,
            1 => P2_0_A::PWM1,
            2 => P2_0_A::TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_0_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_0_A::PWM1
    }
    #[doc = "Checks if the value of the field is `TXD1`"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == P2_0_A::TXD1
    }
}
#[doc = "Write proxy for field `P2_0`"]
pub struct P2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.0"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_0_A::GPIO_P2)
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_0_A::PWM1)
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut W {
        self.variant(P2_0_A::TXD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Pin function select P2.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_1_A {
    #[doc = "0: GPIO P2.1"]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.2"]
    PWM1 = 1,
    #[doc = "2: RXD1"]
    RXD1 = 2,
}
impl From<P2_1_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_1`"]
pub type P2_1_R = crate::R<u8, P2_1_A>;
impl P2_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_1_A {
        match self.bits {
            0 => P2_1_A::GPIO_P2,
            1 => P2_1_A::PWM1,
            2 => P2_1_A::RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_1_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_1_A::PWM1
    }
    #[doc = "Checks if the value of the field is `RXD1`"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == P2_1_A::RXD1
    }
}
#[doc = "Write proxy for field `P2_1`"]
pub struct P2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.1"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_1_A::GPIO_P2)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_1_A::PWM1)
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut W {
        self.variant(P2_1_A::RXD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Pin function select P2.2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_2_A {
    #[doc = "0: GPIO P2.2"]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.3"]
    PWM1 = 1,
    #[doc = "2: CTS1"]
    CTS1 = 2,
}
impl From<P2_2_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_2`"]
pub type P2_2_R = crate::R<u8, P2_2_A>;
impl P2_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_2_A {
        match self.bits {
            0 => P2_2_A::GPIO_P2,
            1 => P2_2_A::PWM1,
            2 => P2_2_A::CTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_2_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_2_A::PWM1
    }
    #[doc = "Checks if the value of the field is `CTS1`"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == P2_2_A::CTS1
    }
}
#[doc = "Write proxy for field `P2_2`"]
pub struct P2_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.2"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_2_A::GPIO_P2)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_2_A::PWM1)
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut W {
        self.variant(P2_2_A::CTS1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Pin function select P2.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_3_A {
    #[doc = "0: GPIO P2.3."]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.4"]
    PWM1 = 1,
    #[doc = "2: DCD1"]
    DCD1 = 2,
}
impl From<P2_3_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_3`"]
pub type P2_3_R = crate::R<u8, P2_3_A>;
impl P2_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_3_A {
        match self.bits {
            0 => P2_3_A::GPIO_P2,
            1 => P2_3_A::PWM1,
            2 => P2_3_A::DCD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_3_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_3_A::PWM1
    }
    #[doc = "Checks if the value of the field is `DCD1`"]
    #[inline(always)]
    pub fn is_dcd1(&self) -> bool {
        *self == P2_3_A::DCD1
    }
}
#[doc = "Write proxy for field `P2_3`"]
pub struct P2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.3."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_3_A::GPIO_P2)
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_3_A::PWM1)
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn dcd1(self) -> &'a mut W {
        self.variant(P2_3_A::DCD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pin function select P2.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_4_A {
    #[doc = "0: GPIO P2.4."]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.5"]
    PWM1 = 1,
    #[doc = "2: DSR1"]
    DSR1 = 2,
}
impl From<P2_4_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_4`"]
pub type P2_4_R = crate::R<u8, P2_4_A>;
impl P2_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_4_A {
        match self.bits {
            0 => P2_4_A::GPIO_P2,
            1 => P2_4_A::PWM1,
            2 => P2_4_A::DSR1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_4_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_4_A::PWM1
    }
    #[doc = "Checks if the value of the field is `DSR1`"]
    #[inline(always)]
    pub fn is_dsr1(&self) -> bool {
        *self == P2_4_A::DSR1
    }
}
#[doc = "Write proxy for field `P2_4`"]
pub struct P2_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.4."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_4_A::GPIO_P2)
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_4_A::PWM1)
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn dsr1(self) -> &'a mut W {
        self.variant(P2_4_A::DSR1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Pin function select P2.5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_5_A {
    #[doc = "0: GPIO P2.5."]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.6"]
    PWM1 = 1,
    #[doc = "2: DTR1"]
    DTR1 = 2,
}
impl From<P2_5_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_5`"]
pub type P2_5_R = crate::R<u8, P2_5_A>;
impl P2_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_5_A {
        match self.bits {
            0 => P2_5_A::GPIO_P2,
            1 => P2_5_A::PWM1,
            2 => P2_5_A::DTR1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_5_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_5_A::PWM1
    }
    #[doc = "Checks if the value of the field is `DTR1`"]
    #[inline(always)]
    pub fn is_dtr1(&self) -> bool {
        *self == P2_5_A::DTR1
    }
}
#[doc = "Write proxy for field `P2_5`"]
pub struct P2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.5."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_5_A::GPIO_P2)
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(P2_5_A::PWM1)
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn dtr1(self) -> &'a mut W {
        self.variant(P2_5_A::DTR1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Pin function select P2.6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_6_A {
    #[doc = "0: GPIO P2.6."]
    GPIO_P2 = 0,
    #[doc = "1: PCAP1.0"]
    PCAP1 = 1,
    #[doc = "2: RI1"]
    RI1 = 2,
}
impl From<P2_6_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_6`"]
pub type P2_6_R = crate::R<u8, P2_6_A>;
impl P2_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_6_A {
        match self.bits {
            0 => P2_6_A::GPIO_P2,
            1 => P2_6_A::PCAP1,
            2 => P2_6_A::RI1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_6_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `PCAP1`"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P2_6_A::PCAP1
    }
    #[doc = "Checks if the value of the field is `RI1`"]
    #[inline(always)]
    pub fn is_ri1(&self) -> bool {
        *self == P2_6_A::RI1
    }
}
#[doc = "Write proxy for field `P2_6`"]
pub struct P2_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.6."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_6_A::GPIO_P2)
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut W {
        self.variant(P2_6_A::PCAP1)
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn ri1(self) -> &'a mut W {
        self.variant(P2_6_A::RI1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Pin function select P2.7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_7_A {
    #[doc = "0: GPIO P2.7."]
    GPIO_P2 = 0,
    #[doc = "1: RD2"]
    RD2 = 1,
    #[doc = "2: RTS1"]
    RTS1 = 2,
}
impl From<P2_7_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_7`"]
pub type P2_7_R = crate::R<u8, P2_7_A>;
impl P2_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_7_A {
        match self.bits {
            0 => P2_7_A::GPIO_P2,
            1 => P2_7_A::RD2,
            2 => P2_7_A::RTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_7_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `RD2`"]
    #[inline(always)]
    pub fn is_rd2(&self) -> bool {
        *self == P2_7_A::RD2
    }
    #[doc = "Checks if the value of the field is `RTS1`"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == P2_7_A::RTS1
    }
}
#[doc = "Write proxy for field `P2_7`"]
pub struct P2_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.7."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_7_A::GPIO_P2)
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn rd2(self) -> &'a mut W {
        self.variant(P2_7_A::RD2)
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut W {
        self.variant(P2_7_A::RTS1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pin function select P2.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_8_A {
    #[doc = "0: GPIO P2.8."]
    GPIO_P2 = 0,
    #[doc = "1: TD2"]
    TD2 = 1,
    #[doc = "2: TXD2"]
    TXD2 = 2,
    #[doc = "3: ENET_MDC"]
    ENET_MDC = 3,
}
impl From<P2_8_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_8`"]
pub type P2_8_R = crate::R<u8, P2_8_A>;
impl P2_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_8_A {
        match self.bits {
            0 => P2_8_A::GPIO_P2,
            1 => P2_8_A::TD2,
            2 => P2_8_A::TXD2,
            3 => P2_8_A::ENET_MDC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_8_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `TD2`"]
    #[inline(always)]
    pub fn is_td2(&self) -> bool {
        *self == P2_8_A::TD2
    }
    #[doc = "Checks if the value of the field is `TXD2`"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == P2_8_A::TXD2
    }
    #[doc = "Checks if the value of the field is `ENET_MDC`"]
    #[inline(always)]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P2_8_A::ENET_MDC
    }
}
#[doc = "Write proxy for field `P2_8`"]
pub struct P2_8_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P2.8."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_8_A::GPIO_P2)
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn td2(self) -> &'a mut W {
        self.variant(P2_8_A::TD2)
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut W {
        self.variant(P2_8_A::TXD2)
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn enet_mdc(self) -> &'a mut W {
        self.variant(P2_8_A::ENET_MDC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Pin function select P2.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_9_A {
    #[doc = "0: GPIO P2.9"]
    GPIO_P2 = 0,
    #[doc = "1: USB_CONNECT"]
    USB_CONNECT = 1,
    #[doc = "2: RXD2"]
    RXD2 = 2,
    #[doc = "3: ENET_MDIO"]
    ENET_MDIO = 3,
}
impl From<P2_9_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_9`"]
pub type P2_9_R = crate::R<u8, P2_9_A>;
impl P2_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_9_A {
        match self.bits {
            0 => P2_9_A::GPIO_P2,
            1 => P2_9_A::USB_CONNECT,
            2 => P2_9_A::RXD2,
            3 => P2_9_A::ENET_MDIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_9_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `USB_CONNECT`"]
    #[inline(always)]
    pub fn is_usb_connect(&self) -> bool {
        *self == P2_9_A::USB_CONNECT
    }
    #[doc = "Checks if the value of the field is `RXD2`"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == P2_9_A::RXD2
    }
    #[doc = "Checks if the value of the field is `ENET_MDIO`"]
    #[inline(always)]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P2_9_A::ENET_MDIO
    }
}
#[doc = "Write proxy for field `P2_9`"]
pub struct P2_9_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P2.9"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_9_A::GPIO_P2)
    }
    #[doc = "USB_CONNECT"]
    #[inline(always)]
    pub fn usb_connect(self) -> &'a mut W {
        self.variant(P2_9_A::USB_CONNECT)
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut W {
        self.variant(P2_9_A::RXD2)
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn enet_mdio(self) -> &'a mut W {
        self.variant(P2_9_A::ENET_MDIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Pin function select P2.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_10_A {
    #[doc = "0: GPIO P2.10"]
    GPIO_P2 = 0,
    #[doc = "1: EINT0"]
    EINT0 = 1,
    #[doc = "2: NMI"]
    NMI = 2,
}
impl From<P2_10_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_10`"]
pub type P2_10_R = crate::R<u8, P2_10_A>;
impl P2_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_10_A {
        match self.bits {
            0 => P2_10_A::GPIO_P2,
            1 => P2_10_A::EINT0,
            2 => P2_10_A::NMI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_10_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT0`"]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        *self == P2_10_A::EINT0
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == P2_10_A::NMI
    }
}
#[doc = "Write proxy for field `P2_10`"]
pub struct P2_10_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.10"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_10_A::GPIO_P2)
    }
    #[doc = "EINT0"]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut W {
        self.variant(P2_10_A::EINT0)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(P2_10_A::NMI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Pin function select P2.11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_11_A {
    #[doc = "0: GPIO P2.11"]
    GPIO_P2 = 0,
    #[doc = "1: EINT1"]
    EINT1 = 1,
    #[doc = "3: I2STX_CLK"]
    I2STX_CLK = 3,
}
impl From<P2_11_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_11`"]
pub type P2_11_R = crate::R<u8, P2_11_A>;
impl P2_11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_11_A {
        match self.bits {
            0 => P2_11_A::GPIO_P2,
            1 => P2_11_A::EINT1,
            3 => P2_11_A::I2STX_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_11_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT1`"]
    #[inline(always)]
    pub fn is_eint1(&self) -> bool {
        *self == P2_11_A::EINT1
    }
    #[doc = "Checks if the value of the field is `I2STX_CLK`"]
    #[inline(always)]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P2_11_A::I2STX_CLK
    }
}
#[doc = "Write proxy for field `P2_11`"]
pub struct P2_11_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.11"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_11_A::GPIO_P2)
    }
    #[doc = "EINT1"]
    #[inline(always)]
    pub fn eint1(self) -> &'a mut W {
        self.variant(P2_11_A::EINT1)
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn i2stx_clk(self) -> &'a mut W {
        self.variant(P2_11_A::I2STX_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pin function select P2.12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_12_A {
    #[doc = "0: GPIO P2.12"]
    GPIO_P2 = 0,
    #[doc = "1: EINT2"]
    EINT2 = 1,
    #[doc = "3: I2STX_WS"]
    I2STX_WS = 3,
}
impl From<P2_12_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_12`"]
pub type P2_12_R = crate::R<u8, P2_12_A>;
impl P2_12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_12_A {
        match self.bits {
            0 => P2_12_A::GPIO_P2,
            1 => P2_12_A::EINT2,
            3 => P2_12_A::I2STX_WS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_12_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT2`"]
    #[inline(always)]
    pub fn is_eint2(&self) -> bool {
        *self == P2_12_A::EINT2
    }
    #[doc = "Checks if the value of the field is `I2STX_WS`"]
    #[inline(always)]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P2_12_A::I2STX_WS
    }
}
#[doc = "Write proxy for field `P2_12`"]
pub struct P2_12_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.12"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_12_A::GPIO_P2)
    }
    #[doc = "EINT2"]
    #[inline(always)]
    pub fn eint2(self) -> &'a mut W {
        self.variant(P2_12_A::EINT2)
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn i2stx_ws(self) -> &'a mut W {
        self.variant(P2_12_A::I2STX_WS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Pin function select P2.13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_13_A {
    #[doc = "0: GPIO P2.13"]
    GPIO_P2 = 0,
    #[doc = "1: EINT3"]
    EINT3 = 1,
    #[doc = "3: I2STX_SDA"]
    I2STX_SDA = 3,
}
impl From<P2_13_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_13`"]
pub type P2_13_R = crate::R<u8, P2_13_A>;
impl P2_13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_13_A {
        match self.bits {
            0 => P2_13_A::GPIO_P2,
            1 => P2_13_A::EINT3,
            3 => P2_13_A::I2STX_SDA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P2`"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_13_A::GPIO_P2
    }
    #[doc = "Checks if the value of the field is `EINT3`"]
    #[inline(always)]
    pub fn is_eint3(&self) -> bool {
        *self == P2_13_A::EINT3
    }
    #[doc = "Checks if the value of the field is `I2STX_SDA`"]
    #[inline(always)]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P2_13_A::I2STX_SDA
    }
}
#[doc = "Write proxy for field `P2_13`"]
pub struct P2_13_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P2.13"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut W {
        self.variant(P2_13_A::GPIO_P2)
    }
    #[doc = "EINT3"]
    #[inline(always)]
    pub fn eint3(self) -> &'a mut W {
        self.variant(P2_13_A::EINT3)
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn i2stx_sda(self) -> &'a mut W {
        self.variant(P2_13_A::I2STX_SDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline(always)]
    pub fn p2_0(&self) -> P2_0_R {
        P2_0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline(always)]
    pub fn p2_1(&self) -> P2_1_R {
        P2_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline(always)]
    pub fn p2_2(&self) -> P2_2_R {
        P2_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline(always)]
    pub fn p2_3(&self) -> P2_3_R {
        P2_3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline(always)]
    pub fn p2_4(&self) -> P2_4_R {
        P2_4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline(always)]
    pub fn p2_5(&self) -> P2_5_R {
        P2_5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline(always)]
    pub fn p2_6(&self) -> P2_6_R {
        P2_6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline(always)]
    pub fn p2_7(&self) -> P2_7_R {
        P2_7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline(always)]
    pub fn p2_8(&self) -> P2_8_R {
        P2_8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline(always)]
    pub fn p2_9(&self) -> P2_9_R {
        P2_9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline(always)]
    pub fn p2_10(&self) -> P2_10_R {
        P2_10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline(always)]
    pub fn p2_11(&self) -> P2_11_R {
        P2_11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline(always)]
    pub fn p2_12(&self) -> P2_12_R {
        P2_12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline(always)]
    pub fn p2_13(&self) -> P2_13_R {
        P2_13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline(always)]
    pub fn p2_0(&mut self) -> P2_0_W {
        P2_0_W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline(always)]
    pub fn p2_1(&mut self) -> P2_1_W {
        P2_1_W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline(always)]
    pub fn p2_2(&mut self) -> P2_2_W {
        P2_2_W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline(always)]
    pub fn p2_3(&mut self) -> P2_3_W {
        P2_3_W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline(always)]
    pub fn p2_4(&mut self) -> P2_4_W {
        P2_4_W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline(always)]
    pub fn p2_5(&mut self) -> P2_5_W {
        P2_5_W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline(always)]
    pub fn p2_6(&mut self) -> P2_6_W {
        P2_6_W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline(always)]
    pub fn p2_7(&mut self) -> P2_7_W {
        P2_7_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline(always)]
    pub fn p2_8(&mut self) -> P2_8_W {
        P2_8_W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline(always)]
    pub fn p2_9(&mut self) -> P2_9_W {
        P2_9_W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline(always)]
    pub fn p2_10(&mut self) -> P2_10_W {
        P2_10_W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline(always)]
    pub fn p2_11(&mut self) -> P2_11_W {
        P2_11_W { w: self }
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline(always)]
    pub fn p2_12(&mut self) -> P2_12_W {
        P2_12_W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline(always)]
    pub fn p2_13(&mut self) -> P2_13_W {
        P2_13_W { w: self }
    }
}
