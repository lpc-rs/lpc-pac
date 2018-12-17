#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINSEL9 {
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
#[doc = "Possible values of the field `P4_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28R {
    #[doc = "GPIO P4.28"]
    GPIO_P4,
    #[doc = "RX_MCLK"]
    RX_MCLK,
    #[doc = "MAT2.0"]
    MAT2,
    #[doc = "TXD3"]
    TXD3,
}
impl P4_28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P4_28R::GPIO_P4 => 0,
            P4_28R::RX_MCLK => 1,
            P4_28R::MAT2 => 2,
            P4_28R::TXD3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P4_28R {
        match value {
            0 => P4_28R::GPIO_P4,
            1 => P4_28R::RX_MCLK,
            2 => P4_28R::MAT2,
            3 => P4_28R::TXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P4`"]
    #[inline]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_28R::GPIO_P4
    }
    #[doc = "Checks if the value of the field is `RX_MCLK`"]
    #[inline]
    pub fn is_rx_mclk(&self) -> bool {
        *self == P4_28R::RX_MCLK
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline]
    pub fn is_mat2(&self) -> bool {
        *self == P4_28R::MAT2
    }
    #[doc = "Checks if the value of the field is `TXD3`"]
    #[inline]
    pub fn is_txd3(&self) -> bool {
        *self == P4_28R::TXD3
    }
}
#[doc = "Possible values of the field `P4_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_29R {
    #[doc = "GPIO P4.29"]
    GPIO_P4,
    #[doc = "TX_MCLK"]
    TX_MCLK,
    #[doc = "MAT2.1"]
    MAT2,
    #[doc = "RXD3"]
    RXD3,
}
impl P4_29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P4_29R::GPIO_P4 => 0,
            P4_29R::TX_MCLK => 1,
            P4_29R::MAT2 => 2,
            P4_29R::RXD3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P4_29R {
        match value {
            0 => P4_29R::GPIO_P4,
            1 => P4_29R::TX_MCLK,
            2 => P4_29R::MAT2,
            3 => P4_29R::RXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P4`"]
    #[inline]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_29R::GPIO_P4
    }
    #[doc = "Checks if the value of the field is `TX_MCLK`"]
    #[inline]
    pub fn is_tx_mclk(&self) -> bool {
        *self == P4_29R::TX_MCLK
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline]
    pub fn is_mat2(&self) -> bool {
        *self == P4_29R::MAT2
    }
    #[doc = "Checks if the value of the field is `RXD3`"]
    #[inline]
    pub fn is_rxd3(&self) -> bool {
        *self == P4_29R::RXD3
    }
}
#[doc = "Values that can be written to the field `P4_28`"]
pub enum P4_28W {
    #[doc = "GPIO P4.28"]
    GPIO_P4,
    #[doc = "RX_MCLK"]
    RX_MCLK,
    #[doc = "MAT2.0"]
    MAT2,
    #[doc = "TXD3"]
    TXD3,
}
impl P4_28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_28W::GPIO_P4 => 0,
            P4_28W::RX_MCLK => 1,
            P4_28W::MAT2 => 2,
            P4_28W::TXD3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P4_28W<'a> {
    w: &'a mut W,
}
impl<'a> _P4_28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P4_28W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P4.28"]
    #[inline]
    pub fn gpio_p4(self) -> &'a mut W {
        self.variant(P4_28W::GPIO_P4)
    }
    #[doc = "RX_MCLK"]
    #[inline]
    pub fn rx_mclk(self) -> &'a mut W {
        self.variant(P4_28W::RX_MCLK)
    }
    #[doc = "MAT2.0"]
    #[inline]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P4_28W::MAT2)
    }
    #[doc = "TXD3"]
    #[inline]
    pub fn txd3(self) -> &'a mut W {
        self.variant(P4_28W::TXD3)
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
#[doc = "Values that can be written to the field `P4_29`"]
pub enum P4_29W {
    #[doc = "GPIO P4.29"]
    GPIO_P4,
    #[doc = "TX_MCLK"]
    TX_MCLK,
    #[doc = "MAT2.1"]
    MAT2,
    #[doc = "RXD3"]
    RXD3,
}
impl P4_29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_29W::GPIO_P4 => 0,
            P4_29W::TX_MCLK => 1,
            P4_29W::MAT2 => 2,
            P4_29W::RXD3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P4_29W<'a> {
    w: &'a mut W,
}
impl<'a> _P4_29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P4_29W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO P4.29"]
    #[inline]
    pub fn gpio_p4(self) -> &'a mut W {
        self.variant(P4_29W::GPIO_P4)
    }
    #[doc = "TX_MCLK"]
    #[inline]
    pub fn tx_mclk(self) -> &'a mut W {
        self.variant(P4_29W::TX_MCLK)
    }
    #[doc = "MAT2.1"]
    #[inline]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P4_29W::MAT2)
    }
    #[doc = "RXD3"]
    #[inline]
    pub fn rxd3(self) -> &'a mut W {
        self.variant(P4_29W::RXD3)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline]
    pub fn p4_28(&self) -> P4_28R {
        P4_28R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline]
    pub fn p4_29(&self) -> P4_29R {
        P4_29R::_from({
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
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline]
    pub fn p4_28(&mut self) -> _P4_28W {
        _P4_28W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline]
    pub fn p4_29(&mut self) -> _P4_29W {
        _P4_29W { w: self }
    }
}
