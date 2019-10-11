#[doc = "Writer for register INTF_SET"]
pub type W = crate::W<u32, super::INTF_SET>;
#[doc = "Register INTF_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTF_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ILIM0_F_SET`"]
pub struct ILIM0_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM0_F_SET_W<'a> {
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
#[doc = "Write proxy for field `IMAT0_F_SET`"]
pub struct IMAT0_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT0_F_SET_W<'a> {
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
#[doc = "Write proxy for field `ICAP0_F_SET`"]
pub struct ICAP0_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP0_F_SET_W<'a> {
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
#[doc = "Write proxy for field `ILIM1_F_SET`"]
pub struct ILIM1_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM1_F_SET_W<'a> {
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
#[doc = "Write proxy for field `IMAT1_F_SET`"]
pub struct IMAT1_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT1_F_SET_W<'a> {
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
#[doc = "Write proxy for field `ICAP1_F_SET`"]
pub struct ICAP1_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP1_F_SET_W<'a> {
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
#[doc = "Write proxy for field `ILIM2_F_SET`"]
pub struct ILIM2_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIM2_F_SET_W<'a> {
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
#[doc = "Write proxy for field `IMAT2_F_SET`"]
pub struct IMAT2_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAT2_F_SET_W<'a> {
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
#[doc = "Write proxy for field `ICAP2_F_SET`"]
pub struct ICAP2_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ICAP2_F_SET_W<'a> {
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
#[doc = "Write proxy for field `ABORT_F_SET`"]
pub struct ABORT_F_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_F_SET_W<'a> {
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
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn ilim0_f_set(&mut self) -> ILIM0_F_SET_W {
        ILIM0_F_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn imat0_f_set(&mut self) -> IMAT0_F_SET_W {
        IMAT0_F_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn icap0_f_set(&mut self) -> ICAP0_F_SET_W {
        ICAP0_F_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn ilim1_f_set(&mut self) -> ILIM1_F_SET_W {
        ILIM1_F_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn imat1_f_set(&mut self) -> IMAT1_F_SET_W {
        IMAT1_F_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn icap1_f_set(&mut self) -> ICAP1_F_SET_W {
        ICAP1_F_SET_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn ilim2_f_set(&mut self) -> ILIM2_F_SET_W {
        ILIM2_F_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn imat2_f_set(&mut self) -> IMAT2_F_SET_W {
        IMAT2_F_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn icap2_f_set(&mut self) -> ICAP2_F_SET_W {
        ICAP2_F_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    pub fn abort_f_set(&mut self) -> ABORT_F_SET_W {
        ABORT_F_SET_W { w: self }
    }
}
