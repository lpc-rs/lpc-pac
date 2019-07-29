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
pub struct _CLRP00W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP00W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP01W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP01W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP02W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP02W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP03W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP03W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP04W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP04W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP05W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP05W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP06W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP06W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP07W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP07W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP08W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP08W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP09W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP09W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP010W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP010W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP011W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP011W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP012W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP012W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP013W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP013W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP014W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP014W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP015W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP015W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP016W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP016W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP017W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP017W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP018W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP018W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP019W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP019W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP020W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP020W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP021W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP021W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP022W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP022W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP023W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP023W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP024W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP024W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP025W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP025W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP026W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP026W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP027W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP027W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP028W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP028W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP029W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP029W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP030W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP030W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CLRP031W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRP031W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp00(&mut self) -> _CLRP00W {
        _CLRP00W { w: self }
    }
    #[doc = "Bit 1 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp01(&mut self) -> _CLRP01W {
        _CLRP01W { w: self }
    }
    #[doc = "Bit 2 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp02(&mut self) -> _CLRP02W {
        _CLRP02W { w: self }
    }
    #[doc = "Bit 3 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp03(&mut self) -> _CLRP03W {
        _CLRP03W { w: self }
    }
    #[doc = "Bit 4 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp04(&mut self) -> _CLRP04W {
        _CLRP04W { w: self }
    }
    #[doc = "Bit 5 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp05(&mut self) -> _CLRP05W {
        _CLRP05W { w: self }
    }
    #[doc = "Bit 6 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp06(&mut self) -> _CLRP06W {
        _CLRP06W { w: self }
    }
    #[doc = "Bit 7 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp07(&mut self) -> _CLRP07W {
        _CLRP07W { w: self }
    }
    #[doc = "Bit 8 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp08(&mut self) -> _CLRP08W {
        _CLRP08W { w: self }
    }
    #[doc = "Bit 9 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp09(&mut self) -> _CLRP09W {
        _CLRP09W { w: self }
    }
    #[doc = "Bit 10 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp010(&mut self) -> _CLRP010W {
        _CLRP010W { w: self }
    }
    #[doc = "Bit 11 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp011(&mut self) -> _CLRP011W {
        _CLRP011W { w: self }
    }
    #[doc = "Bit 12 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp012(&mut self) -> _CLRP012W {
        _CLRP012W { w: self }
    }
    #[doc = "Bit 13 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp013(&mut self) -> _CLRP013W {
        _CLRP013W { w: self }
    }
    #[doc = "Bit 14 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp014(&mut self) -> _CLRP014W {
        _CLRP014W { w: self }
    }
    #[doc = "Bit 15 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp015(&mut self) -> _CLRP015W {
        _CLRP015W { w: self }
    }
    #[doc = "Bit 16 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp016(&mut self) -> _CLRP016W {
        _CLRP016W { w: self }
    }
    #[doc = "Bit 17 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp017(&mut self) -> _CLRP017W {
        _CLRP017W { w: self }
    }
    #[doc = "Bit 18 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp018(&mut self) -> _CLRP018W {
        _CLRP018W { w: self }
    }
    #[doc = "Bit 19 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp019(&mut self) -> _CLRP019W {
        _CLRP019W { w: self }
    }
    #[doc = "Bit 20 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp020(&mut self) -> _CLRP020W {
        _CLRP020W { w: self }
    }
    #[doc = "Bit 21 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp021(&mut self) -> _CLRP021W {
        _CLRP021W { w: self }
    }
    #[doc = "Bit 22 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp022(&mut self) -> _CLRP022W {
        _CLRP022W { w: self }
    }
    #[doc = "Bit 23 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp023(&mut self) -> _CLRP023W {
        _CLRP023W { w: self }
    }
    #[doc = "Bit 24 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp024(&mut self) -> _CLRP024W {
        _CLRP024W { w: self }
    }
    #[doc = "Bit 25 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp025(&mut self) -> _CLRP025W {
        _CLRP025W { w: self }
    }
    #[doc = "Bit 26 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp026(&mut self) -> _CLRP026W {
        _CLRP026W { w: self }
    }
    #[doc = "Bit 27 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp027(&mut self) -> _CLRP027W {
        _CLRP027W { w: self }
    }
    #[doc = "Bit 28 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp028(&mut self) -> _CLRP028W {
        _CLRP028W { w: self }
    }
    #[doc = "Bit 29 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp029(&mut self) -> _CLRP029W {
        _CLRP029W { w: self }
    }
    #[doc = "Bit 30 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp030(&mut self) -> _CLRP030W {
        _CLRP030W { w: self }
    }
    #[doc = "Bit 31 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp031(&mut self) -> _CLRP031W {
        _CLRP031W { w: self }
    }
}
