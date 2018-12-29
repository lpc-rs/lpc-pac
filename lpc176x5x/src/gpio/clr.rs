#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLR {
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
pub struct _PINCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR0W<'a> {
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
pub struct _PINCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR1W<'a> {
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
pub struct _PINCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR2W<'a> {
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
pub struct _PINCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR3W<'a> {
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
pub struct _PINCLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR4W<'a> {
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
pub struct _PINCLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR5W<'a> {
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
pub struct _PINCLR6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR6W<'a> {
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
pub struct _PINCLR7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR7W<'a> {
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
pub struct _PINCLR8W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR8W<'a> {
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
pub struct _PINCLR9W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR9W<'a> {
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
pub struct _PINCLR10W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR10W<'a> {
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
pub struct _PINCLR11W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR11W<'a> {
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
pub struct _PINCLR12W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR12W<'a> {
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
pub struct _PINCLR13W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR13W<'a> {
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
pub struct _PINCLR14W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR14W<'a> {
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
pub struct _PINCLR15W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR15W<'a> {
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
pub struct _PINCLR16W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR16W<'a> {
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
pub struct _PINCLR17W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR17W<'a> {
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
pub struct _PINCLR18W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR18W<'a> {
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
pub struct _PINCLR19W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR19W<'a> {
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
pub struct _PINCLR20W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR20W<'a> {
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
pub struct _PINCLR21W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR21W<'a> {
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
pub struct _PINCLR22W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR22W<'a> {
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
pub struct _PINCLR23W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR23W<'a> {
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
pub struct _PINCLR24W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR24W<'a> {
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
pub struct _PINCLR25W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR25W<'a> {
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
pub struct _PINCLR26W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR26W<'a> {
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
pub struct _PINCLR27W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR27W<'a> {
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
pub struct _PINCLR28W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR28W<'a> {
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
pub struct _PINCLR29W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR29W<'a> {
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
pub struct _PINCLR30W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR30W<'a> {
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
pub struct _PINCLR31W<'a> {
    w: &'a mut W,
}
impl<'a> _PINCLR31W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr0(&mut self) -> _PINCLR0W {
        _PINCLR0W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr1(&mut self) -> _PINCLR1W {
        _PINCLR1W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr2(&mut self) -> _PINCLR2W {
        _PINCLR2W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr3(&mut self) -> _PINCLR3W {
        _PINCLR3W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr4(&mut self) -> _PINCLR4W {
        _PINCLR4W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr5(&mut self) -> _PINCLR5W {
        _PINCLR5W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr6(&mut self) -> _PINCLR6W {
        _PINCLR6W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr7(&mut self) -> _PINCLR7W {
        _PINCLR7W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr8(&mut self) -> _PINCLR8W {
        _PINCLR8W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr9(&mut self) -> _PINCLR9W {
        _PINCLR9W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr10(&mut self) -> _PINCLR10W {
        _PINCLR10W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr11(&mut self) -> _PINCLR11W {
        _PINCLR11W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr12(&mut self) -> _PINCLR12W {
        _PINCLR12W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr13(&mut self) -> _PINCLR13W {
        _PINCLR13W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr14(&mut self) -> _PINCLR14W {
        _PINCLR14W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr15(&mut self) -> _PINCLR15W {
        _PINCLR15W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr16(&mut self) -> _PINCLR16W {
        _PINCLR16W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr17(&mut self) -> _PINCLR17W {
        _PINCLR17W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr18(&mut self) -> _PINCLR18W {
        _PINCLR18W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr19(&mut self) -> _PINCLR19W {
        _PINCLR19W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr20(&mut self) -> _PINCLR20W {
        _PINCLR20W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr21(&mut self) -> _PINCLR21W {
        _PINCLR21W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr22(&mut self) -> _PINCLR22W {
        _PINCLR22W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr23(&mut self) -> _PINCLR23W {
        _PINCLR23W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr24(&mut self) -> _PINCLR24W {
        _PINCLR24W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr25(&mut self) -> _PINCLR25W {
        _PINCLR25W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr26(&mut self) -> _PINCLR26W {
        _PINCLR26W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr27(&mut self) -> _PINCLR27W {
        _PINCLR27W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr28(&mut self) -> _PINCLR28W {
        _PINCLR28W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr29(&mut self) -> _PINCLR29W {
        _PINCLR29W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr30(&mut self) -> _PINCLR30W {
        _PINCLR30W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline]
    pub fn pinclr31(&mut self) -> _PINCLR31W {
        _PINCLR31W { w: self }
    }
}
