#[doc = "Writer for register INTEN_SET"]
pub type W = crate::W<u32, super::INTEN_SET>;
#[doc = "Register INTEN_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ILIM0_SET`"]
pub struct ILIM0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM0_SET_W<'a> {
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
#[doc = "Write proxy for field `IMAT0_SET`"]
pub struct IMAT0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT0_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `ICAP0_SET`"]
pub struct ICAP0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP0_SET_W<'a> {
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
#[doc = "Write proxy for field `ILIM1_SET`"]
pub struct ILIM1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM1_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `IMAT1_SET`"]
pub struct IMAT1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT1_SET_W<'a> {
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
#[doc = "Write proxy for field `ICAP1_SET`"]
pub struct ICAP1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP1_SET_W<'a> {
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
#[doc = "Write proxy for field `ILIM2_SET`"]
pub struct ILIM2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM2_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `IMAT2_SET`"]
pub struct IMAT2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT2_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `ICAP2_SET`"]
pub struct ICAP2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP2_SET_W<'a> {
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
#[doc = "Write proxy for field `ABORT_SET`"]
pub struct ABORT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_SET_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn ilim0_set(&mut self) -> ILIM0_SET_W {
        ILIM0_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn imat0_set(&mut self) -> IMAT0_SET_W {
        IMAT0_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn icap0_set(&mut self) -> ICAP0_SET_W {
        ICAP0_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn ilim1_set(&mut self) -> ILIM1_SET_W {
        ILIM1_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn imat1_set(&mut self) -> IMAT1_SET_W {
        IMAT1_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn icap1_set(&mut self) -> ICAP1_SET_W {
        ICAP1_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn ilim2_set(&mut self) -> ILIM2_SET_W {
        ILIM2_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn imat2_set(&mut self) -> IMAT2_SET_W {
        IMAT2_SET_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn icap2_set(&mut self) -> ICAP2_SET_W {
        ICAP2_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    pub fn abort_set(&mut self) -> ABORT_SET_W {
        ABORT_SET_W { w: self }
    }
}
