#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTSET {
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
pub struct _RXOVERRUNINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVERRUNINTSETW<'a> {
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
pub struct _RXERRORINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRORINTSETW<'a> {
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
pub struct _RXFINISHEDINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFINISHEDINTSETW<'a> {
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
pub struct _RXDONEINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDONEINTSETW<'a> {
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
pub struct _TXUNDERRUNINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUNDERRUNINTSETW<'a> {
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
pub struct _TXERRORINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRORINTSETW<'a> {
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
pub struct _TXFINISHEDINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFINISHEDINTSETW<'a> {
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
pub struct _TXDONEINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDONEINTSETW<'a> {
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
pub struct _SOFTINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTINTSETW<'a> {
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
pub struct _WAKEUPINTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPINTSETW<'a> {
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
    #[doc = "Bit 0 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxoverrunintset(&mut self) -> _RXOVERRUNINTSETW {
        _RXOVERRUNINTSETW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxerrorintset(&mut self) -> _RXERRORINTSETW {
        _RXERRORINTSETW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxfinishedintset(&mut self) -> _RXFINISHEDINTSETW {
        _RXFINISHEDINTSETW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn rxdoneintset(&mut self) -> _RXDONEINTSETW {
        _RXDONEINTSETW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txunderrunintset(&mut self) -> _TXUNDERRUNINTSETW {
        _TXUNDERRUNINTSETW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txerrorintset(&mut self) -> _TXERRORINTSETW {
        _TXERRORINTSETW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txfinishedintset(&mut self) -> _TXFINISHEDINTSETW {
        _TXFINISHEDINTSETW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn txdoneintset(&mut self) -> _TXDONEINTSETW {
        _TXDONEINTSETW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn softintset(&mut self) -> _SOFTINTSETW {
        _SOFTINTSETW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline]
    pub fn wakeupintset(&mut self) -> _WAKEUPINTSETW {
        _WAKEUPINTSETW { w: self }
    }
}
