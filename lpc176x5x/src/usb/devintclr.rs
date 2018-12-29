#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVINTCLR {
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
pub struct _FRAMECLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMECLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EP_FASTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_FASTCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EP_SLOWCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_SLOWCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _DEV_STATCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_STATCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CCEMPTYCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEMPTYCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _CDFULLCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CDFULLCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _RXENDPKTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXENDPKTCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _TXENDPKTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENDPKTCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _EP_RLZEDCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_RLZEDCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
pub struct _ERR_INTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERR_INTCLRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn frameclr(&mut self) -> _FRAMECLRW {
        _FRAMECLRW { w: self }
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn ep_fastclr(&mut self) -> _EP_FASTCLRW {
        _EP_FASTCLRW { w: self }
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn ep_slowclr(&mut self) -> _EP_SLOWCLRW {
        _EP_SLOWCLRW { w: self }
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn dev_statclr(&mut self) -> _DEV_STATCLRW {
        _DEV_STATCLRW { w: self }
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn ccemptyclr(&mut self) -> _CCEMPTYCLRW {
        _CCEMPTYCLRW { w: self }
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn cdfullclr(&mut self) -> _CDFULLCLRW {
        _CDFULLCLRW { w: self }
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn rx_endpktclr(&mut self) -> _RXENDPKTCLRW {
        _RXENDPKTCLRW { w: self }
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn tx_endpktclr(&mut self) -> _TXENDPKTCLRW {
        _TXENDPKTCLRW { w: self }
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn ep_rlzedclr(&mut self) -> _EP_RLZEDCLRW {
        _EP_RLZEDCLRW { w: self }
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is cleared."]
    #[inline]
    pub fn err_intclr(&mut self) -> _ERR_INTCLRW {
        _ERR_INTCLRW { w: self }
    }
}
