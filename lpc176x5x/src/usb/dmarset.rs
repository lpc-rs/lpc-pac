#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMARSET {
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
pub struct _EPRSET0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET0W<'a> {
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
pub struct _EPRSET1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET1W<'a> {
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
pub struct _EPRSET2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET2W<'a> {
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
pub struct _EPRSET3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET3W<'a> {
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
pub struct _EPRSET4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET4W<'a> {
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
pub struct _EPRSET5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET5W<'a> {
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
pub struct _EPRSET6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET6W<'a> {
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
pub struct _EPRSET7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET7W<'a> {
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
pub struct _EPRSET8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET8W<'a> {
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
pub struct _EPRSET9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET9W<'a> {
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
pub struct _EPRSET10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET10W<'a> {
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
pub struct _EPRSET11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET11W<'a> {
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
pub struct _EPRSET12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET12W<'a> {
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
pub struct _EPRSET13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET13W<'a> {
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
pub struct _EPRSET14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET14W<'a> {
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
pub struct _EPRSET15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET15W<'a> {
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
pub struct _EPRSET16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET16W<'a> {
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
pub struct _EPRSET17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET17W<'a> {
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
pub struct _EPRSET18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET18W<'a> {
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
pub struct _EPRSET19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET19W<'a> {
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
pub struct _EPRSET20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET20W<'a> {
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
pub struct _EPRSET21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET21W<'a> {
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
pub struct _EPRSET22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET22W<'a> {
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
pub struct _EPRSET23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET23W<'a> {
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
pub struct _EPRSET24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET24W<'a> {
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
pub struct _EPRSET25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET25W<'a> {
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
pub struct _EPRSET26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET26W<'a> {
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
pub struct _EPRSET27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET27W<'a> {
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
pub struct _EPRSET28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET28W<'a> {
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
pub struct _EPRSET29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET29W<'a> {
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
pub struct _EPRSET30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET30W<'a> {
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
pub struct _EPRSET31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRSET31W<'a> {
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
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0 bit must be 0)."]
    #[inline]
    pub fn eprset0(&mut self) -> _EPRSET0W {
        _EPRSET0W { w: self }
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1 bit must be 0)."]
    #[inline]
    pub fn eprset1(&mut self) -> _EPRSET1W {
        _EPRSET1W { w: self }
    }
    #[doc = "Bit 2 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset2(&mut self) -> _EPRSET2W {
        _EPRSET2W { w: self }
    }
    #[doc = "Bit 3 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset3(&mut self) -> _EPRSET3W {
        _EPRSET3W { w: self }
    }
    #[doc = "Bit 4 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset4(&mut self) -> _EPRSET4W {
        _EPRSET4W { w: self }
    }
    #[doc = "Bit 5 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset5(&mut self) -> _EPRSET5W {
        _EPRSET5W { w: self }
    }
    #[doc = "Bit 6 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset6(&mut self) -> _EPRSET6W {
        _EPRSET6W { w: self }
    }
    #[doc = "Bit 7 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset7(&mut self) -> _EPRSET7W {
        _EPRSET7W { w: self }
    }
    #[doc = "Bit 8 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset8(&mut self) -> _EPRSET8W {
        _EPRSET8W { w: self }
    }
    #[doc = "Bit 9 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset9(&mut self) -> _EPRSET9W {
        _EPRSET9W { w: self }
    }
    #[doc = "Bit 10 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset10(&mut self) -> _EPRSET10W {
        _EPRSET10W { w: self }
    }
    #[doc = "Bit 11 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset11(&mut self) -> _EPRSET11W {
        _EPRSET11W { w: self }
    }
    #[doc = "Bit 12 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset12(&mut self) -> _EPRSET12W {
        _EPRSET12W { w: self }
    }
    #[doc = "Bit 13 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset13(&mut self) -> _EPRSET13W {
        _EPRSET13W { w: self }
    }
    #[doc = "Bit 14 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset14(&mut self) -> _EPRSET14W {
        _EPRSET14W { w: self }
    }
    #[doc = "Bit 15 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset15(&mut self) -> _EPRSET15W {
        _EPRSET15W { w: self }
    }
    #[doc = "Bit 16 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset16(&mut self) -> _EPRSET16W {
        _EPRSET16W { w: self }
    }
    #[doc = "Bit 17 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset17(&mut self) -> _EPRSET17W {
        _EPRSET17W { w: self }
    }
    #[doc = "Bit 18 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset18(&mut self) -> _EPRSET18W {
        _EPRSET18W { w: self }
    }
    #[doc = "Bit 19 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset19(&mut self) -> _EPRSET19W {
        _EPRSET19W { w: self }
    }
    #[doc = "Bit 20 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset20(&mut self) -> _EPRSET20W {
        _EPRSET20W { w: self }
    }
    #[doc = "Bit 21 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset21(&mut self) -> _EPRSET21W {
        _EPRSET21W { w: self }
    }
    #[doc = "Bit 22 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset22(&mut self) -> _EPRSET22W {
        _EPRSET22W { w: self }
    }
    #[doc = "Bit 23 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset23(&mut self) -> _EPRSET23W {
        _EPRSET23W { w: self }
    }
    #[doc = "Bit 24 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset24(&mut self) -> _EPRSET24W {
        _EPRSET24W { w: self }
    }
    #[doc = "Bit 25 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset25(&mut self) -> _EPRSET25W {
        _EPRSET25W { w: self }
    }
    #[doc = "Bit 26 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset26(&mut self) -> _EPRSET26W {
        _EPRSET26W { w: self }
    }
    #[doc = "Bit 27 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset27(&mut self) -> _EPRSET27W {
        _EPRSET27W { w: self }
    }
    #[doc = "Bit 28 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset28(&mut self) -> _EPRSET28W {
        _EPRSET28W { w: self }
    }
    #[doc = "Bit 29 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset29(&mut self) -> _EPRSET29W {
        _EPRSET29W { w: self }
    }
    #[doc = "Bit 30 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset30(&mut self) -> _EPRSET30W {
        _EPRSET30W { w: self }
    }
    #[doc = "Bit 31 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline]
    pub fn eprset31(&mut self) -> _EPRSET31W {
        _EPRSET31W { w: self }
    }
}
