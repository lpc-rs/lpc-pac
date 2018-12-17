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
    #[doc = "USART FIFOs are disabled. Must not be used in the application."]
    DISABLED,
    #[doc = "Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    ENABLED,
}
impl FIFOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFOENW::DISABLED => false,
            FIFOENW::ENABLED => true,
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
    #[doc = "USART FIFOs are disabled. Must not be used in the application."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOENW::DISABLED)
    }
    #[doc = "Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOENW::ENABLED)
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
    #[doc = "No impact on either of USART FIFOs."]
    NO_IMPACT,
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR,
}
impl RXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFORESW::NO_IMPACT => false,
            RXFIFORESW::CLEAR => true,
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
    #[doc = "No impact on either of USART FIFOs."]
    #[inline]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(RXFIFORESW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFORESW::CLEAR)
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
    #[doc = "No impact on either of USART FIFOs."]
    NO_IMPACT,
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR,
}
impl TXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFIFORESW::NO_IMPACT => false,
            TXFIFORESW::CLEAR => true,
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
    #[doc = "No impact on either of USART FIFOs."]
    #[inline]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(TXFIFORESW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFORESW::CLEAR)
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
#[doc = "Values that can be written to the field `RXTL`"]
pub enum RXTLW {
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    TRIGGER_LEVEL_0_1_C,
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    TRIGGER_LEVEL_1_4_C,
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    TRIGGER_LEVEL_2_8_C,
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    TRIGGER_LEVEL_3_14_,
}
impl RXTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTLW::TRIGGER_LEVEL_0_1_C => 0,
            RXTLW::TRIGGER_LEVEL_1_4_C => 1,
            RXTLW::TRIGGER_LEVEL_2_8_C => 2,
            RXTLW::TRIGGER_LEVEL_3_14_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline]
    pub fn trigger_level_0_1_c(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_0_1_C)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline]
    pub fn trigger_level_1_4_c(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_1_4_C)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline]
    pub fn trigger_level_2_8_c(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_2_8_C)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline]
    pub fn trigger_level_3_14_(self) -> &'a mut W {
        self.variant(RXTLW::TRIGGER_LEVEL_3_14_)
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
    #[doc = "Bit 0 - FIFO enable"]
    #[inline]
    pub fn fifoen(&mut self) -> _FIFOENW {
        _FIFOENW { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset"]
    #[inline]
    pub fn rxfifores(&mut self) -> _RXFIFORESW {
        _RXFIFORESW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset"]
    #[inline]
    pub fn txfifores(&mut self) -> _TXFIFORESW {
        _TXFIFORESW { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated."]
    #[inline]
    pub fn rxtl(&mut self) -> _RXTLW {
        _RXTLW { w: self }
    }
}
