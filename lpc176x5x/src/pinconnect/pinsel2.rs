#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL2 {
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
#[doc = "Possible values of the field `P1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_0R {
    #[doc = "GPIO P1.0"]
    GPIO_P1,
    #[doc = "ENET_TXD0"]
    ENET_TXD0,
}
impl P1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_0R::GPIO_P1 => 0,
            P1_0R::ENET_TXD0 => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_0R {
        match value {
            0 => P1_0R::GPIO_P1,
            1 => P1_0R::ENET_TXD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_0R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TXD0`"]
    #[inline]
    pub fn is_enet_txd0(&self) -> bool {
        *self == P1_0R::ENET_TXD0
    }
}
#[doc = "Possible values of the field `P1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_1R {
    #[doc = "GPIO P1.1"]
    GPIO_P1,
    #[doc = "ENET_TXD1"]
    ENET_TXD1,
}
impl P1_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_1R::GPIO_P1 => 0,
            P1_1R::ENET_TXD1 => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_1R {
        match value {
            0 => P1_1R::GPIO_P1,
            1 => P1_1R::ENET_TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_1R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TXD1`"]
    #[inline]
    pub fn is_enet_txd1(&self) -> bool {
        *self == P1_1R::ENET_TXD1
    }
}
#[doc = "Possible values of the field `P1_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_4R {
    #[doc = "GPIO P1.4."]
    GPIO_P1,
    #[doc = "ENET_TX_EN"]
    ENET_TX_EN,
}
impl P1_4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_4R::GPIO_P1 => 0,
            P1_4R::ENET_TX_EN => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_4R {
        match value {
            0 => P1_4R::GPIO_P1,
            1 => P1_4R::ENET_TX_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_4R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_TX_EN`"]
    #[inline]
    pub fn is_enet_tx_en(&self) -> bool {
        *self == P1_4R::ENET_TX_EN
    }
}
#[doc = "Possible values of the field `P1_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_8R {
    #[doc = "GPIO P1.8."]
    GPIO_P1,
    #[doc = "ENET_CRS"]
    ENET_CRS,
}
impl P1_8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_8R::GPIO_P1 => 0,
            P1_8R::ENET_CRS => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_8R {
        match value {
            0 => P1_8R::GPIO_P1,
            1 => P1_8R::ENET_CRS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_8R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_CRS`"]
    #[inline]
    pub fn is_enet_crs(&self) -> bool {
        *self == P1_8R::ENET_CRS
    }
}
#[doc = "Possible values of the field `P1_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_9R {
    #[doc = "GPIO Port 1.9"]
    GPIO_PORT_1,
    #[doc = "ENET_RXD0"]
    ENET_RXD0,
}
impl P1_9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_9R::GPIO_PORT_1 => 0,
            P1_9R::ENET_RXD0 => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_9R {
        match value {
            0 => P1_9R::GPIO_PORT_1,
            1 => P1_9R::ENET_RXD0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == P1_9R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `ENET_RXD0`"]
    #[inline]
    pub fn is_enet_rxd0(&self) -> bool {
        *self == P1_9R::ENET_RXD0
    }
}
#[doc = "Possible values of the field `P1_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10R {
    #[doc = "GPIO P1.10"]
    GPIO_P1,
    #[doc = "ENET_RXD1"]
    ENET_RXD1,
}
impl P1_10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_10R::GPIO_P1 => 0,
            P1_10R::ENET_RXD1 => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_10R {
        match value {
            0 => P1_10R::GPIO_P1,
            1 => P1_10R::ENET_RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_10R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_RXD1`"]
    #[inline]
    pub fn is_enet_rxd1(&self) -> bool {
        *self == P1_10R::ENET_RXD1
    }
}
#[doc = "Possible values of the field `P1_14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14R {
    #[doc = "GPIO P1.14"]
    GPIO_P1,
    #[doc = "ENET_RX_ER"]
    ENET_RX_ER,
}
impl P1_14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_14R::GPIO_P1 => 0,
            P1_14R::ENET_RX_ER => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_14R {
        match value {
            0 => P1_14R::GPIO_P1,
            1 => P1_14R::ENET_RX_ER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_14R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_RX_ER`"]
    #[inline]
    pub fn is_enet_rx_er(&self) -> bool {
        *self == P1_14R::ENET_RX_ER
    }
}
#[doc = "Possible values of the field `P1_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15R {
    #[doc = "GPIO P1.15"]
    GPIO_P1,
    #[doc = "ENET_REF_CLK"]
    ENET_REF_CLK,
}
impl P1_15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_15R::GPIO_P1 => 0,
            P1_15R::ENET_REF_CLK => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_15R {
        match value {
            0 => P1_15R::GPIO_P1,
            1 => P1_15R::ENET_REF_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P1`"]
    #[inline]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_15R::GPIO_P1
    }
    #[doc = "Checks if the value of the field is `ENET_REF_CLK`"]
    #[inline]
    pub fn is_enet_ref_clk(&self) -> bool {
        *self == P1_15R::ENET_REF_CLK
    }
}
#[doc = "Values that can be written to the field `P1_0`"]
pub enum P1_0W {
    #[doc = "GPIO P1.0"]
    GPIO_P1,
    #[doc = "ENET_TXD0"]
    ENET_TXD0,
}
impl P1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_0W::GPIO_P1 => 0,
            P1_0W::ENET_TXD0 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.0"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_0W::GPIO_P1)
    }
    #[doc = "ENET_TXD0"]
    #[inline]
    pub fn enet_txd0(self) -> &'a mut W {
        self.variant(P1_0W::ENET_TXD0)
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
#[doc = "Values that can be written to the field `P1_1`"]
pub enum P1_1W {
    #[doc = "GPIO P1.1"]
    GPIO_P1,
    #[doc = "ENET_TXD1"]
    ENET_TXD1,
}
impl P1_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_1W::GPIO_P1 => 0,
            P1_1W::ENET_TXD1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_1W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.1"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_1W::GPIO_P1)
    }
    #[doc = "ENET_TXD1"]
    #[inline]
    pub fn enet_txd1(self) -> &'a mut W {
        self.variant(P1_1W::ENET_TXD1)
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
#[doc = "Values that can be written to the field `P1_4`"]
pub enum P1_4W {
    #[doc = "GPIO P1.4."]
    GPIO_P1,
    #[doc = "ENET_TX_EN"]
    ENET_TX_EN,
}
impl P1_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_4W::GPIO_P1 => 0,
            P1_4W::ENET_TX_EN => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_4W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.4."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_4W::GPIO_P1)
    }
    #[doc = "ENET_TX_EN"]
    #[inline]
    pub fn enet_tx_en(self) -> &'a mut W {
        self.variant(P1_4W::ENET_TX_EN)
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
#[doc = "Values that can be written to the field `P1_8`"]
pub enum P1_8W {
    #[doc = "GPIO P1.8."]
    GPIO_P1,
    #[doc = "ENET_CRS"]
    ENET_CRS,
}
impl P1_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_8W::GPIO_P1 => 0,
            P1_8W::ENET_CRS => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_8W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.8."]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_8W::GPIO_P1)
    }
    #[doc = "ENET_CRS"]
    #[inline]
    pub fn enet_crs(self) -> &'a mut W {
        self.variant(P1_8W::ENET_CRS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_9`"]
pub enum P1_9W {
    #[doc = "GPIO Port 1.9"]
    GPIO_PORT_1,
    #[doc = "ENET_RXD0"]
    ENET_RXD0,
}
impl P1_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_9W::GPIO_PORT_1 => 0,
            P1_9W::ENET_RXD0 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_9W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO Port 1.9"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(P1_9W::GPIO_PORT_1)
    }
    #[doc = "ENET_RXD0"]
    #[inline]
    pub fn enet_rxd0(self) -> &'a mut W {
        self.variant(P1_9W::ENET_RXD0)
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
#[doc = "Values that can be written to the field `P1_10`"]
pub enum P1_10W {
    #[doc = "GPIO P1.10"]
    GPIO_P1,
    #[doc = "ENET_RXD1"]
    ENET_RXD1,
}
impl P1_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_10W::GPIO_P1 => 0,
            P1_10W::ENET_RXD1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_10W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.10"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_10W::GPIO_P1)
    }
    #[doc = "ENET_RXD1"]
    #[inline]
    pub fn enet_rxd1(self) -> &'a mut W {
        self.variant(P1_10W::ENET_RXD1)
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
#[doc = "Values that can be written to the field `P1_14`"]
pub enum P1_14W {
    #[doc = "GPIO P1.14"]
    GPIO_P1,
    #[doc = "ENET_RX_ER"]
    ENET_RX_ER,
}
impl P1_14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_14W::GPIO_P1 => 0,
            P1_14W::ENET_RX_ER => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_14W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.14"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_14W::GPIO_P1)
    }
    #[doc = "ENET_RX_ER"]
    #[inline]
    pub fn enet_rx_er(self) -> &'a mut W {
        self.variant(P1_14W::ENET_RX_ER)
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
#[doc = "Values that can be written to the field `P1_15`"]
pub enum P1_15W {
    #[doc = "GPIO P1.15"]
    GPIO_P1,
    #[doc = "ENET_REF_CLK"]
    ENET_REF_CLK,
}
impl P1_15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_15W::GPIO_P1 => 0,
            P1_15W::ENET_REF_CLK => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_15W<'a> {
    w: &'a mut W,
}
impl<'a> _P1_15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO P1.15"]
    #[inline]
    pub fn gpio_p1(self) -> &'a mut W {
        self.variant(P1_15W::GPIO_P1)
    }
    #[doc = "ENET_REF_CLK"]
    #[inline]
    pub fn enet_ref_clk(self) -> &'a mut W {
        self.variant(P1_15W::ENET_REF_CLK)
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
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline]
    pub fn p1_0(&self) -> P1_0R {
        P1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline]
    pub fn p1_1(&self) -> P1_1R {
        P1_1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline]
    pub fn p1_4(&self) -> P1_4R {
        P1_4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline]
    pub fn p1_8(&self) -> P1_8R {
        P1_8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline]
    pub fn p1_9(&self) -> P1_9R {
        P1_9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline]
    pub fn p1_10(&self) -> P1_10R {
        P1_10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline]
    pub fn p1_14(&self) -> P1_14R {
        P1_14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline]
    pub fn p1_15(&self) -> P1_15R {
        P1_15R::_from({
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
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline]
    pub fn p1_0(&mut self) -> _P1_0W {
        _P1_0W { w: self }
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline]
    pub fn p1_1(&mut self) -> _P1_1W {
        _P1_1W { w: self }
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline]
    pub fn p1_4(&mut self) -> _P1_4W {
        _P1_4W { w: self }
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline]
    pub fn p1_8(&mut self) -> _P1_8W {
        _P1_8W { w: self }
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline]
    pub fn p1_9(&mut self) -> _P1_9W {
        _P1_9W { w: self }
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline]
    pub fn p1_10(&mut self) -> _P1_10W {
        _P1_10W { w: self }
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline]
    pub fn p1_14(&mut self) -> _P1_14W {
        _P1_14W { w: self }
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline]
    pub fn p1_15(&mut self) -> _P1_15W {
        _P1_15W { w: self }
    }
}
