#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPINTSET {
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
pub struct _EPSET0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET0W<'a> {
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
pub struct _EPSET1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET1W<'a> {
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
pub struct _EPSET2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET2W<'a> {
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
pub struct _EPSET3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET3W<'a> {
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
pub struct _EPSET4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET4W<'a> {
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
pub struct _EPSET5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET5W<'a> {
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
pub struct _EPSET6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET6W<'a> {
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
pub struct _EPSET7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET7W<'a> {
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
pub struct _EPSET8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET8W<'a> {
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
pub struct _EPSET9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET9W<'a> {
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
pub struct _EPSET10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET10W<'a> {
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
pub struct _EPSET11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET11W<'a> {
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
pub struct _EPSET12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET12W<'a> {
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
pub struct _EPSET13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET13W<'a> {
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
pub struct _EPSET14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET14W<'a> {
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
pub struct _EPSET15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET15W<'a> {
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
pub struct _EPSET16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET16W<'a> {
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
pub struct _EPSET17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET17W<'a> {
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
pub struct _EPSET18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET18W<'a> {
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
pub struct _EPSET19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET19W<'a> {
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
pub struct _EPSET20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET20W<'a> {
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
pub struct _EPSET21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET21W<'a> {
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
pub struct _EPSET22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET22W<'a> {
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
pub struct _EPSET23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET23W<'a> {
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
pub struct _EPSET24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET24W<'a> {
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
pub struct _EPSET25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET25W<'a> {
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
pub struct _EPSET26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET26W<'a> {
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
pub struct _EPSET27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET27W<'a> {
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
pub struct _EPSET28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET28W<'a> {
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
pub struct _EPSET29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET29W<'a> {
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
pub struct _EPSET30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET30W<'a> {
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
pub struct _EPSET31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSET31W<'a> {
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
    #[doc = "Bit 0 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset0(&mut self) -> _EPSET0W {
        _EPSET0W { w: self }
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset1(&mut self) -> _EPSET1W {
        _EPSET1W { w: self }
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset2(&mut self) -> _EPSET2W {
        _EPSET2W { w: self }
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset3(&mut self) -> _EPSET3W {
        _EPSET3W { w: self }
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset4(&mut self) -> _EPSET4W {
        _EPSET4W { w: self }
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset5(&mut self) -> _EPSET5W {
        _EPSET5W { w: self }
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset6(&mut self) -> _EPSET6W {
        _EPSET6W { w: self }
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset7(&mut self) -> _EPSET7W {
        _EPSET7W { w: self }
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset8(&mut self) -> _EPSET8W {
        _EPSET8W { w: self }
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset9(&mut self) -> _EPSET9W {
        _EPSET9W { w: self }
    }
    #[doc = "Bit 10 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset10(&mut self) -> _EPSET10W {
        _EPSET10W { w: self }
    }
    #[doc = "Bit 11 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset11(&mut self) -> _EPSET11W {
        _EPSET11W { w: self }
    }
    #[doc = "Bit 12 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset12(&mut self) -> _EPSET12W {
        _EPSET12W { w: self }
    }
    #[doc = "Bit 13 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset13(&mut self) -> _EPSET13W {
        _EPSET13W { w: self }
    }
    #[doc = "Bit 14 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset14(&mut self) -> _EPSET14W {
        _EPSET14W { w: self }
    }
    #[doc = "Bit 15 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset15(&mut self) -> _EPSET15W {
        _EPSET15W { w: self }
    }
    #[doc = "Bit 16 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset16(&mut self) -> _EPSET16W {
        _EPSET16W { w: self }
    }
    #[doc = "Bit 17 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset17(&mut self) -> _EPSET17W {
        _EPSET17W { w: self }
    }
    #[doc = "Bit 18 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset18(&mut self) -> _EPSET18W {
        _EPSET18W { w: self }
    }
    #[doc = "Bit 19 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset19(&mut self) -> _EPSET19W {
        _EPSET19W { w: self }
    }
    #[doc = "Bit 20 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset20(&mut self) -> _EPSET20W {
        _EPSET20W { w: self }
    }
    #[doc = "Bit 21 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset21(&mut self) -> _EPSET21W {
        _EPSET21W { w: self }
    }
    #[doc = "Bit 22 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset22(&mut self) -> _EPSET22W {
        _EPSET22W { w: self }
    }
    #[doc = "Bit 23 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset23(&mut self) -> _EPSET23W {
        _EPSET23W { w: self }
    }
    #[doc = "Bit 24 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset24(&mut self) -> _EPSET24W {
        _EPSET24W { w: self }
    }
    #[doc = "Bit 25 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset25(&mut self) -> _EPSET25W {
        _EPSET25W { w: self }
    }
    #[doc = "Bit 26 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset26(&mut self) -> _EPSET26W {
        _EPSET26W { w: self }
    }
    #[doc = "Bit 27 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset27(&mut self) -> _EPSET27W {
        _EPSET27W { w: self }
    }
    #[doc = "Bit 28 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset28(&mut self) -> _EPSET28W {
        _EPSET28W { w: self }
    }
    #[doc = "Bit 29 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset29(&mut self) -> _EPSET29W {
        _EPSET29W { w: self }
    }
    #[doc = "Bit 30 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset30(&mut self) -> _EPSET30W {
        _EPSET30W { w: self }
    }
    #[doc = "Bit 31 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline]
    pub fn epset31(&mut self) -> _EPSET31W {
        _EPSET31W { w: self }
    }
}
