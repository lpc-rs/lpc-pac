#[doc = "Reader of register PINSEL0"]
pub type R = crate::R<u32, super::PINSEL0>;
#[doc = "Writer for register PINSEL0"]
pub type W = crate::W<u32, super::PINSEL0>;
#[doc = "Register PINSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin function select P0.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_0_A {
    #[doc = "0: GPIO P0.0"]
    GPIO_P0 = 0,
    #[doc = "1: RD1"]
    RD1 = 1,
    #[doc = "2: TXD3"]
    TXD3 = 2,
    #[doc = "3: SDA1"]
    SDA1 = 3,
}
impl From<P0_0_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_0`"]
pub type P0_0_R = crate::R<u8, P0_0_A>;
impl P0_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_0_A {
        match self.bits {
            0 => P0_0_A::GPIO_P0,
            1 => P0_0_A::RD1,
            2 => P0_0_A::TXD3,
            3 => P0_0_A::SDA1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_0_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RD1`"]
    #[inline(always)]
    pub fn is_rd1(&self) -> bool {
        *self == P0_0_A::RD1
    }
    #[doc = "Checks if the value of the field is `TXD3`"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P0_0_A::TXD3
    }
    #[doc = "Checks if the value of the field is `SDA1`"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == P0_0_A::SDA1
    }
}
#[doc = "Write proxy for field `P0_0`"]
pub struct P0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.0"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_0_A::GPIO_P0)
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn rd1(self) -> &'a mut W {
        self.variant(P0_0_A::RD1)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut W {
        self.variant(P0_0_A::TXD3)
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut W {
        self.variant(P0_0_A::SDA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Pin function select P0.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_1_A {
    #[doc = "0: GPIO P0.1"]
    GPIO_P0 = 0,
    #[doc = "1: TD1"]
    TD1 = 1,
    #[doc = "2: RXD3"]
    RXD3 = 2,
    #[doc = "3: SCL1"]
    SCL1 = 3,
}
impl From<P0_1_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_1`"]
pub type P0_1_R = crate::R<u8, P0_1_A>;
impl P0_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_1_A {
        match self.bits {
            0 => P0_1_A::GPIO_P0,
            1 => P0_1_A::TD1,
            2 => P0_1_A::RXD3,
            3 => P0_1_A::SCL1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_1_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TD1`"]
    #[inline(always)]
    pub fn is_td1(&self) -> bool {
        *self == P0_1_A::TD1
    }
    #[doc = "Checks if the value of the field is `RXD3`"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P0_1_A::RXD3
    }
    #[doc = "Checks if the value of the field is `SCL1`"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == P0_1_A::SCL1
    }
}
#[doc = "Write proxy for field `P0_1`"]
pub struct P0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.1"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_1_A::GPIO_P0)
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn td1(self) -> &'a mut W {
        self.variant(P0_1_A::TD1)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut W {
        self.variant(P0_1_A::RXD3)
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut W {
        self.variant(P0_1_A::SCL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Pin function select P0.2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_2_A {
    #[doc = "0: GPIO P0.2"]
    GPIO_P0 = 0,
    #[doc = "1: TXD0"]
    TXD0 = 1,
    #[doc = "2: AD0.7"]
    AD0 = 2,
}
impl From<P0_2_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_2`"]
pub type P0_2_R = crate::R<u8, P0_2_A>;
impl P0_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_2_A {
        match self.bits {
            0 => P0_2_A::GPIO_P0,
            1 => P0_2_A::TXD0,
            2 => P0_2_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_2_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TXD0`"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == P0_2_A::TXD0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_2_A::AD0
    }
}
#[doc = "Write proxy for field `P0_2`"]
pub struct P0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.2"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_2_A::GPIO_P0)
    }
    #[doc = "TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut W {
        self.variant(P0_2_A::TXD0)
    }
    #[doc = "AD0.7"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_2_A::AD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Pin function select P0.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_3_A {
    #[doc = "0: GPIO P0.3."]
    GPIO_P0 = 0,
    #[doc = "1: RXD0"]
    RXD0 = 1,
    #[doc = "2: AD0.6"]
    AD0 = 2,
}
impl From<P0_3_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_3`"]
pub type P0_3_R = crate::R<u8, P0_3_A>;
impl P0_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_3_A {
        match self.bits {
            0 => P0_3_A::GPIO_P0,
            1 => P0_3_A::RXD0,
            2 => P0_3_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_3_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RXD0`"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == P0_3_A::RXD0
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_3_A::AD0
    }
}
#[doc = "Write proxy for field `P0_3`"]
pub struct P0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P0.3."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_3_A::GPIO_P0)
    }
    #[doc = "RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut W {
        self.variant(P0_3_A::RXD0)
    }
    #[doc = "AD0.6"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(P0_3_A::AD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pin function select P0.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_4_A {
    #[doc = "0: GPIO P0.4."]
    GPIO_P0 = 0,
    #[doc = "1: I2SRX_CLK"]
    I2SRX_CLK = 1,
    #[doc = "2: RD2"]
    RD2 = 2,
    #[doc = "3: CAP2.0"]
    CAP2 = 3,
}
impl From<P0_4_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_4`"]
pub type P0_4_R = crate::R<u8, P0_4_A>;
impl P0_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_4_A {
        match self.bits {
            0 => P0_4_A::GPIO_P0,
            1 => P0_4_A::I2SRX_CLK,
            2 => P0_4_A::RD2,
            3 => P0_4_A::CAP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_4_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2SRX_CLK`"]
    #[inline(always)]
    pub fn is_i2srx_clk(&self) -> bool {
        *self == P0_4_A::I2SRX_CLK
    }
    #[doc = "Checks if the value of the field is `RD2`"]
    #[inline(always)]
    pub fn is_rd2(&self) -> bool {
        *self == P0_4_A::RD2
    }
    #[doc = "Checks if the value of the field is `CAP2`"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_4_A::CAP2
    }
}
#[doc = "Write proxy for field `P0_4`"]
pub struct P0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.4."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_4_A::GPIO_P0)
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn i2srx_clk(self) -> &'a mut W {
        self.variant(P0_4_A::I2SRX_CLK)
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn rd2(self) -> &'a mut W {
        self.variant(P0_4_A::RD2)
    }
    #[doc = "CAP2.0"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut W {
        self.variant(P0_4_A::CAP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Pin function select P0.5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_5_A {
    #[doc = "0: GPIO P0.5."]
    GPIO_P0 = 0,
    #[doc = "1: I2SRX_WS"]
    I2SRX_WS = 1,
    #[doc = "2: TD2"]
    TD2 = 2,
    #[doc = "3: CAP2.1"]
    CAP2 = 3,
}
impl From<P0_5_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_5`"]
pub type P0_5_R = crate::R<u8, P0_5_A>;
impl P0_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_5_A {
        match self.bits {
            0 => P0_5_A::GPIO_P0,
            1 => P0_5_A::I2SRX_WS,
            2 => P0_5_A::TD2,
            3 => P0_5_A::CAP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_5_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2SRX_WS`"]
    #[inline(always)]
    pub fn is_i2srx_ws(&self) -> bool {
        *self == P0_5_A::I2SRX_WS
    }
    #[doc = "Checks if the value of the field is `TD2`"]
    #[inline(always)]
    pub fn is_td2(&self) -> bool {
        *self == P0_5_A::TD2
    }
    #[doc = "Checks if the value of the field is `CAP2`"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_5_A::CAP2
    }
}
#[doc = "Write proxy for field `P0_5`"]
pub struct P0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.5."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_5_A::GPIO_P0)
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn i2srx_ws(self) -> &'a mut W {
        self.variant(P0_5_A::I2SRX_WS)
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn td2(self) -> &'a mut W {
        self.variant(P0_5_A::TD2)
    }
    #[doc = "CAP2.1"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut W {
        self.variant(P0_5_A::CAP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Pin function select P0.6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_6_A {
    #[doc = "0: GPIO P0.6."]
    GPIO_P0 = 0,
    #[doc = "1: I2SRX_SDA"]
    I2SRX_SDA = 1,
    #[doc = "2: SSEL1"]
    SSEL1 = 2,
    #[doc = "3: MAT2.0"]
    MAT2 = 3,
}
impl From<P0_6_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_6`"]
pub type P0_6_R = crate::R<u8, P0_6_A>;
impl P0_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_6_A {
        match self.bits {
            0 => P0_6_A::GPIO_P0,
            1 => P0_6_A::I2SRX_SDA,
            2 => P0_6_A::SSEL1,
            3 => P0_6_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_6_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2SRX_SDA`"]
    #[inline(always)]
    pub fn is_i2srx_sda(&self) -> bool {
        *self == P0_6_A::I2SRX_SDA
    }
    #[doc = "Checks if the value of the field is `SSEL1`"]
    #[inline(always)]
    pub fn is_ssel1(&self) -> bool {
        *self == P0_6_A::SSEL1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_6_A::MAT2
    }
}
#[doc = "Write proxy for field `P0_6`"]
pub struct P0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.6."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_6_A::GPIO_P0)
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn i2srx_sda(self) -> &'a mut W {
        self.variant(P0_6_A::I2SRX_SDA)
    }
    #[doc = "SSEL1"]
    #[inline(always)]
    pub fn ssel1(self) -> &'a mut W {
        self.variant(P0_6_A::SSEL1)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_6_A::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Pin function select P0.7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_7_A {
    #[doc = "0: GPIO P0.7."]
    GPIO_P0 = 0,
    #[doc = "1: I2STX_CLK"]
    I2STX_CLK = 1,
    #[doc = "2: SCK1"]
    SCK1 = 2,
    #[doc = "3: MAT2.1"]
    MAT2 = 3,
}
impl From<P0_7_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_7`"]
pub type P0_7_R = crate::R<u8, P0_7_A>;
impl P0_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_7_A {
        match self.bits {
            0 => P0_7_A::GPIO_P0,
            1 => P0_7_A::I2STX_CLK,
            2 => P0_7_A::SCK1,
            3 => P0_7_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_7_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2STX_CLK`"]
    #[inline(always)]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P0_7_A::I2STX_CLK
    }
    #[doc = "Checks if the value of the field is `SCK1`"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P0_7_A::SCK1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_7_A::MAT2
    }
}
#[doc = "Write proxy for field `P0_7`"]
pub struct P0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.7."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_7_A::GPIO_P0)
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn i2stx_clk(self) -> &'a mut W {
        self.variant(P0_7_A::I2STX_CLK)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut W {
        self.variant(P0_7_A::SCK1)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_7_A::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pin function select P0.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_8_A {
    #[doc = "0: GPIO P0.8."]
    GPIO_P0 = 0,
    #[doc = "1: I2STX_WS"]
    I2STX_WS = 1,
    #[doc = "2: MISO1"]
    MISO1 = 2,
    #[doc = "3: MAT2.2"]
    MAT2 = 3,
}
impl From<P0_8_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_8`"]
pub type P0_8_R = crate::R<u8, P0_8_A>;
impl P0_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_8_A {
        match self.bits {
            0 => P0_8_A::GPIO_P0,
            1 => P0_8_A::I2STX_WS,
            2 => P0_8_A::MISO1,
            3 => P0_8_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_8_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2STX_WS`"]
    #[inline(always)]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P0_8_A::I2STX_WS
    }
    #[doc = "Checks if the value of the field is `MISO1`"]
    #[inline(always)]
    pub fn is_miso1(&self) -> bool {
        *self == P0_8_A::MISO1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_8_A::MAT2
    }
}
#[doc = "Write proxy for field `P0_8`"]
pub struct P0_8_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.8."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_8_A::GPIO_P0)
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn i2stx_ws(self) -> &'a mut W {
        self.variant(P0_8_A::I2STX_WS)
    }
    #[doc = "MISO1"]
    #[inline(always)]
    pub fn miso1(self) -> &'a mut W {
        self.variant(P0_8_A::MISO1)
    }
    #[doc = "MAT2.2"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_8_A::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Pin function select P0.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_9_A {
    #[doc = "0: GPIO P0.9"]
    GPIO_P0 = 0,
    #[doc = "1: I2STX_SDA"]
    I2STX_SDA = 1,
    #[doc = "2: MOSI1"]
    MOSI1 = 2,
    #[doc = "3: MAT2.3"]
    MAT2 = 3,
}
impl From<P0_9_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_9`"]
pub type P0_9_R = crate::R<u8, P0_9_A>;
impl P0_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_9_A {
        match self.bits {
            0 => P0_9_A::GPIO_P0,
            1 => P0_9_A::I2STX_SDA,
            2 => P0_9_A::MOSI1,
            3 => P0_9_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_9_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `I2STX_SDA`"]
    #[inline(always)]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P0_9_A::I2STX_SDA
    }
    #[doc = "Checks if the value of the field is `MOSI1`"]
    #[inline(always)]
    pub fn is_mosi1(&self) -> bool {
        *self == P0_9_A::MOSI1
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_9_A::MAT2
    }
}
#[doc = "Write proxy for field `P0_9`"]
pub struct P0_9_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.9"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_9_A::GPIO_P0)
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn i2stx_sda(self) -> &'a mut W {
        self.variant(P0_9_A::I2STX_SDA)
    }
    #[doc = "MOSI1"]
    #[inline(always)]
    pub fn mosi1(self) -> &'a mut W {
        self.variant(P0_9_A::MOSI1)
    }
    #[doc = "MAT2.3"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P0_9_A::MAT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Pin function select P0.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_10_A {
    #[doc = "0: GPIO P0.10"]
    GPIO_P0 = 0,
    #[doc = "1: TXD2"]
    TXD2 = 1,
    #[doc = "2: SDA2"]
    SDA2 = 2,
    #[doc = "3: MAT3.0"]
    MAT3 = 3,
}
impl From<P0_10_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_10`"]
pub type P0_10_R = crate::R<u8, P0_10_A>;
impl P0_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_10_A {
        match self.bits {
            0 => P0_10_A::GPIO_P0,
            1 => P0_10_A::TXD2,
            2 => P0_10_A::SDA2,
            3 => P0_10_A::MAT3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_10_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TXD2`"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == P0_10_A::TXD2
    }
    #[doc = "Checks if the value of the field is `SDA2`"]
    #[inline(always)]
    pub fn is_sda2(&self) -> bool {
        *self == P0_10_A::SDA2
    }
    #[doc = "Checks if the value of the field is `MAT3`"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_10_A::MAT3
    }
}
#[doc = "Write proxy for field `P0_10`"]
pub struct P0_10_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.10"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_10_A::GPIO_P0)
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut W {
        self.variant(P0_10_A::TXD2)
    }
    #[doc = "SDA2"]
    #[inline(always)]
    pub fn sda2(self) -> &'a mut W {
        self.variant(P0_10_A::SDA2)
    }
    #[doc = "MAT3.0"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut W {
        self.variant(P0_10_A::MAT3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Pin function select P0.11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_11_A {
    #[doc = "0: GPIO P0.11"]
    GPIO_P0 = 0,
    #[doc = "1: RXD2"]
    RXD2 = 1,
    #[doc = "2: SCL2"]
    SCL2 = 2,
    #[doc = "3: MAT3.1"]
    MAT3 = 3,
}
impl From<P0_11_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_11`"]
pub type P0_11_R = crate::R<u8, P0_11_A>;
impl P0_11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_11_A {
        match self.bits {
            0 => P0_11_A::GPIO_P0,
            1 => P0_11_A::RXD2,
            2 => P0_11_A::SCL2,
            3 => P0_11_A::MAT3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_11_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `RXD2`"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == P0_11_A::RXD2
    }
    #[doc = "Checks if the value of the field is `SCL2`"]
    #[inline(always)]
    pub fn is_scl2(&self) -> bool {
        *self == P0_11_A::SCL2
    }
    #[doc = "Checks if the value of the field is `MAT3`"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_11_A::MAT3
    }
}
#[doc = "Write proxy for field `P0_11`"]
pub struct P0_11_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.11"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_11_A::GPIO_P0)
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut W {
        self.variant(P0_11_A::RXD2)
    }
    #[doc = "SCL2"]
    #[inline(always)]
    pub fn scl2(self) -> &'a mut W {
        self.variant(P0_11_A::SCL2)
    }
    #[doc = "MAT3.1"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut W {
        self.variant(P0_11_A::MAT3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pin function select P0.15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_15_A {
    #[doc = "0: GPIO P0.15"]
    GPIO_P0 = 0,
    #[doc = "1: TXD1"]
    TXD1 = 1,
    #[doc = "2: SCK0"]
    SCK0 = 2,
    #[doc = "3: SCK"]
    SCK = 3,
}
impl From<P0_15_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_15`"]
pub type P0_15_R = crate::R<u8, P0_15_A>;
impl P0_15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_15_A {
        match self.bits {
            0 => P0_15_A::GPIO_P0,
            1 => P0_15_A::TXD1,
            2 => P0_15_A::SCK0,
            3 => P0_15_A::SCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P0`"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_15_A::GPIO_P0
    }
    #[doc = "Checks if the value of the field is `TXD1`"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == P0_15_A::TXD1
    }
    #[doc = "Checks if the value of the field is `SCK0`"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P0_15_A::SCK0
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == P0_15_A::SCK
    }
}
#[doc = "Write proxy for field `P0_15`"]
pub struct P0_15_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P0.15"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut W {
        self.variant(P0_15_A::GPIO_P0)
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut W {
        self.variant(P0_15_A::TXD1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut W {
        self.variant(P0_15_A::SCK0)
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(P0_15_A::SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    pub fn p0_0(&self) -> P0_0_R {
        P0_0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    pub fn p0_1(&self) -> P0_1_R {
        P0_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    pub fn p0_2(&self) -> P0_2_R {
        P0_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    pub fn p0_3(&self) -> P0_3_R {
        P0_3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    pub fn p0_4(&self) -> P0_4_R {
        P0_4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    pub fn p0_5(&self) -> P0_5_R {
        P0_5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    pub fn p0_6(&self) -> P0_6_R {
        P0_6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    pub fn p0_7(&self) -> P0_7_R {
        P0_7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    pub fn p0_8(&self) -> P0_8_R {
        P0_8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    pub fn p0_9(&self) -> P0_9_R {
        P0_9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    pub fn p0_10(&self) -> P0_10_R {
        P0_10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    pub fn p0_11(&self) -> P0_11_R {
        P0_11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    pub fn p0_15(&self) -> P0_15_R {
        P0_15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    pub fn p0_0(&mut self) -> P0_0_W {
        P0_0_W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    pub fn p0_1(&mut self) -> P0_1_W {
        P0_1_W { w: self }
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    pub fn p0_2(&mut self) -> P0_2_W {
        P0_2_W { w: self }
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    pub fn p0_3(&mut self) -> P0_3_W {
        P0_3_W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    pub fn p0_4(&mut self) -> P0_4_W {
        P0_4_W { w: self }
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    pub fn p0_5(&mut self) -> P0_5_W {
        P0_5_W { w: self }
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    pub fn p0_6(&mut self) -> P0_6_W {
        P0_6_W { w: self }
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    pub fn p0_7(&mut self) -> P0_7_W {
        P0_7_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    pub fn p0_8(&mut self) -> P0_8_W {
        P0_8_W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    pub fn p0_9(&mut self) -> P0_9_W {
        P0_9_W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    pub fn p0_10(&mut self) -> P0_10_W {
        P0_10_W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    pub fn p0_11(&mut self) -> P0_11_W {
        P0_11_W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    pub fn p0_15(&mut self) -> P0_15_W {
        P0_15_W { w: self }
    }
}
