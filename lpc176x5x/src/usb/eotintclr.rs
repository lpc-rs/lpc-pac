#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EOTINTCLR {
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
pub struct _EPTXINTCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR0W<'a> {
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
pub struct _EPTXINTCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR1W<'a> {
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
pub struct _EPTXINTCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR2W<'a> {
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
pub struct _EPTXINTCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR3W<'a> {
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
pub struct _EPTXINTCLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR4W<'a> {
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
pub struct _EPTXINTCLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR5W<'a> {
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
pub struct _EPTXINTCLR6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR6W<'a> {
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
pub struct _EPTXINTCLR7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR7W<'a> {
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
pub struct _EPTXINTCLR8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR8W<'a> {
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
pub struct _EPTXINTCLR9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR9W<'a> {
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
pub struct _EPTXINTCLR10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR10W<'a> {
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
pub struct _EPTXINTCLR11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR11W<'a> {
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
pub struct _EPTXINTCLR12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR12W<'a> {
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
pub struct _EPTXINTCLR13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR13W<'a> {
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
pub struct _EPTXINTCLR14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR14W<'a> {
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
pub struct _EPTXINTCLR15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR15W<'a> {
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
pub struct _EPTXINTCLR16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR16W<'a> {
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
pub struct _EPTXINTCLR17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR17W<'a> {
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
pub struct _EPTXINTCLR18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR18W<'a> {
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
pub struct _EPTXINTCLR19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR19W<'a> {
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
pub struct _EPTXINTCLR20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR20W<'a> {
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
pub struct _EPTXINTCLR21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR21W<'a> {
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
pub struct _EPTXINTCLR22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR22W<'a> {
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
pub struct _EPTXINTCLR23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR23W<'a> {
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
pub struct _EPTXINTCLR24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR24W<'a> {
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
pub struct _EPTXINTCLR25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR25W<'a> {
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
pub struct _EPTXINTCLR26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR26W<'a> {
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
pub struct _EPTXINTCLR27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR27W<'a> {
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
pub struct _EPTXINTCLR28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR28W<'a> {
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
pub struct _EPTXINTCLR29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR29W<'a> {
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
pub struct _EPTXINTCLR30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR30W<'a> {
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
pub struct _EPTXINTCLR31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXINTCLR31W<'a> {
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
    #[doc = "Bit 0 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr0(&mut self) -> _EPTXINTCLR0W {
        _EPTXINTCLR0W { w: self }
    }
    #[doc = "Bit 1 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr1(&mut self) -> _EPTXINTCLR1W {
        _EPTXINTCLR1W { w: self }
    }
    #[doc = "Bit 2 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr2(&mut self) -> _EPTXINTCLR2W {
        _EPTXINTCLR2W { w: self }
    }
    #[doc = "Bit 3 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr3(&mut self) -> _EPTXINTCLR3W {
        _EPTXINTCLR3W { w: self }
    }
    #[doc = "Bit 4 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr4(&mut self) -> _EPTXINTCLR4W {
        _EPTXINTCLR4W { w: self }
    }
    #[doc = "Bit 5 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr5(&mut self) -> _EPTXINTCLR5W {
        _EPTXINTCLR5W { w: self }
    }
    #[doc = "Bit 6 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr6(&mut self) -> _EPTXINTCLR6W {
        _EPTXINTCLR6W { w: self }
    }
    #[doc = "Bit 7 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr7(&mut self) -> _EPTXINTCLR7W {
        _EPTXINTCLR7W { w: self }
    }
    #[doc = "Bit 8 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr8(&mut self) -> _EPTXINTCLR8W {
        _EPTXINTCLR8W { w: self }
    }
    #[doc = "Bit 9 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr9(&mut self) -> _EPTXINTCLR9W {
        _EPTXINTCLR9W { w: self }
    }
    #[doc = "Bit 10 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr10(&mut self) -> _EPTXINTCLR10W {
        _EPTXINTCLR10W { w: self }
    }
    #[doc = "Bit 11 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr11(&mut self) -> _EPTXINTCLR11W {
        _EPTXINTCLR11W { w: self }
    }
    #[doc = "Bit 12 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr12(&mut self) -> _EPTXINTCLR12W {
        _EPTXINTCLR12W { w: self }
    }
    #[doc = "Bit 13 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr13(&mut self) -> _EPTXINTCLR13W {
        _EPTXINTCLR13W { w: self }
    }
    #[doc = "Bit 14 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr14(&mut self) -> _EPTXINTCLR14W {
        _EPTXINTCLR14W { w: self }
    }
    #[doc = "Bit 15 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr15(&mut self) -> _EPTXINTCLR15W {
        _EPTXINTCLR15W { w: self }
    }
    #[doc = "Bit 16 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr16(&mut self) -> _EPTXINTCLR16W {
        _EPTXINTCLR16W { w: self }
    }
    #[doc = "Bit 17 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr17(&mut self) -> _EPTXINTCLR17W {
        _EPTXINTCLR17W { w: self }
    }
    #[doc = "Bit 18 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr18(&mut self) -> _EPTXINTCLR18W {
        _EPTXINTCLR18W { w: self }
    }
    #[doc = "Bit 19 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr19(&mut self) -> _EPTXINTCLR19W {
        _EPTXINTCLR19W { w: self }
    }
    #[doc = "Bit 20 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr20(&mut self) -> _EPTXINTCLR20W {
        _EPTXINTCLR20W { w: self }
    }
    #[doc = "Bit 21 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr21(&mut self) -> _EPTXINTCLR21W {
        _EPTXINTCLR21W { w: self }
    }
    #[doc = "Bit 22 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr22(&mut self) -> _EPTXINTCLR22W {
        _EPTXINTCLR22W { w: self }
    }
    #[doc = "Bit 23 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr23(&mut self) -> _EPTXINTCLR23W {
        _EPTXINTCLR23W { w: self }
    }
    #[doc = "Bit 24 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr24(&mut self) -> _EPTXINTCLR24W {
        _EPTXINTCLR24W { w: self }
    }
    #[doc = "Bit 25 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr25(&mut self) -> _EPTXINTCLR25W {
        _EPTXINTCLR25W { w: self }
    }
    #[doc = "Bit 26 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr26(&mut self) -> _EPTXINTCLR26W {
        _EPTXINTCLR26W { w: self }
    }
    #[doc = "Bit 27 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr27(&mut self) -> _EPTXINTCLR27W {
        _EPTXINTCLR27W { w: self }
    }
    #[doc = "Bit 28 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr28(&mut self) -> _EPTXINTCLR28W {
        _EPTXINTCLR28W { w: self }
    }
    #[doc = "Bit 29 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr29(&mut self) -> _EPTXINTCLR29W {
        _EPTXINTCLR29W { w: self }
    }
    #[doc = "Bit 30 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr30(&mut self) -> _EPTXINTCLR30W {
        _EPTXINTCLR30W { w: self }
    }
    #[doc = "Bit 31 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline]
    pub fn eptxintclr31(&mut self) -> _EPTXINTCLR31W {
        _EPTXINTCLR31W { w: self }
    }
}
