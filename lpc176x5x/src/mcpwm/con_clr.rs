#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CON_CLR {
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
pub struct _RUN0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN0_CLRW<'a> {
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
pub struct _CENTER0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER0_CLRW<'a> {
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
pub struct _POLA0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA0_CLRW<'a> {
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
pub struct _DTE0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE0_CLRW<'a> {
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
pub struct _DISUP0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP0_CLRW<'a> {
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
pub struct _RUN1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN1_CLRW<'a> {
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
pub struct _CENTER1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER1_CLRW<'a> {
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
pub struct _POLA1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA1_CLRW<'a> {
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
pub struct _DTE1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE1_CLRW<'a> {
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
pub struct _DISUP1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP1_CLRW<'a> {
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
pub struct _RUN2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RUN2_CLRW<'a> {
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
pub struct _CENTER2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CENTER2_CLRW<'a> {
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
pub struct _POLA2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _POLA2_CLRW<'a> {
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
pub struct _DTE2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DTE2_CLRW<'a> {
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
pub struct _DISUP2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISUP2_CLRW<'a> {
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
pub struct _INVBDC_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _INVBDC_CLRW<'a> {
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
pub struct _ACMOD_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMOD_CLRW<'a> {
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
pub struct _DCMODE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMODE_CLRW<'a> {
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
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn run0_clr(&mut self) -> _RUN0_CLRW {
        _RUN0_CLRW { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn center0_clr(&mut self) -> _CENTER0_CLRW {
        _CENTER0_CLRW { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn pola0_clr(&mut self) -> _POLA0_CLRW {
        _POLA0_CLRW { w: self }
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn dte0_clr(&mut self) -> _DTE0_CLRW {
        _DTE0_CLRW { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn disup0_clr(&mut self) -> _DISUP0_CLRW {
        _DISUP0_CLRW { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn run1_clr(&mut self) -> _RUN1_CLRW {
        _RUN1_CLRW { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn center1_clr(&mut self) -> _CENTER1_CLRW {
        _CENTER1_CLRW { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn pola1_clr(&mut self) -> _POLA1_CLRW {
        _POLA1_CLRW { w: self }
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn dte1_clr(&mut self) -> _DTE1_CLRW {
        _DTE1_CLRW { w: self }
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn disup1_clr(&mut self) -> _DISUP1_CLRW {
        _DISUP1_CLRW { w: self }
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn run2_clr(&mut self) -> _RUN2_CLRW {
        _RUN2_CLRW { w: self }
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn center2_clr(&mut self) -> _CENTER2_CLRW {
        _CENTER2_CLRW { w: self }
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn pola2_clr(&mut self) -> _POLA2_CLRW {
        _POLA2_CLRW { w: self }
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn dte2_clr(&mut self) -> _DTE2_CLRW {
        _DTE2_CLRW { w: self }
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn disup2_clr(&mut self) -> _DISUP2_CLRW {
        _DISUP2_CLRW { w: self }
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn invbdc_clr(&mut self) -> _INVBDC_CLRW {
        _INVBDC_CLRW { w: self }
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn acmod_clr(&mut self) -> _ACMOD_CLRW {
        _ACMOD_CLRW { w: self }
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CON register."]
    #[inline]
    pub fn dcmode_clr(&mut self) -> _DCMODE_CLRW {
        _DCMODE_CLRW { w: self }
    }
}
