#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NDDRINTSET {
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
pub struct _EPNDDINTSET0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET0W<'a> {
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
pub struct _EPNDDINTSET1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET1W<'a> {
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
pub struct _EPNDDINTSET2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET2W<'a> {
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
pub struct _EPNDDINTSET3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET3W<'a> {
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
pub struct _EPNDDINTSET4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET4W<'a> {
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
pub struct _EPNDDINTSET5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET5W<'a> {
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
pub struct _EPNDDINTSET6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET6W<'a> {
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
pub struct _EPNDDINTSET7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET7W<'a> {
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
pub struct _EPNDDINTSET8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET8W<'a> {
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
pub struct _EPNDDINTSET9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET9W<'a> {
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
pub struct _EPNDDINTSET10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET10W<'a> {
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
pub struct _EPNDDINTSET11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET11W<'a> {
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
pub struct _EPNDDINTSET12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET12W<'a> {
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
pub struct _EPNDDINTSET13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET13W<'a> {
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
pub struct _EPNDDINTSET14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET14W<'a> {
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
pub struct _EPNDDINTSET15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET15W<'a> {
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
pub struct _EPNDDINTSET16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET16W<'a> {
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
pub struct _EPNDDINTSET17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET17W<'a> {
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
pub struct _EPNDDINTSET18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET18W<'a> {
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
pub struct _EPNDDINTSET19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET19W<'a> {
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
pub struct _EPNDDINTSET20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET20W<'a> {
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
pub struct _EPNDDINTSET21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET21W<'a> {
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
pub struct _EPNDDINTSET22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET22W<'a> {
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
pub struct _EPNDDINTSET23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET23W<'a> {
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
pub struct _EPNDDINTSET24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET24W<'a> {
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
pub struct _EPNDDINTSET25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET25W<'a> {
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
pub struct _EPNDDINTSET26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET26W<'a> {
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
pub struct _EPNDDINTSET27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET27W<'a> {
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
pub struct _EPNDDINTSET28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET28W<'a> {
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
pub struct _EPNDDINTSET29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET29W<'a> {
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
pub struct _EPNDDINTSET30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET30W<'a> {
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
pub struct _EPNDDINTSET31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPNDDINTSET31W<'a> {
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
    #[doc = "Bit 0 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset0(&mut self) -> _EPNDDINTSET0W {
        _EPNDDINTSET0W { w: self }
    }
    #[doc = "Bit 1 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset1(&mut self) -> _EPNDDINTSET1W {
        _EPNDDINTSET1W { w: self }
    }
    #[doc = "Bit 2 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset2(&mut self) -> _EPNDDINTSET2W {
        _EPNDDINTSET2W { w: self }
    }
    #[doc = "Bit 3 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset3(&mut self) -> _EPNDDINTSET3W {
        _EPNDDINTSET3W { w: self }
    }
    #[doc = "Bit 4 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset4(&mut self) -> _EPNDDINTSET4W {
        _EPNDDINTSET4W { w: self }
    }
    #[doc = "Bit 5 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset5(&mut self) -> _EPNDDINTSET5W {
        _EPNDDINTSET5W { w: self }
    }
    #[doc = "Bit 6 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset6(&mut self) -> _EPNDDINTSET6W {
        _EPNDDINTSET6W { w: self }
    }
    #[doc = "Bit 7 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset7(&mut self) -> _EPNDDINTSET7W {
        _EPNDDINTSET7W { w: self }
    }
    #[doc = "Bit 8 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset8(&mut self) -> _EPNDDINTSET8W {
        _EPNDDINTSET8W { w: self }
    }
    #[doc = "Bit 9 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset9(&mut self) -> _EPNDDINTSET9W {
        _EPNDDINTSET9W { w: self }
    }
    #[doc = "Bit 10 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset10(&mut self) -> _EPNDDINTSET10W {
        _EPNDDINTSET10W { w: self }
    }
    #[doc = "Bit 11 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset11(&mut self) -> _EPNDDINTSET11W {
        _EPNDDINTSET11W { w: self }
    }
    #[doc = "Bit 12 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset12(&mut self) -> _EPNDDINTSET12W {
        _EPNDDINTSET12W { w: self }
    }
    #[doc = "Bit 13 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset13(&mut self) -> _EPNDDINTSET13W {
        _EPNDDINTSET13W { w: self }
    }
    #[doc = "Bit 14 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset14(&mut self) -> _EPNDDINTSET14W {
        _EPNDDINTSET14W { w: self }
    }
    #[doc = "Bit 15 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset15(&mut self) -> _EPNDDINTSET15W {
        _EPNDDINTSET15W { w: self }
    }
    #[doc = "Bit 16 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset16(&mut self) -> _EPNDDINTSET16W {
        _EPNDDINTSET16W { w: self }
    }
    #[doc = "Bit 17 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset17(&mut self) -> _EPNDDINTSET17W {
        _EPNDDINTSET17W { w: self }
    }
    #[doc = "Bit 18 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset18(&mut self) -> _EPNDDINTSET18W {
        _EPNDDINTSET18W { w: self }
    }
    #[doc = "Bit 19 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset19(&mut self) -> _EPNDDINTSET19W {
        _EPNDDINTSET19W { w: self }
    }
    #[doc = "Bit 20 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset20(&mut self) -> _EPNDDINTSET20W {
        _EPNDDINTSET20W { w: self }
    }
    #[doc = "Bit 21 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset21(&mut self) -> _EPNDDINTSET21W {
        _EPNDDINTSET21W { w: self }
    }
    #[doc = "Bit 22 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset22(&mut self) -> _EPNDDINTSET22W {
        _EPNDDINTSET22W { w: self }
    }
    #[doc = "Bit 23 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset23(&mut self) -> _EPNDDINTSET23W {
        _EPNDDINTSET23W { w: self }
    }
    #[doc = "Bit 24 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset24(&mut self) -> _EPNDDINTSET24W {
        _EPNDDINTSET24W { w: self }
    }
    #[doc = "Bit 25 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset25(&mut self) -> _EPNDDINTSET25W {
        _EPNDDINTSET25W { w: self }
    }
    #[doc = "Bit 26 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset26(&mut self) -> _EPNDDINTSET26W {
        _EPNDDINTSET26W { w: self }
    }
    #[doc = "Bit 27 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset27(&mut self) -> _EPNDDINTSET27W {
        _EPNDDINTSET27W { w: self }
    }
    #[doc = "Bit 28 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset28(&mut self) -> _EPNDDINTSET28W {
        _EPNDDINTSET28W { w: self }
    }
    #[doc = "Bit 29 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset29(&mut self) -> _EPNDDINTSET29W {
        _EPNDDINTSET29W { w: self }
    }
    #[doc = "Bit 30 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset30(&mut self) -> _EPNDDINTSET30W {
        _EPNDDINTSET30W { w: self }
    }
    #[doc = "Bit 31 - Set endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Set the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline]
    pub fn epnddintset31(&mut self) -> _EPNDDINTSET31W {
        _EPNDDINTSET31W { w: self }
    }
}
