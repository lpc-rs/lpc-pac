#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NOT {
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
pub struct _NOTP0W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP0W<'a> {
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
pub struct _NOTP1W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP1W<'a> {
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
pub struct _NOTP2W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP2W<'a> {
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
pub struct _NOTP3W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP3W<'a> {
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
pub struct _NOTP4W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP4W<'a> {
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
pub struct _NOTP5W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP5W<'a> {
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
pub struct _NOTP6W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP6W<'a> {
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
pub struct _NOTP7W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP7W<'a> {
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
pub struct _NOTP8W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP8W<'a> {
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
pub struct _NOTP9W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP9W<'a> {
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
pub struct _NOTP10W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP10W<'a> {
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
pub struct _NOTP11W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP11W<'a> {
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
pub struct _NOTP12W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP12W<'a> {
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
pub struct _NOTP13W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP13W<'a> {
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
pub struct _NOTP14W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP14W<'a> {
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
pub struct _NOTP15W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP15W<'a> {
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
pub struct _NOTP16W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP16W<'a> {
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
pub struct _NOTP17W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP17W<'a> {
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
pub struct _NOTP18W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP18W<'a> {
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
pub struct _NOTP19W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP19W<'a> {
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
pub struct _NOTP20W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP20W<'a> {
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
pub struct _NOTP21W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP21W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP22W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP22W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP23W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP23W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP24W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP24W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP25W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP25W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP26W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP26W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP27W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP27W<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP28W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP28W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOTP29W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP29W<'a> {
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
pub struct _NOTP30W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP30W<'a> {
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
pub struct _NOTP31W<'a> {
    w: &'a mut W,
}
impl<'a> _NOTP31W<'a> {
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
    #[doc = "Bit 0 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp0(&mut self) -> _NOTP0W {
        _NOTP0W { w: self }
    }
    #[doc = "Bit 1 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp1(&mut self) -> _NOTP1W {
        _NOTP1W { w: self }
    }
    #[doc = "Bit 2 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp2(&mut self) -> _NOTP2W {
        _NOTP2W { w: self }
    }
    #[doc = "Bit 3 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp3(&mut self) -> _NOTP3W {
        _NOTP3W { w: self }
    }
    #[doc = "Bit 4 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp4(&mut self) -> _NOTP4W {
        _NOTP4W { w: self }
    }
    #[doc = "Bit 5 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp5(&mut self) -> _NOTP5W {
        _NOTP5W { w: self }
    }
    #[doc = "Bit 6 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp6(&mut self) -> _NOTP6W {
        _NOTP6W { w: self }
    }
    #[doc = "Bit 7 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp7(&mut self) -> _NOTP7W {
        _NOTP7W { w: self }
    }
    #[doc = "Bit 8 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp8(&mut self) -> _NOTP8W {
        _NOTP8W { w: self }
    }
    #[doc = "Bit 9 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp9(&mut self) -> _NOTP9W {
        _NOTP9W { w: self }
    }
    #[doc = "Bit 10 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp10(&mut self) -> _NOTP10W {
        _NOTP10W { w: self }
    }
    #[doc = "Bit 11 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp11(&mut self) -> _NOTP11W {
        _NOTP11W { w: self }
    }
    #[doc = "Bit 12 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp12(&mut self) -> _NOTP12W {
        _NOTP12W { w: self }
    }
    #[doc = "Bit 13 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp13(&mut self) -> _NOTP13W {
        _NOTP13W { w: self }
    }
    #[doc = "Bit 14 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp14(&mut self) -> _NOTP14W {
        _NOTP14W { w: self }
    }
    #[doc = "Bit 15 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp15(&mut self) -> _NOTP15W {
        _NOTP15W { w: self }
    }
    #[doc = "Bit 16 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp16(&mut self) -> _NOTP16W {
        _NOTP16W { w: self }
    }
    #[doc = "Bit 17 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp17(&mut self) -> _NOTP17W {
        _NOTP17W { w: self }
    }
    #[doc = "Bit 18 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp18(&mut self) -> _NOTP18W {
        _NOTP18W { w: self }
    }
    #[doc = "Bit 19 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp19(&mut self) -> _NOTP19W {
        _NOTP19W { w: self }
    }
    #[doc = "Bit 20 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp20(&mut self) -> _NOTP20W {
        _NOTP20W { w: self }
    }
    #[doc = "Bit 21 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp21(&mut self) -> _NOTP21W {
        _NOTP21W { w: self }
    }
    #[doc = "Bit 22 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp22(&mut self) -> _NOTP22W {
        _NOTP22W { w: self }
    }
    #[doc = "Bit 23 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp23(&mut self) -> _NOTP23W {
        _NOTP23W { w: self }
    }
    #[doc = "Bit 24 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp24(&mut self) -> _NOTP24W {
        _NOTP24W { w: self }
    }
    #[doc = "Bit 25 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp25(&mut self) -> _NOTP25W {
        _NOTP25W { w: self }
    }
    #[doc = "Bit 26 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp26(&mut self) -> _NOTP26W {
        _NOTP26W { w: self }
    }
    #[doc = "Bit 27 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp27(&mut self) -> _NOTP27W {
        _NOTP27W { w: self }
    }
    #[doc = "Bit 28 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp28(&mut self) -> _NOTP28W {
        _NOTP28W { w: self }
    }
    #[doc = "Bit 29 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp29(&mut self) -> _NOTP29W {
        _NOTP29W { w: self }
    }
    #[doc = "Bit 30 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp30(&mut self) -> _NOTP30W {
        _NOTP30W { w: self }
    }
    #[doc = "Bit 31 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline]
    pub fn notp31(&mut self) -> _NOTP31W {
        _NOTP31W { w: self }
    }
}
