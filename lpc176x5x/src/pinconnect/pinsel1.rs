#[doc = "Reader of register PINSEL1"]
pub type R = crate::R<u32, super::PINSEL1>;
#[doc = "Writer for register PINSEL1"]
pub type W = crate::W<u32, super::PINSEL1>;
#[doc = "Register PINSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin function select P0.16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_16_A {
    #[doc = "0: GPIO P0.16"]
    GPIO_P0 = 0,
    #[doc = "1: RXD1"]
    RXD1 = 1,
    #[doc = "2: SSEL0"]
    SSEL0 = 2,
    #[doc = "3: SSEL"]
    SSEL = 3,
}
impl From<P0_16_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_16_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_16`"]
pub type P0_16_R = crate::R<u8, P0_16_A>;
impl P0_16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_16_A {
        match self.bits {
            0 => P0_16_A::GPIO_P0,
            1 => P0_16_A::RXD1,
            2 => P0_16_A::SSEL0,
            3 => P0_16_A::SSEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_16_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RXD1`"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == P0_16_A::RXD1
    }
    #[doc = "Checks if the value of the field is `SSEL0`"]
    #[inline(always)]
    pub fn is_ssel0(&self) -> bool {
        *self == P0_16_A::SSEL0
    }
    #[doc = "Checks if the value of the field is `SSEL`"]
    #[inline(always)]
    pub fn is_ssel(&self) -> bool {
        *self == P0_16_A::SSEL
    }
}
#[doc = "Write proxy for field `P0_16`"]
pub struct P0_16_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_16_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.16"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_16_A::GPIO_P0)
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut W {
        self.variant(P0_16_A::RXD1)
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn ssel0(self) -> &'a mut W {
        self.variant(P0_16_A::SSEL0)
    }
    #[doc = "SSEL"]
    #[inline(always)]
    pub fn ssel(self) -> &'a mut W {
        self.variant(P0_16_A::SSEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Pin function select P0.17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_17_A {
    #[doc = "0: GPIO P0.17"]
    GPIO_P0 = 0,
    #[doc = "1: CTS1"]
    CTS1 = 1,
    #[doc = "2: MISO0"]
    MISO0 = 2,
    #[doc = "3: MISO"]
    MISO = 3,
}
impl From<P0_17_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_17_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_17`"]
pub type P0_17_R = crate::R<u8, P0_17_A>;
impl P0_17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_17_A {
        match self.bits {
            0 => P0_17_A::GPIO_P0,
            1 => P0_17_A::CTS1,
            2 => P0_17_A::MISO0,
            3 => P0_17_A::MISO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_17_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `CTS1`"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == P0_17_A::CTS1
    }
    #[doc = "Checks if the value of the field is `MISO0`"]
    #[inline(always)]
    pub fn is_miso0(&self) -> bool {
        *self == P0_17_A::MISO0
    }
    #[doc = "Checks if the value of the field is `MISO`"]
    #[inline(always)]
    pub fn is_miso(&self) -> bool {
        *self == P0_17_A::MISO
    }
}
#[doc = "Write proxy for field `P0_17`"]
pub struct P0_17_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_17_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.17"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_17_A::GPIO_P0)
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut W {
        self.variant(P0_17_A::CTS1)
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn miso0(self) -> &'a mut W {
        self.variant(P0_17_A::MISO0)
    }
    #[doc = "MISO"]
    #[inline(always)]
    pub fn miso(self) -> &'a mut W {
        self.variant(P0_17_A::MISO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Pin function select P0.18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_18_A {
    #[doc = "0: GPIO P0.18"]
    GPIO_P0 = 0,
    #[doc = "1: DCD1"]
    DCD1 = 1,
    #[doc = "2: MOSI0"]
    MOSI0 = 2,
    #[doc = "3: MOSI"]
    MOSI = 3,
}
impl From<P0_18_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_18_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_18`"]
pub type P0_18_R = crate::R<u8, P0_18_A>;
impl P0_18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_18_A {
        match self.bits {
            0 => P0_18_A::GPIO_P0,
            1 => P0_18_A::DCD1,
            2 => P0_18_A::MOSI0,
            3 => P0_18_A::MOSI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_18_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `DCD1`"]
    #[inline(always)]
    pub fn is_dcd1(&self) -> bool {
        *self == P0_18_A::DCD1
    }
    #[doc = "Checks if the value of the field is `MOSI0`"]
    #[inline(always)]
    pub fn is_mosi0(&self) -> bool {
        *self == P0_18_A::MOSI0
    }
    #[doc = "Checks if the value of the field is `MOSI`"]
    #[inline(always)]
    pub fn is_mosi(&self) -> bool {
        *self == P0_18_A::MOSI
    }
}
#[doc = "Write proxy for field `P0_18`"]
pub struct P0_18_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_18_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.18"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_18_A::GPIO_P0)
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn dcd1(self) -> &'a mut W {
        self.variant(P0_18_A::DCD1)
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn mosi0(self) -> &'a mut W {
        self.variant(P0_18_A::MOSI0)
    }
    #[doc = "MOSI"]
    #[inline(always)]
    pub fn mosi(self) -> &'a mut W {
        self.variant(P0_18_A::MOSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Pin function select P019.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_19_A {
    #[doc = "0: GPIO P0.19."]
    GPIO_P0 = 0,
    #[doc = "1: DSR1"]
    DSR1 = 1,
    #[doc = "3: SDA1"]
    SDA1 = 3,
}
impl From<P0_19_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_19_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_19`"]
pub type P0_19_R = crate::R<u8, P0_19_A>;
impl P0_19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_19_A {
        match self.bits {
            0 => P0_19_A::GPIO_P0,
            1 => P0_19_A::DSR1,
            3 => P0_19_A::SDA1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_19_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `DSR1`"]
    #[inline(always)]
    pub fn is_dsr1(&self) -> bool {
        *self == P0_19_A::DSR1
    }
    #[doc = "Checks if the value of the field is `SDA1`"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == P0_19_A::SDA1
    }
}
#[doc = "Write proxy for field `P0_19`"]
pub struct P0_19_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_19_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.19."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_19_A::GPIO_P0)
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn dsr1(self) -> &'a mut W {
        self.variant(P0_19_A::DSR1)
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut W {
        self.variant(P0_19_A::SDA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pin function select P0.20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_20_A {
    #[doc = "0: GPIO P0.20."]
    GPIO_P0 = 0,
    #[doc = "1: DTR1"]
    DTR1 = 1,
    #[doc = "3: SCL1"]
    SCL1 = 3,
}
impl From<P0_20_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_20_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_20`"]
pub type P0_20_R = crate::R<u8, P0_20_A>;
impl P0_20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_20_A {
        match self.bits {
            0 => P0_20_A::GPIO_P0,
            1 => P0_20_A::DTR1,
            3 => P0_20_A::SCL1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_20_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `DTR1`"]
    #[inline(always)]
    pub fn is_dtr1(&self) -> bool {
        *self == P0_20_A::DTR1
    }
    #[doc = "Checks if the value of the field is `SCL1`"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == P0_20_A::SCL1
    }
}
#[doc = "Write proxy for field `P0_20`"]
pub struct P0_20_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_20_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.20."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_20_A::GPIO_P0)
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn dtr1(self) -> &'a mut W {
        self.variant(P0_20_A::DTR1)
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut W {
        self.variant(P0_20_A::SCL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Pin function select P0.21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_21_A {
    #[doc = "0: GPIO Port 0.21."]
    GPIO_PORT_0 = 0,
    #[doc = "1: RI1"]
    RI1 = 1,
    #[doc = "3: RD1"]
    RD1 = 3,
}
impl From<P0_21_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_21_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_21`"]
pub type P0_21_R = crate::R<u8, P0_21_A>;
impl P0_21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_21_A {
        match self.bits {
            0 => P0_21_A::GPIO_PORT_0,
            1 => P0_21_A::RI1,
            3 => P0_21_A::RD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline(always)]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == P0_21_A::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `RI1`"]
    #[inline(always)]
    pub fn is_ri1(&self) -> bool {
        *self == P0_21_A::RI1
    }
    #[doc = "Checks if the value of the field is `RD1`"]
    #[inline(always)]
    pub fn is_rd1(&self) -> bool {
        *self == P0_21_A::RD1
    }
}
#[doc = "Write proxy for field `P0_21`"]
pub struct P0_21_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_21_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO Port 0.21."]
    #[inline(always)]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(P0_21_A::GPIO_PORT_0)
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn ri1(self) -> &'a mut W {
        self.variant(P0_21_A::RI1)
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn rd1(self) -> &'a mut W {
        self.variant(P0_21_A::RD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Pin function select P022\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_22_A {
    #[doc = "0: GPIO P0.22."]
    GPIO_P0 = 0,
    #[doc = "1: RTS1"]
    RTS1 = 1,
    #[doc = "3: TD1"]
    TD1 = 3,
}
impl From<P0_22_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_22_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_22`"]
pub type P0_22_R = crate::R<u8, P0_22_A>;
impl P0_22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_22_A {
        match self.bits {
            0 => P0_22_A::GPIO_P0,
            1 => P0_22_A::RTS1,
            3 => P0_22_A::TD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_22_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RTS1`"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == P0_22_A::RTS1
    }
    #[doc = "Checks if the value of the field is `TD1`"]
    #[inline(always)]
    pub fn is_td1(&self) -> bool {
        *self == P0_22_A::TD1
    }
}
#[doc = "Write proxy for field `P0_22`"]
pub struct P0_22_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_22_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.22."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_22_A::GPIO_P0)
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut W {
        self.variant(P0_22_A::RTS1)
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn td1(self) -> &'a mut W {
        self.variant(P0_22_A::TD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Pin function select P023.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_23_A {
    #[doc = "0: GPIO P0.23."]
    GPIO_P0 = 0,
    #[doc = "1: AD0.0"]
    AD0 = 1,
    #[doc = "2: I2SRX_CLK"]
    I2SRX_CLK = 2,
    #[doc = "3: CAP3.0"]
    CAP3 = 3,
}
impl From<P0_23_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_23`"]
pub type P0_23_R = crate::R<u8, P0_23_A>;
impl P0_23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_23_A {
        match self.bits {
            0 => P0_23_A::GPIO_P0,
            1 => P0_23_A::AD0,
            2 => P0_23_A::I2SRX_CLK,
            3 => P0_23_A::CAP3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_23_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_23_A::AD0
    }
    #[doc = "Checks if the value of the field is `I2SRX_CLK`"]
    #[inline(always)]
    pub fn is_i2srx_clk(&self) -> bool {
        *self == P0_23_A::I2SRX_CLK
    }
    #[doc = "Checks if the value of the field is `CAP3`"]
    #[inline(always)]
    pub fn is_cap3(&self) -> bool {
        *self == P0_23_A::CAP3
    }
}
#[doc = "Write proxy for field `P0_23`"]
pub struct P0_23_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.23."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_23_A::GPIO_P0)
    }
    #[doc = "AD0.0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_23_A::AD0)
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn i2srx_clk(self) -> &'a mut W {
        self.variant(P0_23_A::I2SRX_CLK)
    }
    #[doc = "CAP3.0"]
    #[inline(always)]
    pub fn cap3(self) -> &'a mut W {
        self.variant(P0_23_A::CAP3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pin function select P0.24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_24_A {
    #[doc = "0: GPIO P0.24."]
    GPIO_P0 = 0,
    #[doc = "1: AD0.1"]
    AD0 = 1,
    #[doc = "2: I2SRX_WS"]
    I2SRX_WS = 2,
    #[doc = "3: CAP3.1"]
    CAP3 = 3,
}
impl From<P0_24_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_24_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_24`"]
pub type P0_24_R = crate::R<u8, P0_24_A>;
impl P0_24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_24_A {
        match self.bits {
            0 => P0_24_A::GPIO_P0,
            1 => P0_24_A::AD0,
            2 => P0_24_A::I2SRX_WS,
            3 => P0_24_A::CAP3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_24_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_24_A::AD0
    }
    #[doc = "Checks if the value of the field is `I2SRX_WS`"]
    #[inline(always)]
    pub fn is_i2srx_ws(&self) -> bool {
        *self == P0_24_A::I2SRX_WS
    }
    #[doc = "Checks if the value of the field is `CAP3`"]
    #[inline(always)]
    pub fn is_cap3(&self) -> bool {
        *self == P0_24_A::CAP3
    }
}
#[doc = "Write proxy for field `P0_24`"]
pub struct P0_24_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_24_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.24."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_24_A::GPIO_P0)
    }
    #[doc = "AD0.1"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_24_A::AD0)
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn i2srx_ws(self) -> &'a mut W {
        self.variant(P0_24_A::I2SRX_WS)
    }
    #[doc = "CAP3.1"]
    #[inline(always)]
    pub fn cap3(self) -> &'a mut W {
        self.variant(P0_24_A::CAP3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Pin function select P0.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_25_A {
    #[doc = "0: GPIO P0.25"]
    GPIO_P0 = 0,
    #[doc = "1: AD0.2"]
    AD0 = 1,
    #[doc = "2: I2SRX_SDA"]
    I2SRX_SDA = 2,
    #[doc = "3: TXD3"]
    TXD3 = 3,
}
impl From<P0_25_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_25_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_25`"]
pub type P0_25_R = crate::R<u8, P0_25_A>;
impl P0_25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_25_A {
        match self.bits {
            0 => P0_25_A::GPIO_P0,
            1 => P0_25_A::AD0,
            2 => P0_25_A::I2SRX_SDA,
            3 => P0_25_A::TXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_25_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_25_A::AD0
    }
    #[doc = "Checks if the value of the field is `I2SRX_SDA`"]
    #[inline(always)]
    pub fn is_i2srx_sda(&self) -> bool {
        *self == P0_25_A::I2SRX_SDA
    }
    #[doc = "Checks if the value of the field is `TXD3`"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P0_25_A::TXD3
    }
}
#[doc = "Write proxy for field `P0_25`"]
pub struct P0_25_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_25_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.25"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_25_A::GPIO_P0)
    }
    #[doc = "AD0.2"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_25_A::AD0)
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn i2srx_sda(self) -> &'a mut W {
        self.variant(P0_25_A::I2SRX_SDA)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut W {
        self.variant(P0_25_A::TXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Pin function select P0.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_26_A {
    #[doc = "0: GPIO P0.26"]
    GPIO_P0 = 0,
    #[doc = "1: AD0.3"]
    AD0 = 1,
    #[doc = "2: AOUT"]
    AOUT = 2,
    #[doc = "3: RXD3"]
    RXD3 = 3,
}
impl From<P0_26_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_26_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_26`"]
pub type P0_26_R = crate::R<u8, P0_26_A>;
impl P0_26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_26_A {
        match self.bits {
            0 => P0_26_A::GPIO_P0,
            1 => P0_26_A::AD0,
            2 => P0_26_A::AOUT,
            3 => P0_26_A::RXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_26_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_26_A::AD0
    }
    #[doc = "Checks if the value of the field is `AOUT`"]
    #[inline(always)]
    pub fn is_aout(&self) -> bool {
        *self == P0_26_A::AOUT
    }
    #[doc = "Checks if the value of the field is `RXD3`"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P0_26_A::RXD3
    }
}
#[doc = "Write proxy for field `P0_26`"]
pub struct P0_26_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_26_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.26"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_26_A::GPIO_P0)
    }
    #[doc = "AD0.3"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_26_A::AD0)
    }
    #[doc = "AOUT"]
    #[inline(always)]
    pub fn aout(self) -> &'a mut W {
        self.variant(P0_26_A::AOUT)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut W {
        self.variant(P0_26_A::RXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Pin function select P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_27_A {
    #[doc = "0: GPIO P0.27"]
    GPIO_P0 = 0,
    #[doc = "1: SDA0"]
    SDA0 = 1,
    #[doc = "2: USB_SDA"]
    USB_SDA = 2,
}
impl From<P0_27_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_27_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_27`"]
pub type P0_27_R = crate::R<u8, P0_27_A>;
impl P0_27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_27_A {
        match self.bits {
            0 => P0_27_A::GPIO_P0,
            1 => P0_27_A::SDA0,
            2 => P0_27_A::USB_SDA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_27_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `SDA0`"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == P0_27_A::SDA0
    }
    #[doc = "Checks if the value of the field is `USB_SDA`"]
    #[inline(always)]
    pub fn is_usb_sda(&self) -> bool {
        *self == P0_27_A::USB_SDA
    }
}
#[doc = "Write proxy for field `P0_27`"]
pub struct P0_27_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_27_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.27"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_27_A::GPIO_P0)
    }
    #[doc = "SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut W {
        self.variant(P0_27_A::SDA0)
    }
    #[doc = "USB_SDA"]
    #[inline(always)]
    pub fn usb_sda(self) -> &'a mut W {
        self.variant(P0_27_A::USB_SDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pin function select P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_28_A {
    #[doc = "0: GPIO P0.28"]
    GPIO_P0 = 0,
    #[doc = "1: SCL0"]
    SCL0 = 1,
    #[doc = "2: USB_SCL"]
    USB_SCL = 2,
}
impl From<P0_28_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_28_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_28`"]
pub type P0_28_R = crate::R<u8, P0_28_A>;
impl P0_28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_28_A {
        match self.bits {
            0 => P0_28_A::GPIO_P0,
            1 => P0_28_A::SCL0,
            2 => P0_28_A::USB_SCL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_28_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `SCL0`"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == P0_28_A::SCL0
    }
    #[doc = "Checks if the value of the field is `USB_SCL`"]
    #[inline(always)]
    pub fn is_usb_scl(&self) -> bool {
        *self == P0_28_A::USB_SCL
    }
}
#[doc = "Write proxy for field `P0_28`"]
pub struct P0_28_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_28_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.28"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_28_A::GPIO_P0)
    }
    #[doc = "SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut W {
        self.variant(P0_28_A::SCL0)
    }
    #[doc = "USB_SCL"]
    #[inline(always)]
    pub fn usb_scl(self) -> &'a mut W {
        self.variant(P0_28_A::USB_SCL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Pin function select P0.29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_29_A {
    #[doc = "0: GPIO P0.29"]
    GPIO_P0 = 0,
    #[doc = "1: USB_D+"]
    USB_DP = 1,
}
impl From<P0_29_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_29_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_29`"]
pub type P0_29_R = crate::R<u8, P0_29_A>;
impl P0_29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_29_A {
        match self.bits {
            0 => P0_29_A::GPIO_P0,
            1 => P0_29_A::USB_DP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_29_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `USB_DP`"]
    #[inline(always)]
    pub fn is_usb_dp(&self) -> bool {
        *self == P0_29_A::USB_DP
    }
}
#[doc = "Write proxy for field `P0_29`"]
pub struct P0_29_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_29_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.29"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_29_A::GPIO_P0)
    }
    #[doc = "USB_D+"]
    #[inline(always)]
    pub fn usb_dp(self) -> &'a mut W {
        self.variant(P0_29_A::USB_DP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Pin function select P0.30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_30_A {
    #[doc = "0: GPIO P0.30"]
    GPIO_P0 = 0,
    #[doc = "1: USB_D-"]
    USB_DM = 1,
}
impl From<P0_30_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_30_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_30`"]
pub type P0_30_R = crate::R<u8, P0_30_A>;
impl P0_30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_30_A {
        match self.bits {
            0 => P0_30_A::GPIO_P0,
            1 => P0_30_A::USB_DM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_30_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `USB_DM`"]
    #[inline(always)]
    pub fn is_usb_dm(&self) -> bool {
        *self == P0_30_A::USB_DM
    }
}
#[doc = "Write proxy for field `P0_30`"]
pub struct P0_30_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_30_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.30"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_30_A::GPIO_P0)
    }
    #[doc = "USB_D-"]
    #[inline(always)]
    pub fn usb_dm(self) -> &'a mut W {
        self.variant(P0_30_A::USB_DM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P0.16."]
    #[inline(always)]
    pub fn p0_16(&self) -> P0_16_R {
        P0_16_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P0.17."]
    #[inline(always)]
    pub fn p0_17(&self) -> P0_17_R {
        P0_17_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P0.18."]
    #[inline(always)]
    pub fn p0_18(&self) -> P0_18_R {
        P0_18_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P019."]
    #[inline(always)]
    pub fn p0_19(&self) -> P0_19_R {
        P0_19_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P0.20."]
    #[inline(always)]
    pub fn p0_20(&self) -> P0_20_R {
        P0_20_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.21."]
    #[inline(always)]
    pub fn p0_21(&self) -> P0_21_R {
        P0_21_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P022"]
    #[inline(always)]
    pub fn p0_22(&self) -> P0_22_R {
        P0_22_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P023."]
    #[inline(always)]
    pub fn p0_23(&self) -> P0_23_R {
        P0_23_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P0.24."]
    #[inline(always)]
    pub fn p0_24(&self) -> P0_24_R {
        P0_24_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P0.25."]
    #[inline(always)]
    pub fn p0_25(&self) -> P0_25_R {
        P0_25_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P0.26."]
    #[inline(always)]
    pub fn p0_26(&self) -> P0_26_R {
        P0_26_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P0.27."]
    #[inline(always)]
    pub fn p0_27(&self) -> P0_27_R {
        P0_27_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P0.28."]
    #[inline(always)]
    pub fn p0_28(&self) -> P0_28_R {
        P0_28_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P0.29"]
    #[inline(always)]
    pub fn p0_29(&self) -> P0_29_R {
        P0_29_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Pin function select P0.30."]
    #[inline(always)]
    pub fn p0_30(&self) -> P0_30_R {
        P0_30_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P0.16."]
    #[inline(always)]
    pub fn p0_16(&mut self) -> P0_16_W {
        P0_16_W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P0.17."]
    #[inline(always)]
    pub fn p0_17(&mut self) -> P0_17_W {
        P0_17_W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P0.18."]
    #[inline(always)]
    pub fn p0_18(&mut self) -> P0_18_W {
        P0_18_W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P019."]
    #[inline(always)]
    pub fn p0_19(&mut self) -> P0_19_W {
        P0_19_W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P0.20."]
    #[inline(always)]
    pub fn p0_20(&mut self) -> P0_20_W {
        P0_20_W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P0.21."]
    #[inline(always)]
    pub fn p0_21(&mut self) -> P0_21_W {
        P0_21_W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P022"]
    #[inline(always)]
    pub fn p0_22(&mut self) -> P0_22_W {
        P0_22_W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P023."]
    #[inline(always)]
    pub fn p0_23(&mut self) -> P0_23_W {
        P0_23_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P0.24."]
    #[inline(always)]
    pub fn p0_24(&mut self) -> P0_24_W {
        P0_24_W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P0.25."]
    #[inline(always)]
    pub fn p0_25(&mut self) -> P0_25_W {
        P0_25_W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P0.26."]
    #[inline(always)]
    pub fn p0_26(&mut self) -> P0_26_W {
        P0_26_W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P0.27."]
    #[inline(always)]
    pub fn p0_27(&mut self) -> P0_27_W {
        P0_27_W { w: self }
    }
    #[doc = "Bits 24:25 - Pin function select P0.28."]
    #[inline(always)]
    pub fn p0_28(&mut self) -> P0_28_W {
        P0_28_W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P0.29"]
    #[inline(always)]
    pub fn p0_29(&mut self) -> P0_29_W {
        P0_29_W { w: self }
    }
    #[doc = "Bits 28:29 - Pin function select P0.30."]
    #[inline(always)]
    pub fn p0_30(&mut self) -> P0_30_W {
        P0_30_W { w: self }
    }
}
