#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTCLR {
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
pub struct _TMR_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _REMOVE_PU_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _REMOVE_PU_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _HNP_FAILURE_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_FAILURE_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _HNP_SUCCES_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_SUCCES_CLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline]
    pub fn tmr_clr(&mut self) -> _TMR_CLRW {
        _TMR_CLRW { w: self }
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline]
    pub fn remove_pu_clr(&mut self) -> _REMOVE_PU_CLRW {
        _REMOVE_PU_CLRW { w: self }
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline]
    pub fn hnp_failure_clr(&mut self) -> _HNP_FAILURE_CLRW {
        _HNP_FAILURE_CLRW { w: self }
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline]
    pub fn hnp_succes_clr(&mut self) -> _HNP_SUCCES_CLRW {
        _HNP_SUCCES_CLRW { w: self }
    }
}
