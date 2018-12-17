#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTSET {
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
pub struct _TMR_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_SETW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _REMOVE_PU_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _REMOVE_PU_SETW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _HNP_FAILURE_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_FAILURE_SETW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _HNP_SUCCES_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _HNP_SUCCES_SETW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline]
    pub fn tmr_set(&mut self) -> _TMR_SETW {
        _TMR_SETW { w: self }
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline]
    pub fn remove_pu_set(&mut self) -> _REMOVE_PU_SETW {
        _REMOVE_PU_SETW { w: self }
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline]
    pub fn hnp_failure_set(&mut self) -> _HNP_FAILURE_SETW {
        _HNP_FAILURE_SETW { w: self }
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline]
    pub fn hnp_succes_set(&mut self) -> _HNP_SUCCES_SETW {
        _HNP_SUCCES_SETW { w: self }
    }
}
