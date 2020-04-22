#[doc = "Reader of register PINSEL2"]
pub type R = crate::R<u32, super::PINSEL2>;
#[doc = "Writer for register PINSEL2"]
pub type W = crate::W<u32, super::PINSEL2>;
#[doc = "Register PINSEL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin function select P1.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_0_A {
    #[doc = "0: GPIO P1.0"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_TXD0"]
    ENET_TXD0 = 1,
}
impl From<P1_0_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_0`"]
pub type P1_0_R = crate::R<u8, P1_0_A>;
impl P1_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_0_A {
        match self.bits {
            0 => P1_0_A::GPIO_P1,
            1 => P1_0_A::ENET_TXD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_0_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TXD0`"]
    #[inline(always)]
    pub fn is_enet_txd0(&self) -> bool {
        *self == P1_0_A::ENET_TXD0
    }
}
#[doc = "Write proxy for field `P1_0`"]
pub struct P1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.0"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_0_A::GPIO_P1)
    }
    #[doc = "ENET_TXD0"]
    #[inline(always)]
    pub fn enet_txd0(self) -> &'a mut W {
        self.variant(P1_0_A::ENET_TXD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Pin function select P1.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_1_A {
    #[doc = "0: GPIO P1.1"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_TXD1"]
    ENET_TXD1 = 1,
}
impl From<P1_1_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_1`"]
pub type P1_1_R = crate::R<u8, P1_1_A>;
impl P1_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_1_A {
        match self.bits {
            0 => P1_1_A::GPIO_P1,
            1 => P1_1_A::ENET_TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_1_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TXD1`"]
    #[inline(always)]
    pub fn is_enet_txd1(&self) -> bool {
        *self == P1_1_A::ENET_TXD1
    }
}
#[doc = "Write proxy for field `P1_1`"]
pub struct P1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.1"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_1_A::GPIO_P1)
    }
    #[doc = "ENET_TXD1"]
    #[inline(always)]
    pub fn enet_txd1(self) -> &'a mut W {
        self.variant(P1_1_A::ENET_TXD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Pin function select P1.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_4_A {
    #[doc = "0: GPIO P1.4."]
    GPIO_P1 = 0,
    #[doc = "1: ENET_TX_EN"]
    ENET_TX_EN = 1,
}
impl From<P1_4_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_4`"]
pub type P1_4_R = crate::R<u8, P1_4_A>;
impl P1_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_4_A {
        match self.bits {
            0 => P1_4_A::GPIO_P1,
            1 => P1_4_A::ENET_TX_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_4_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TX_EN`"]
    #[inline(always)]
    pub fn is_enet_tx_en(&self) -> bool {
        *self == P1_4_A::ENET_TX_EN
    }
}
#[doc = "Write proxy for field `P1_4`"]
pub struct P1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.4."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_4_A::GPIO_P1)
    }
    #[doc = "ENET_TX_EN"]
    #[inline(always)]
    pub fn enet_tx_en(self) -> &'a mut W {
        self.variant(P1_4_A::ENET_TX_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Pin function select P1.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_8_A {
    #[doc = "0: GPIO P1.8."]
    GPIO_P1 = 0,
    #[doc = "1: ENET_CRS"]
    ENET_CRS = 1,
}
impl From<P1_8_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_8`"]
pub type P1_8_R = crate::R<u8, P1_8_A>;
impl P1_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_8_A {
        match self.bits {
            0 => P1_8_A::GPIO_P1,
            1 => P1_8_A::ENET_CRS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_8_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_CRS`"]
    #[inline(always)]
    pub fn is_enet_crs(&self) -> bool {
        *self == P1_8_A::ENET_CRS
    }
}
#[doc = "Write proxy for field `P1_8`"]
pub struct P1_8_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.8."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_8_A::GPIO_P1)
    }
    #[doc = "ENET_CRS"]
    #[inline(always)]
    pub fn enet_crs(self) -> &'a mut W {
        self.variant(P1_8_A::ENET_CRS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Pin function select P1.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_9_A {
    #[doc = "0: GPIO Port 1.9"]
    GPIO_PORT_1 = 0,
    #[doc = "1: ENET_RXD0"]
    ENET_RXD0 = 1,
}
impl From<P1_9_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_9`"]
pub type P1_9_R = crate::R<u8, P1_9_A>;
impl P1_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_9_A {
        match self.bits {
            0 => P1_9_A::GPIO_PORT_1,
            1 => P1_9_A::ENET_RXD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline(always)]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_9_A::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `ENET_RXD0`"]
    #[inline(always)]
    pub fn is_enet_rxd0(&self) -> bool {
        *self == P1_9_A::ENET_RXD0
    }
}
#[doc = "Write proxy for field `P1_9`"]
pub struct P1_9_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO Port 1.9"]
    #[inline(always)]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(P1_9_A::GPIO_PORT_1)
    }
    #[doc = "ENET_RXD0"]
    #[inline(always)]
    pub fn enet_rxd0(self) -> &'a mut W {
        self.variant(P1_9_A::ENET_RXD0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Pin function select P1.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_10_A {
    #[doc = "0: GPIO P1.10"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_RXD1"]
    ENET_RXD1 = 1,
}
impl From<P1_10_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_10`"]
pub type P1_10_R = crate::R<u8, P1_10_A>;
impl P1_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_10_A {
        match self.bits {
            0 => P1_10_A::GPIO_P1,
            1 => P1_10_A::ENET_RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_10_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_RXD1`"]
    #[inline(always)]
    pub fn is_enet_rxd1(&self) -> bool {
        *self == P1_10_A::ENET_RXD1
    }
}
#[doc = "Write proxy for field `P1_10`"]
pub struct P1_10_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.10"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_10_A::GPIO_P1)
    }
    #[doc = "ENET_RXD1"]
    #[inline(always)]
    pub fn enet_rxd1(self) -> &'a mut W {
        self.variant(P1_10_A::ENET_RXD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Pin function select P1.14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_14_A {
    #[doc = "0: GPIO P1.14"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_RX_ER"]
    ENET_RX_ER = 1,
}
impl From<P1_14_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_14`"]
pub type P1_14_R = crate::R<u8, P1_14_A>;
impl P1_14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_14_A {
        match self.bits {
            0 => P1_14_A::GPIO_P1,
            1 => P1_14_A::ENET_RX_ER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_14_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_RX_ER`"]
    #[inline(always)]
    pub fn is_enet_rx_er(&self) -> bool {
        *self == P1_14_A::ENET_RX_ER
    }
}
#[doc = "Write proxy for field `P1_14`"]
pub struct P1_14_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.14"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_14_A::GPIO_P1)
    }
    #[doc = "ENET_RX_ER"]
    #[inline(always)]
    pub fn enet_rx_er(self) -> &'a mut W {
        self.variant(P1_14_A::ENET_RX_ER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pin function select P1.15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_15_A {
    #[doc = "0: GPIO P1.15"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_REF_CLK"]
    ENET_REF_CLK = 1,
}
impl From<P1_15_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_15`"]
pub type P1_15_R = crate::R<u8, P1_15_A>;
impl P1_15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_15_A {
        match self.bits {
            0 => P1_15_A::GPIO_P1,
            1 => P1_15_A::ENET_REF_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_15_A::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_REF_CLK`"]
    #[inline(always)]
    pub fn is_enet_ref_clk(&self) -> bool {
        *self == P1_15_A::ENET_REF_CLK
    }
}
#[doc = "Write proxy for field `P1_15`"]
pub struct P1_15_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO P1.15"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_15_A::GPIO_P1)
    }
    #[doc = "ENET_REF_CLK"]
    #[inline(always)]
    pub fn enet_ref_clk(self) -> &'a mut W {
        self.variant(P1_15_A::ENET_REF_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    pub fn p1_0(&self) -> P1_0_R {
        P1_0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    pub fn p1_1(&self) -> P1_1_R {
        P1_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    pub fn p1_4(&self) -> P1_4_R {
        P1_4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    pub fn p1_8(&self) -> P1_8_R {
        P1_8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    pub fn p1_9(&self) -> P1_9_R {
        P1_9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    pub fn p1_10(&self) -> P1_10_R {
        P1_10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    pub fn p1_14(&self) -> P1_14_R {
        P1_14_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    pub fn p1_15(&self) -> P1_15_R {
        P1_15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    pub fn p1_0(&mut self) -> P1_0_W {
        P1_0_W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    pub fn p1_1(&mut self) -> P1_1_W {
        P1_1_W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    pub fn p1_4(&mut self) -> P1_4_W {
        P1_4_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    pub fn p1_8(&mut self) -> P1_8_W {
        P1_8_W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    pub fn p1_9(&mut self) -> P1_9_W {
        P1_9_W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    pub fn p1_10(&mut self) -> P1_10_W {
        P1_10_W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    pub fn p1_14(&mut self) -> P1_14_W {
        P1_14_W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    pub fn p1_15(&mut self) -> P1_15_W {
        P1_15_W { w: self }
    }
}
