#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR {
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Proxy"]
pub struct _RXRDYCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDYCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TXRDYCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDYCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TXIDLECLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIDLECLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _DELTACTSCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DELTACTSCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TXDISINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISINTCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _OVERRUNCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUNCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _DELTARXBRKCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DELTARXBRKCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _STARTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _FRAMERRCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMERRCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PARITYERRCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYERRCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RXNOISECLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNOISECLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _ABERRCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABERRCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn rxrdyclr(&mut self) -> _RXRDYCLRW {
        _RXRDYCLRW { w: self }
    }
    #[doc = "Bit 2 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txrdyclr(&mut self) -> _TXRDYCLRW {
        _TXRDYCLRW { w: self }
    }
    #[doc = "Bit 3 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txidleclr(&mut self) -> _TXIDLECLRW {
        _TXIDLECLRW { w: self }
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltactsclr(&mut self) -> _DELTACTSCLRW {
        _DELTACTSCLRW { w: self }
    }
    #[doc = "Bit 6 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn txdisintclr(&mut self) -> _TXDISINTCLRW {
        _TXDISINTCLRW { w: self }
    }
    #[doc = "Bit 8 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn overrunclr(&mut self) -> _OVERRUNCLRW {
        _OVERRUNCLRW { w: self }
    }
    #[doc = "Bit 11 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn deltarxbrkclr(&mut self) -> _DELTARXBRKCLRW {
        _DELTARXBRKCLRW { w: self }
    }
    #[doc = "Bit 12 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn startclr(&mut self) -> _STARTCLRW {
        _STARTCLRW { w: self }
    }
    #[doc = "Bit 13 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn framerrclr(&mut self) -> _FRAMERRCLRW {
        _FRAMERRCLRW { w: self }
    }
    #[doc = "Bit 14 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn parityerrclr(&mut self) -> _PARITYERRCLRW {
        _PARITYERRCLRW { w: self }
    }
    #[doc = "Bit 15 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn rxnoiseclr(&mut self) -> _RXNOISECLRW {
        _RXNOISECLRW { w: self }
    }
    #[doc = "Bit 16 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn aberrclr(&mut self) -> _ABERRCLRW {
        _ABERRCLRW { w: self }
    }
}
