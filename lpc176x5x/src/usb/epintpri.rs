#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPINTPRI {
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
pub struct _EPPRI0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI0W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI1W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI3W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI4W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI5W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI6W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI7W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI8W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI9W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI10W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI10W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI11W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI11W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI12W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI12W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI13W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI13W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI14W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI14W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI15W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI15W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI16W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI16W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI17W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI17W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI18W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI18W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI19W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI19W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI20W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI20W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI21W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI21W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI22W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI22W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI23W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI23W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI24W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI24W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI25W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI25W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI26W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI26W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI27W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI27W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI28W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI28W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI29W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI29W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI30W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI30W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EPPRI31W<'a> {
    w: &'a mut W,
}
impl<'a> _EPPRI31W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri0(&mut self) -> _EPPRI0W {
        _EPPRI0W { w: self }
    }
    #[doc = "Bit 1 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri1(&mut self) -> _EPPRI1W {
        _EPPRI1W { w: self }
    }
    #[doc = "Bit 2 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri2(&mut self) -> _EPPRI2W {
        _EPPRI2W { w: self }
    }
    #[doc = "Bit 3 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri3(&mut self) -> _EPPRI3W {
        _EPPRI3W { w: self }
    }
    #[doc = "Bit 4 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri4(&mut self) -> _EPPRI4W {
        _EPPRI4W { w: self }
    }
    #[doc = "Bit 5 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri5(&mut self) -> _EPPRI5W {
        _EPPRI5W { w: self }
    }
    #[doc = "Bit 6 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri6(&mut self) -> _EPPRI6W {
        _EPPRI6W { w: self }
    }
    #[doc = "Bit 7 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri7(&mut self) -> _EPPRI7W {
        _EPPRI7W { w: self }
    }
    #[doc = "Bit 8 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri8(&mut self) -> _EPPRI8W {
        _EPPRI8W { w: self }
    }
    #[doc = "Bit 9 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri9(&mut self) -> _EPPRI9W {
        _EPPRI9W { w: self }
    }
    #[doc = "Bit 10 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri10(&mut self) -> _EPPRI10W {
        _EPPRI10W { w: self }
    }
    #[doc = "Bit 11 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri11(&mut self) -> _EPPRI11W {
        _EPPRI11W { w: self }
    }
    #[doc = "Bit 12 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri12(&mut self) -> _EPPRI12W {
        _EPPRI12W { w: self }
    }
    #[doc = "Bit 13 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri13(&mut self) -> _EPPRI13W {
        _EPPRI13W { w: self }
    }
    #[doc = "Bit 14 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri14(&mut self) -> _EPPRI14W {
        _EPPRI14W { w: self }
    }
    #[doc = "Bit 15 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri15(&mut self) -> _EPPRI15W {
        _EPPRI15W { w: self }
    }
    #[doc = "Bit 16 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri16(&mut self) -> _EPPRI16W {
        _EPPRI16W { w: self }
    }
    #[doc = "Bit 17 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri17(&mut self) -> _EPPRI17W {
        _EPPRI17W { w: self }
    }
    #[doc = "Bit 18 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri18(&mut self) -> _EPPRI18W {
        _EPPRI18W { w: self }
    }
    #[doc = "Bit 19 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri19(&mut self) -> _EPPRI19W {
        _EPPRI19W { w: self }
    }
    #[doc = "Bit 20 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri20(&mut self) -> _EPPRI20W {
        _EPPRI20W { w: self }
    }
    #[doc = "Bit 21 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri21(&mut self) -> _EPPRI21W {
        _EPPRI21W { w: self }
    }
    #[doc = "Bit 22 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri22(&mut self) -> _EPPRI22W {
        _EPPRI22W { w: self }
    }
    #[doc = "Bit 23 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri23(&mut self) -> _EPPRI23W {
        _EPPRI23W { w: self }
    }
    #[doc = "Bit 24 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri24(&mut self) -> _EPPRI24W {
        _EPPRI24W { w: self }
    }
    #[doc = "Bit 25 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri25(&mut self) -> _EPPRI25W {
        _EPPRI25W { w: self }
    }
    #[doc = "Bit 26 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri26(&mut self) -> _EPPRI26W {
        _EPPRI26W { w: self }
    }
    #[doc = "Bit 27 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri27(&mut self) -> _EPPRI27W {
        _EPPRI27W { w: self }
    }
    #[doc = "Bit 28 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri28(&mut self) -> _EPPRI28W {
        _EPPRI28W { w: self }
    }
    #[doc = "Bit 29 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri29(&mut self) -> _EPPRI29W {
        _EPPRI29W { w: self }
    }
    #[doc = "Bit 30 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri30(&mut self) -> _EPPRI30W {
        _EPPRI30W { w: self }
    }
    #[doc = "Bit 31 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline]
    pub fn eppri31(&mut self) -> _EPPRI31W {
        _EPPRI31W { w: self }
    }
}
