#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DAO {
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
#[doc = "Possible values of the field `WORDWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDWIDTHR {
    #[doc = "8-bit data"]
    _8_BIT_DATA,
    #[doc = "16-bit data"]
    _16_BIT_DATA,
    #[doc = "32-bit data"]
    _32_BIT_DATA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WORDWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WORDWIDTHR::_8_BIT_DATA => 0,
            WORDWIDTHR::_16_BIT_DATA => 1,
            WORDWIDTHR::_32_BIT_DATA => 3,
            WORDWIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WORDWIDTHR {
        match value {
            0 => WORDWIDTHR::_8_BIT_DATA,
            1 => WORDWIDTHR::_16_BIT_DATA,
            3 => WORDWIDTHR::_32_BIT_DATA,
            i => WORDWIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT_DATA`"]
    #[inline]
    pub fn is_8_bit_data(&self) -> bool {
        *self == WORDWIDTHR::_8_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_16_BIT_DATA`"]
    #[inline]
    pub fn is_16_bit_data(&self) -> bool {
        *self == WORDWIDTHR::_16_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_32_BIT_DATA`"]
    #[inline]
    pub fn is_32_bit_data(&self) -> bool {
        *self == WORDWIDTHR::_32_BIT_DATA
    }
}
#[doc = r" Value of the field"]
pub struct MONOR {
    bits: bool,
}
impl MONOR {
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
pub struct STOPR {
    bits: bool,
}
impl STOPR {
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
pub struct RESETR {
    bits: bool,
}
impl RESETR {
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
pub struct WS_SELR {
    bits: bool,
}
impl WS_SELR {
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
pub struct WS_HALFPERIODR {
    bits: u16,
}
impl WS_HALFPERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUTER {
    bits: bool,
}
impl MUTER {
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
#[doc = "Values that can be written to the field `WORDWIDTH`"]
pub enum WORDWIDTHW {
    #[doc = "8-bit data"]
    _8_BIT_DATA,
    #[doc = "16-bit data"]
    _16_BIT_DATA,
    #[doc = "32-bit data"]
    _32_BIT_DATA,
}
impl WORDWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WORDWIDTHW::_8_BIT_DATA => 0,
            WORDWIDTHW::_16_BIT_DATA => 1,
            WORDWIDTHW::_32_BIT_DATA => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WORDWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WORDWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WORDWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit data"]
    #[inline]
    pub fn _8_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTHW::_8_BIT_DATA)
    }
    #[doc = "16-bit data"]
    #[inline]
    pub fn _16_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTHW::_16_BIT_DATA)
    }
    #[doc = "32-bit data"]
    #[inline]
    pub fn _32_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTHW::_32_BIT_DATA)
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
pub struct _MONOW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOW<'a> {
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
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
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
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WS_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _WS_SELW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WS_HALFPERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _WS_HALFPERIODW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MUTEW<'a> {
    w: &'a mut W,
}
impl<'a> _MUTEW<'a> {
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline]
    pub fn wordwidth(&self) -> WORDWIDTHR {
        WORDWIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline]
    pub fn mono(&self) -> MONOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MONOR { bits }
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline]
    pub fn stop(&self) -> STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOPR { bits }
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline]
    pub fn reset(&self) -> RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESETR { bits }
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline]
    pub fn ws_sel(&self) -> WS_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WS_SELR { bits }
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline]
    pub fn ws_halfperiod(&self) -> WS_HALFPERIODR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WS_HALFPERIODR { bits }
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline]
    pub fn mute(&self) -> MUTER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MUTER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 34785 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline]
    pub fn wordwidth(&mut self) -> _WORDWIDTHW {
        _WORDWIDTHW { w: self }
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline]
    pub fn mono(&mut self) -> _MONOW {
        _MONOW { w: self }
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline]
    pub fn ws_sel(&mut self) -> _WS_SELW {
        _WS_SELW { w: self }
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline]
    pub fn ws_halfperiod(&mut self) -> _WS_HALFPERIODW {
        _WS_HALFPERIODW { w: self }
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline]
    pub fn mute(&mut self) -> _MUTEW {
        _MUTEW { w: self }
    }
}
