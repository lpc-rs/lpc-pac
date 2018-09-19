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
pub struct _CLRPW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:28 - Clear output bits (bit 0 = PIO0_0, bit 1 =PIO0_1, ..., bit 28 = PIO0_28). 0 = No operation. 1 = Clear output bit."]
    #[inline]
    pub fn clrp(&mut self) -> _CLRPW {
        _CLRPW { w: self }
    }
}
