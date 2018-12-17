#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IEC {
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
pub struct _INX_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INX_INTW<'a> {
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
pub struct _TIM_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM_INTW<'a> {
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
pub struct _VELC_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _VELC_INTW<'a> {
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
pub struct _DIR_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIR_INTW<'a> {
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
pub struct _ERR_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERR_INTW<'a> {
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
pub struct _ENCLK_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCLK_INTW<'a> {
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
pub struct _POS0_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS0_INTW<'a> {
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
pub struct _POS1_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS1_INTW<'a> {
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
pub struct _POS2_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS2_INTW<'a> {
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
pub struct _REV0_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _REV0_INTW<'a> {
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
pub struct _POS0REV_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS0REV_INTW<'a> {
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
pub struct _POS1REV_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS1REV_INTW<'a> {
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
pub struct _POS2REV_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _POS2REV_INTW<'a> {
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
pub struct _REV1_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _REV1_INTW<'a> {
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
pub struct _REV2_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _REV2_INTW<'a> {
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
pub struct _MAXPOS_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXPOS_INTW<'a> {
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
    #[doc = "Bit 0 - Writing a 1 disables the INX_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn inx_int(&mut self) -> _INX_INTW {
        _INX_INTW { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 disables the TIN_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn tim_int(&mut self) -> _TIM_INTW {
        _TIM_INTW { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 disables the VELC_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn velc_int(&mut self) -> _VELC_INTW {
        _VELC_INTW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 disables the DIR_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn dir_int(&mut self) -> _DIR_INTW {
        _DIR_INTW { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 disables the ERR_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn err_int(&mut self) -> _ERR_INTW {
        _ERR_INTW { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 disables the ENCLK_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn enclk_int(&mut self) -> _ENCLK_INTW {
        _ENCLK_INTW { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 disables the POS0_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn pos0_int(&mut self) -> _POS0_INTW {
        _POS0_INTW { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 disables the POS1_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn pos1_int(&mut self) -> _POS1_INTW {
        _POS1_INTW { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 disables the POS2_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn pos2_int(&mut self) -> _POS2_INTW {
        _POS2_INTW { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 disables the REV0_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn rev0_int(&mut self) -> _REV0_INTW {
        _REV0_INTW { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 disables the POS0REV_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn pos0rev_int(&mut self) -> _POS0REV_INTW {
        _POS0REV_INTW { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 disables the POS1REV_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn pos1rev_int(&mut self) -> _POS1REV_INTW {
        _POS1REV_INTW { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 disables the POS2REV_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn pos2rev_int(&mut self) -> _POS2REV_INTW {
        _POS2REV_INTW { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 disables the REV1_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn rev1_int(&mut self) -> _REV1_INTW {
        _REV1_INTW { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 disables the REV2_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn rev2_int(&mut self) -> _REV2_INTW {
        _REV2_INTW { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 disables the MAXPOS_Int interrupt in the QEIIE register."]
    #[inline]
    pub fn maxpos_int(&mut self) -> _MAXPOS_INTW {
        _MAXPOS_INTW { w: self }
    }
}
