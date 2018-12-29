#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CAPCON_CLR {
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
pub struct _CAP0MCI0_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0MCI0_RE_CLRW<'a> {
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
pub struct _CAP0MCI0_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0MCI0_FE_CLRW<'a> {
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
pub struct _CAP0MCI1_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0MCI1_RE_CLRW<'a> {
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
pub struct _CAP0MCI1_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0MCI1_FE_CLRW<'a> {
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
pub struct _CAP0MCI2_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0MCI2_RE_CLRW<'a> {
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
pub struct _CAP0MCI2_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0MCI2_FE_CLRW<'a> {
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
pub struct _CAP1MCI0_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1MCI0_RE_CLRW<'a> {
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
pub struct _CAP1MCI0_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1MCI0_FE_CLRW<'a> {
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
pub struct _CAP1MCI1_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1MCI1_RE_CLRW<'a> {
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
pub struct _CAP1MCI1_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1MCI1_FE_CLRW<'a> {
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
pub struct _CAP1MCI2_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1MCI2_RE_CLRW<'a> {
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
pub struct _CAP1MCI2_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1MCI2_FE_CLRW<'a> {
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
pub struct _CAP2MCI0_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2MCI0_RE_CLRW<'a> {
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
pub struct _CAP2MCI0_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2MCI0_FE_CLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _CAP2MCI1_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2MCI1_RE_CLRW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAP2MCI1_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2MCI1_FE_CLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _CAP2MCI2_RE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2MCI2_RE_CLRW<'a> {
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
pub struct _CAP2MCI2_FE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2MCI2_FE_CLRW<'a> {
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
pub struct _RT0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RT0_CLRW<'a> {
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
pub struct _RT1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RT1_CLRW<'a> {
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
pub struct _RT2_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RT2_CLRW<'a> {
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
    #[doc = "Bit 0 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap0mci0_re_clr(&mut self) -> _CAP0MCI0_RE_CLRW {
        _CAP0MCI0_RE_CLRW { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap0mci0_fe_clr(&mut self) -> _CAP0MCI0_FE_CLRW {
        _CAP0MCI0_FE_CLRW { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap0mci1_re_clr(&mut self) -> _CAP0MCI1_RE_CLRW {
        _CAP0MCI1_RE_CLRW { w: self }
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap0mci1_fe_clr(&mut self) -> _CAP0MCI1_FE_CLRW {
        _CAP0MCI1_FE_CLRW { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap0mci2_re_clr(&mut self) -> _CAP0MCI2_RE_CLRW {
        _CAP0MCI2_RE_CLRW { w: self }
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap0mci2_fe_clr(&mut self) -> _CAP0MCI2_FE_CLRW {
        _CAP0MCI2_FE_CLRW { w: self }
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap1mci0_re_clr(&mut self) -> _CAP1MCI0_RE_CLRW {
        _CAP1MCI0_RE_CLRW { w: self }
    }
    #[doc = "Bit 7 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap1mci0_fe_clr(&mut self) -> _CAP1MCI0_FE_CLRW {
        _CAP1MCI0_FE_CLRW { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap1mci1_re_clr(&mut self) -> _CAP1MCI1_RE_CLRW {
        _CAP1MCI1_RE_CLRW { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap1mci1_fe_clr(&mut self) -> _CAP1MCI1_FE_CLRW {
        _CAP1MCI1_FE_CLRW { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap1mci2_re_clr(&mut self) -> _CAP1MCI2_RE_CLRW {
        _CAP1MCI2_RE_CLRW { w: self }
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap1mci2_fe_clr(&mut self) -> _CAP1MCI2_FE_CLRW {
        _CAP1MCI2_FE_CLRW { w: self }
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap2mci0_re_clr(&mut self) -> _CAP2MCI0_RE_CLRW {
        _CAP2MCI0_RE_CLRW { w: self }
    }
    #[doc = "Bit 13 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap2mci0_fe_clr(&mut self) -> _CAP2MCI0_FE_CLRW {
        _CAP2MCI0_FE_CLRW { w: self }
    }
    #[doc = "Bit 14 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap2mci1_re_clr(&mut self) -> _CAP2MCI1_RE_CLRW {
        _CAP2MCI1_RE_CLRW { w: self }
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap2mci1_fe_clr(&mut self) -> _CAP2MCI1_FE_CLRW {
        _CAP2MCI1_FE_CLRW { w: self }
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap2mci2_re_clr(&mut self) -> _CAP2MCI2_RE_CLRW {
        _CAP2MCI2_RE_CLRW { w: self }
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn cap2mci2_fe_clr(&mut self) -> _CAP2MCI2_FE_CLRW {
        _CAP2MCI2_FE_CLRW { w: self }
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn rt0_clr(&mut self) -> _RT0_CLRW {
        _RT0_CLRW { w: self }
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn rt1_clr(&mut self) -> _RT1_CLRW {
        _RT1_CLRW { w: self }
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline]
    pub fn rt2_clr(&mut self) -> _RT2_CLRW {
        _RT2_CLRW { w: self }
    }
}
