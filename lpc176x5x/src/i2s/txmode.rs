#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXMODE {
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
#[doc = "Possible values of the field `TXCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCLKSELR {
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    SELECT_THE_TX_FRACTI,
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    SELECT_THE_RX_MCLK_S,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXCLKSELR::SELECT_THE_TX_FRACTI => 0,
            TXCLKSELR::SELECT_THE_RX_MCLK_S => 2,
            TXCLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXCLKSELR {
        match value {
            0 => TXCLKSELR::SELECT_THE_TX_FRACTI,
            2 => TXCLKSELR::SELECT_THE_RX_MCLK_S,
            i => TXCLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_TX_FRACTI`"]
    #[inline]
    pub fn is_select_the_tx_fracti(&self) -> bool {
        *self == TXCLKSELR::SELECT_THE_TX_FRACTI
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_RX_MCLK_S`"]
    #[inline]
    pub fn is_select_the_rx_mclk_s(&self) -> bool {
        *self == TXCLKSELR::SELECT_THE_RX_MCLK_S
    }
}
#[doc = r" Value of the field"]
pub struct TX4PINR {
    bits: bool,
}
impl TX4PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TXMCENAR {
    bits: bool,
}
impl TXMCENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `TXCLKSEL`"]
pub enum TXCLKSELW {
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    SELECT_THE_TX_FRACTI,
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    SELECT_THE_RX_MCLK_S,
}
impl TXCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXCLKSELW::SELECT_THE_TX_FRACTI => 0,
            TXCLKSELW::SELECT_THE_RX_MCLK_S => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    #[inline]
    pub fn select_the_tx_fracti(self) -> &'a mut W {
        self.variant(TXCLKSELW::SELECT_THE_TX_FRACTI)
    }
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    #[inline]
    pub fn select_the_rx_mclk_s(self) -> &'a mut W {
        self.variant(TXCLKSELW::SELECT_THE_RX_MCLK_S)
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
#[doc = r" Proxy"]
pub struct _TX4PINW<'a> {
    w: &'a mut W,
}
impl<'a> _TX4PINW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXMCENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMCENAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
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
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline]
    pub fn txclksel(&self) -> TXCLKSELR {
        TXCLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline]
    pub fn tx4pin(&self) -> TX4PINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX4PINR { bits }
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline]
    pub fn txmcena(&self) -> TXMCENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXMCENAR { bits }
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
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline]
    pub fn txclksel(&mut self) -> _TXCLKSELW {
        _TXCLKSELW { w: self }
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline]
    pub fn tx4pin(&mut self) -> _TX4PINW {
        _TX4PINW { w: self }
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline]
    pub fn txmcena(&mut self) -> _TXMCENAW {
        _TXMCENAW { w: self }
    }
}
