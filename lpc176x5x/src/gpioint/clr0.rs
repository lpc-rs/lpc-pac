#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLR0 {
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
pub struct _P0_0CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_0CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_1CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_1CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_2CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_2CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_3CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_3CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_4CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_4CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_5CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_5CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_6CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_6CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_7CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_7CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_8CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_8CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_9CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_9CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_10CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_10CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_11CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_11CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_12CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_12CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_13CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_13CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_14CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_14CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_15CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_15CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_16CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_16CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_17CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_17CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_18CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_18CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_19CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_19CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_20CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_20CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_21CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_21CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_22CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_22CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_23CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_23CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_24CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_24CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_25CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_25CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_26CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_26CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_27CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_27CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_28CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_28CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_29CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_29CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _P0_30CIW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_30CIW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Clear GPIO port Interrupts for P0\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_0ci(&mut self) -> _P0_0CIW {
        _P0_0CIW { w: self }
    }
    #[doc = "Bit 1 - Clear GPIO port Interrupts for P0\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_1ci(&mut self) -> _P0_1CIW {
        _P0_1CIW { w: self }
    }
    #[doc = "Bit 2 - Clear GPIO port Interrupts for P0\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_2ci(&mut self) -> _P0_2CIW {
        _P0_2CIW { w: self }
    }
    #[doc = "Bit 3 - Clear GPIO port Interrupts for P0\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_3ci(&mut self) -> _P0_3CIW {
        _P0_3CIW { w: self }
    }
    #[doc = "Bit 4 - Clear GPIO port Interrupts for P0\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_4ci(&mut self) -> _P0_4CIW {
        _P0_4CIW { w: self }
    }
    #[doc = "Bit 5 - Clear GPIO port Interrupts for P0\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_5ci(&mut self) -> _P0_5CIW {
        _P0_5CIW { w: self }
    }
    #[doc = "Bit 6 - Clear GPIO port Interrupts for P0\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_6ci(&mut self) -> _P0_6CIW {
        _P0_6CIW { w: self }
    }
    #[doc = "Bit 7 - Clear GPIO port Interrupts for P0\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_7ci(&mut self) -> _P0_7CIW {
        _P0_7CIW { w: self }
    }
    #[doc = "Bit 8 - Clear GPIO port Interrupts for P0\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_8ci(&mut self) -> _P0_8CIW {
        _P0_8CIW { w: self }
    }
    #[doc = "Bit 9 - Clear GPIO port Interrupts for P0\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_9ci(&mut self) -> _P0_9CIW {
        _P0_9CIW { w: self }
    }
    #[doc = "Bit 10 - Clear GPIO port Interrupts for P0\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_10ci(&mut self) -> _P0_10CIW {
        _P0_10CIW { w: self }
    }
    #[doc = "Bit 11 - Clear GPIO port Interrupts for P0\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_11ci(&mut self) -> _P0_11CIW {
        _P0_11CIW { w: self }
    }
    #[doc = "Bit 12 - Clear GPIO port Interrupts for P0\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_12ci(&mut self) -> _P0_12CIW {
        _P0_12CIW { w: self }
    }
    #[doc = "Bit 13 - Clear GPIO port Interrupts for P0\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_13ci(&mut self) -> _P0_13CIW {
        _P0_13CIW { w: self }
    }
    #[doc = "Bit 14 - Clear GPIO port Interrupts for P0\\[14\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_14ci(&mut self) -> _P0_14CIW {
        _P0_14CIW { w: self }
    }
    #[doc = "Bit 15 - Clear GPIO port Interrupts for P0\\[15\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_15ci(&mut self) -> _P0_15CIW {
        _P0_15CIW { w: self }
    }
    #[doc = "Bit 16 - Clear GPIO port Interrupts for P0\\[16\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_16ci(&mut self) -> _P0_16CIW {
        _P0_16CIW { w: self }
    }
    #[doc = "Bit 17 - Clear GPIO port Interrupts for P0\\[17\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_17ci(&mut self) -> _P0_17CIW {
        _P0_17CIW { w: self }
    }
    #[doc = "Bit 18 - Clear GPIO port Interrupts for P0\\[18\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_18ci(&mut self) -> _P0_18CIW {
        _P0_18CIW { w: self }
    }
    #[doc = "Bit 19 - Clear GPIO port Interrupts for P0\\[19\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_19ci(&mut self) -> _P0_19CIW {
        _P0_19CIW { w: self }
    }
    #[doc = "Bit 20 - Clear GPIO port Interrupts for P0\\[20\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_20ci(&mut self) -> _P0_20CIW {
        _P0_20CIW { w: self }
    }
    #[doc = "Bit 21 - Clear GPIO port Interrupts for P0\\[21\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_21ci(&mut self) -> _P0_21CIW {
        _P0_21CIW { w: self }
    }
    #[doc = "Bit 22 - Clear GPIO port Interrupts for P0\\[22\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_22ci(&mut self) -> _P0_22CIW {
        _P0_22CIW { w: self }
    }
    #[doc = "Bit 23 - Clear GPIO port Interrupts for P0\\[23\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_23ci(&mut self) -> _P0_23CIW {
        _P0_23CIW { w: self }
    }
    #[doc = "Bit 24 - Clear GPIO port Interrupts for P0\\[24\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_24ci(&mut self) -> _P0_24CIW {
        _P0_24CIW { w: self }
    }
    #[doc = "Bit 25 - Clear GPIO port Interrupts for P0\\[25\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_25ci(&mut self) -> _P0_25CIW {
        _P0_25CIW { w: self }
    }
    #[doc = "Bit 26 - Clear GPIO port Interrupts for P0\\[26\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_26ci(&mut self) -> _P0_26CIW {
        _P0_26CIW { w: self }
    }
    #[doc = "Bit 27 - Clear GPIO port Interrupts for P0\\[27\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_27ci(&mut self) -> _P0_27CIW {
        _P0_27CIW { w: self }
    }
    #[doc = "Bit 28 - Clear GPIO port Interrupts for P0\\[28\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_28ci(&mut self) -> _P0_28CIW {
        _P0_28CIW { w: self }
    }
    #[doc = "Bit 29 - Clear GPIO port Interrupts for P0\\[29\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_29ci(&mut self) -> _P0_29CIW {
        _P0_29CIW { w: self }
    }
    #[doc = "Bit 30 - Clear GPIO port Interrupts for P0\\[30\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline]
    pub fn p0_30ci(&mut self) -> _P0_30CIW {
        _P0_30CIW { w: self }
    }
}
