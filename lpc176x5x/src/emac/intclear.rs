#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTCLEAR {
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
#[doc = r" Proxy"]
pub struct _RXOVERRUNINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVERRUNINTCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _RXERRORINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRORINTCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _RXFINISHEDINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFINISHEDINTCLRW<'a> {
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
pub struct _RXDONEINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDONEINTCLRW<'a> {
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
pub struct _TXUNDERRUNINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUNDERRUNINTCLRW<'a> {
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
pub struct _TXERRORINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRORINTCLRW<'a> {
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
pub struct _TXFINISHEDINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFINISHEDINTCLRW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXDONEINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDONEINTCLRW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOFTINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTINTCLRW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPINTCLRW<'a> {
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxoverrunintclr(&mut self) -> _RXOVERRUNINTCLRW {
        _RXOVERRUNINTCLRW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxerrorintclr(&mut self) -> _RXERRORINTCLRW {
        _RXERRORINTCLRW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxfinishedintclr(&mut self) -> _RXFINISHEDINTCLRW {
        _RXFINISHEDINTCLRW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxdoneintclr(&mut self) -> _RXDONEINTCLRW {
        _RXDONEINTCLRW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txunderrunintclr(&mut self) -> _TXUNDERRUNINTCLRW {
        _TXUNDERRUNINTCLRW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txerrorintclr(&mut self) -> _TXERRORINTCLRW {
        _TXERRORINTCLRW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txfinishedintclr(&mut self) -> _TXFINISHEDINTCLRW {
        _TXFINISHEDINTCLRW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txdoneintclr(&mut self) -> _TXDONEINTCLRW {
        _TXDONEINTCLRW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn softintclr(&mut self) -> _SOFTINTCLRW {
        _SOFTINTCLRW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn wakeupintclr(&mut self) -> _WAKEUPINTCLRW {
        _WAKEUPINTCLRW { w: self }
    }
}
