#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CON_SET {
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
pub struct _RUN0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN0_SETW<'a> {
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
pub struct _CENTER0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER0_SETW<'a> {
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
pub struct _POLA0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA0_SETW<'a> {
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
pub struct _DTE0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE0_SETW<'a> {
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
pub struct _DISUP0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP0_SETW<'a> {
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
pub struct _RUN1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN1_SETW<'a> {
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
pub struct _CENTER1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER1_SETW<'a> {
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
pub struct _POLA1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA1_SETW<'a> {
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
pub struct _DTE1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE1_SETW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISUP1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP1_SETW<'a> {
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
pub struct _RUN2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN2_SETW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CENTER2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER2_SETW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLA2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA2_SETW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTE2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE2_SETW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISUP2_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP2_SETW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVBDC_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _INVBDC_SETW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMODE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMODE_SETW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCMODE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMODE_SETW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn run0_set(&mut self) -> _RUN0_SETW {
        _RUN0_SETW { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn center0_set(&mut self) -> _CENTER0_SETW {
        _CENTER0_SETW { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn pola0_set(&mut self) -> _POLA0_SETW {
        _POLA0_SETW { w: self }
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn dte0_set(&mut self) -> _DTE0_SETW {
        _DTE0_SETW { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn disup0_set(&mut self) -> _DISUP0_SETW {
        _DISUP0_SETW { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn run1_set(&mut self) -> _RUN1_SETW {
        _RUN1_SETW { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn center1_set(&mut self) -> _CENTER1_SETW {
        _CENTER1_SETW { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn pola1_set(&mut self) -> _POLA1_SETW {
        _POLA1_SETW { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn dte1_set(&mut self) -> _DTE1_SETW {
        _DTE1_SETW { w: self }
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn disup1_set(&mut self) -> _DISUP1_SETW {
        _DISUP1_SETW { w: self }
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn run2_set(&mut self) -> _RUN2_SETW {
        _RUN2_SETW { w: self }
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn center2_set(&mut self) -> _CENTER2_SETW {
        _CENTER2_SETW { w: self }
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn pola2_set(&mut self) -> _POLA2_SETW {
        _POLA2_SETW { w: self }
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn dte2_set(&mut self) -> _DTE2_SETW {
        _DTE2_SETW { w: self }
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn disup2_set(&mut self) -> _DISUP2_SETW {
        _DISUP2_SETW { w: self }
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn invbdc_set(&mut self) -> _INVBDC_SETW {
        _INVBDC_SETW { w: self }
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn acmode_set(&mut self) -> _ACMODE_SETW {
        _ACMODE_SETW { w: self }
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CON register."]
    #[inline]
    pub fn dcmode_set(&mut self) -> _DCMODE_SETW {
        _DCMODE_SETW { w: self }
    }
}
