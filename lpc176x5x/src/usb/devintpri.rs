#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVINTPRI {
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
#[doc = "Values that can be written to the field `FRAME`"]
pub enum FRAMEW {
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    LP,
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    HP,
}
impl FRAMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAMEW::LP => false,
            FRAMEW::HP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAMEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    #[inline]
    pub fn lp(self) -> &'a mut W {
        self.variant(FRAMEW::LP)
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    #[inline]
    pub fn hp(self) -> &'a mut W {
        self.variant(FRAMEW::HP)
    }
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
#[doc = "Values that can be written to the field `EP_FAST`"]
pub enum EP_FASTW {
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    LP,
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    HP,
}
impl EP_FASTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EP_FASTW::LP => false,
            EP_FASTW::HP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EP_FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_FASTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_FASTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    #[inline]
    pub fn lp(self) -> &'a mut W {
        self.variant(EP_FASTW::LP)
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    #[inline]
    pub fn hp(self) -> &'a mut W {
        self.variant(EP_FASTW::HP)
    }
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
    #[doc = "Bit 0 - Frame interrupt routing"]
    #[inline]
    pub fn frame(&mut self) -> _FRAMEW {
        _FRAMEW { w: self }
    }
    #[doc = "Bit 1 - Fast endpoint interrupt routing"]
    #[inline]
    pub fn ep_fast(&mut self) -> _EP_FASTW {
        _EP_FASTW { w: self }
    }
}
