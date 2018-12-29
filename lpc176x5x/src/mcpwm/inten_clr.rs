#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN_CLR {
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
pub struct _ILIM0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ILIM0_CLRW<'a> {
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
pub struct _IMAT0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMAT0_CLRW<'a> {
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
pub struct _ICAP0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ICAP0_CLRW<'a> {
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
pub struct _ILIM1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ILIM1_CLRW<'a> {
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
pub struct _IMAT1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMAT1_CLRW<'a> {
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
pub struct _ICAP1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ICAP1_CLRW<'a> {
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
pub struct _ILIM2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ILIM2_CLRW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IMAT2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMAT2_CLRW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ICAP2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ICAP2_CLRW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ABORT_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_CLRW<'a> {
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
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn ilim0_clr(&mut self) -> _ILIM0_CLRW {
        _ILIM0_CLRW { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn imat0_clr(&mut self) -> _IMAT0_CLRW {
        _IMAT0_CLRW { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn icap0_clr(&mut self) -> _ICAP0_CLRW {
        _ICAP0_CLRW { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn ilim1_clr(&mut self) -> _ILIM1_CLRW {
        _ILIM1_CLRW { w: self }
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn imat1_clr(&mut self) -> _IMAT1_CLRW {
        _IMAT1_CLRW { w: self }
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn icap1_clr(&mut self) -> _ICAP1_CLRW {
        _ICAP1_CLRW { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn ilim2_clr(&mut self) -> _ILIM2_CLRW {
        _ILIM2_CLRW { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn imat2_clr(&mut self) -> _IMAT2_CLRW {
        _IMAT2_CLRW { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn icap2_clr(&mut self) -> _ICAP2_CLRW {
        _ICAP2_CLRW { w: self }
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline]
    pub fn abort_clr(&mut self) -> _ABORT_CLRW {
        _ABORT_CLRW { w: self }
    }
}
