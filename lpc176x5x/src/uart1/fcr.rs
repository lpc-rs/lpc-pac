#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCR {
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
}
#[doc = "Values that can be written to the field `FIFOEN`"]
pub enum FIFOENW {
    #[doc = "Must not be used in the application."]
    MUST_NOT_BE_USED_IN_,
    #[doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    ACTIVE_HIGH_ENABLE_F,
}
impl FIFOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFOENW::MUST_NOT_BE_USED_IN_ => false,
            FIFOENW::ACTIVE_HIGH_ENABLE_F => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFOENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Must not be used in the application."]
    #[inline]
    pub fn must_not_be_used_in_(self) -> &'a mut W {
        self.variant(FIFOENW::MUST_NOT_BE_USED_IN_)
    }
    #[doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    #[inline]
    pub fn active_high_enable_f(self) -> &'a mut W {
        self.variant(FIFOENW::ACTIVE_HIGH_ENABLE_F)
    }
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXFIFORES`"]
pub enum RXFIFORESW {
    #[doc = "No impact on either of UART1 FIFOs."]
    NO_IMPACT_ON_EITHER_,
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    WRITING_A_LOGIC_1_TO,
}
impl RXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFORESW::NO_IMPACT_ON_EITHER_ => false,
            RXFIFORESW::WRITING_A_LOGIC_1_TO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFORESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline]
    pub fn no_impact_on_either_(self) -> &'a mut W {
        self.variant(RXFIFORESW::NO_IMPACT_ON_EITHER_)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline]
    pub fn writing_a_logic_1_to(self) -> &'a mut W {
        self.variant(RXFIFORESW::WRITING_A_LOGIC_1_TO)
    }
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXFIFORES`"]
pub enum TXFIFORESW {
    #[doc = "No impact on either of UART1 FIFOs."]
    NO_IMPACT_ON_EITHER_,
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    WRITING_A_LOGIC_1_TO,
}
impl TXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFIFORESW::NO_IMPACT_ON_EITHER_ => false,
            TXFIFORESW::WRITING_A_LOGIC_1_TO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFIFORESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline]
    pub fn no_impact_on_either_(self) -> &'a mut W {
        self.variant(TXFIFORESW::NO_IMPACT_ON_EITHER_)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline]
    pub fn writing_a_logic_1_to(self) -> &'a mut W {
        self.variant(TXFIFORESW::WRITING_A_LOGIC_1_TO)
    }
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
pub struct _DMAMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMODEW<'a> {
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
#[doc = "Values that can be written to the field `RXTRIGLVL`"]
pub enum RXTRIGLVLW {
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    TRIGGER_LEVEL_0_1_C,
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    TRIGGER_LEVEL_1_4_C,
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    TRIGGER_LEVEL_2_8_C,
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    TRIGGER_LEVEL_3_14_,
}
impl RXTRIGLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTRIGLVLW::TRIGGER_LEVEL_0_1_C => 0,
            RXTRIGLVLW::TRIGGER_LEVEL_1_4_C => 1,
            RXTRIGLVLW::TRIGGER_LEVEL_2_8_C => 2,
            RXTRIGLVLW::TRIGGER_LEVEL_3_14_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTRIGLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTRIGLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTRIGLVLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline]
    pub fn trigger_level_0_1_c(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::TRIGGER_LEVEL_0_1_C)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline]
    pub fn trigger_level_1_4_c(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::TRIGGER_LEVEL_1_4_C)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline]
    pub fn trigger_level_2_8_c(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::TRIGGER_LEVEL_2_8_C)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline]
    pub fn trigger_level_3_14_(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::TRIGGER_LEVEL_3_14_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - FIFO enable."]
    #[inline]
    pub fn fifoen(&mut self) -> _FIFOENW {
        _FIFOENW { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline]
    pub fn rxfifores(&mut self) -> _RXFIFORESW {
        _RXFIFORESW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline]
    pub fn txfifores(&mut self) -> _TXFIFORESW {
        _TXFIFORESW { w: self }
    }
    #[doc = "Bit 3 - DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this bit selects the DMA mode. See Section 36.6.6.1."]
    #[inline]
    pub fn dmamode(&mut self) -> _DMAMODEW {
        _DMAMODEW { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated."]
    #[inline]
    pub fn rxtriglvl(&mut self) -> _RXTRIGLVLW {
        _RXTRIGLVLW { w: self }
    }
}
