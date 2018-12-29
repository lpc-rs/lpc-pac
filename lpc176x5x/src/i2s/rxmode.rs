#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXMODE {
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
#[doc = "Possible values of the field `RXCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCLKSELR {
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    SELECT_THE_RX_FRACTI,
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    SELECT_THE_TX_MCLK_S,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXCLKSELR::SELECT_THE_RX_FRACTI => 0,
            RXCLKSELR::SELECT_THE_TX_MCLK_S => 2,
            RXCLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXCLKSELR {
        match value {
            0 => RXCLKSELR::SELECT_THE_RX_FRACTI,
            2 => RXCLKSELR::SELECT_THE_TX_MCLK_S,
            i => RXCLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_RX_FRACTI`"]
    #[inline]
    pub fn is_select_the_rx_fracti(&self) -> bool {
        *self == RXCLKSELR::SELECT_THE_RX_FRACTI
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_TX_MCLK_S`"]
    #[inline]
    pub fn is_select_the_tx_mclk_s(&self) -> bool {
        *self == RXCLKSELR::SELECT_THE_TX_MCLK_S
    }
}
#[doc = r" Value of the field"]
pub struct RX4PINR {
    bits: bool,
}
impl RX4PINR {
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
pub struct RXMCENAR {
    bits: bool,
}
impl RXMCENAR {
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
#[doc = "Values that can be written to the field `RXCLKSEL`"]
pub enum RXCLKSELW {
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    SELECT_THE_RX_FRACTI,
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    SELECT_THE_TX_MCLK_S,
}
impl RXCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXCLKSELW::SELECT_THE_RX_FRACTI => 0,
            RXCLKSELW::SELECT_THE_TX_MCLK_S => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline]
    pub fn select_the_rx_fracti(self) -> &'a mut W {
        self.variant(RXCLKSELW::SELECT_THE_RX_FRACTI)
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline]
    pub fn select_the_tx_mclk_s(self) -> &'a mut W {
        self.variant(RXCLKSELW::SELECT_THE_TX_MCLK_S)
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
pub struct _RX4PINW<'a> {
    w: &'a mut W,
}
impl<'a> _RX4PINW<'a> {
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
pub struct _RXMCENAW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMCENAW<'a> {
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
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline]
    pub fn rxclksel(&self) -> RXCLKSELR {
        RXCLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline]
    pub fn rx4pin(&self) -> RX4PINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX4PINR { bits }
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline]
    pub fn rxmcena(&self) -> RXMCENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXMCENAR { bits }
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
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline]
    pub fn rxclksel(&mut self) -> _RXCLKSELW {
        _RXCLKSELW { w: self }
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline]
    pub fn rx4pin(&mut self) -> _RX4PINW {
        _RX4PINW { w: self }
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline]
    pub fn rxmcena(&mut self) -> _RXMCENAW {
        _RXMCENAW { w: self }
    }
}
