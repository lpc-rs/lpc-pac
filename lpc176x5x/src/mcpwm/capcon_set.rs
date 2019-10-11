#[doc = "Writer for register CAPCON_SET"]
pub type W = crate::W<u32, super::CAPCON_SET>;
#[doc = "Register CAPCON_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPCON_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CAP0MCI0_RE_SET`"]
pub struct CAP0MCI0_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI0_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP0MCI0_FE_SET`"]
pub struct CAP0MCI0_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI0_FE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP0MCI1_RE_SET`"]
pub struct CAP0MCI1_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI1_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP0MCI1_FE_SET`"]
pub struct CAP0MCI1_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI1_FE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP0MCI2_RE_SET`"]
pub struct CAP0MCI2_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI2_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP0MCI2_FE_SET`"]
pub struct CAP0MCI2_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0MCI2_FE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP1MCI0_RE_SET`"]
pub struct CAP1MCI0_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI0_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP1MCI0_FE_SET`"]
pub struct CAP1MCI0_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI0_FE_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `CAP1MCI1_RE_SET`"]
pub struct CAP1MCI1_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI1_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP1MCI1_FE_SET`"]
pub struct CAP1MCI1_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI1_FE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP1MCI2_RE_SET`"]
pub struct CAP1MCI2_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI2_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP1MCI2_FE_SET`"]
pub struct CAP1MCI2_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1MCI2_FE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP2MCI0_RE_SET`"]
pub struct CAP2MCI0_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI0_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP2MCI0_FE_SET`"]
pub struct CAP2MCI0_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI0_FE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP2MCI1_RE_SET`"]
pub struct CAP2MCI1_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI1_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP2MCI1_FE_SET`"]
pub struct CAP2MCI1_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI1_FE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP2MCI2_RE_SET`"]
pub struct CAP2MCI2_RE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI2_RE_SET_W<'a> {
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
#[doc = "Write proxy for field `CAP2MCI2_FE_SET`"]
pub struct CAP2MCI2_FE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2MCI2_FE_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `RT0_SET`"]
pub struct RT0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `RT1_SET`"]
pub struct RT1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `RT2_SET`"]
pub struct RT2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RT2_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_re_set(&mut self) -> CAP0MCI0_RE_SET_W {
        CAP0MCI0_RE_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_fe_set(&mut self) -> CAP0MCI0_FE_SET_W {
        CAP0MCI0_FE_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_re_set(&mut self) -> CAP0MCI1_RE_SET_W {
        CAP0MCI1_RE_SET_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_fe_set(&mut self) -> CAP0MCI1_FE_SET_W {
        CAP0MCI1_FE_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_re_set(&mut self) -> CAP0MCI2_RE_SET_W {
        CAP0MCI2_RE_SET_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_fe_set(&mut self) -> CAP0MCI2_FE_SET_W {
        CAP0MCI2_FE_SET_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_re_set(&mut self) -> CAP1MCI0_RE_SET_W {
        CAP1MCI0_RE_SET_W { w: self }
    }
    #[doc = "Bit 7 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_fe_set(&mut self) -> CAP1MCI0_FE_SET_W {
        CAP1MCI0_FE_SET_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_re_set(&mut self) -> CAP1MCI1_RE_SET_W {
        CAP1MCI1_RE_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_fe_set(&mut self) -> CAP1MCI1_FE_SET_W {
        CAP1MCI1_FE_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_re_set(&mut self) -> CAP1MCI2_RE_SET_W {
        CAP1MCI2_RE_SET_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_fe_set(&mut self) -> CAP1MCI2_FE_SET_W {
        CAP1MCI2_FE_SET_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_re_set(&mut self) -> CAP2MCI0_RE_SET_W {
        CAP2MCI0_RE_SET_W { w: self }
    }
    #[doc = "Bit 13 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_fe_set(&mut self) -> CAP2MCI0_FE_SET_W {
        CAP2MCI0_FE_SET_W { w: self }
    }
    #[doc = "Bit 14 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_re_set(&mut self) -> CAP2MCI1_RE_SET_W {
        CAP2MCI1_RE_SET_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_fe_set(&mut self) -> CAP2MCI1_FE_SET_W {
        CAP2MCI1_FE_SET_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_re_set(&mut self) -> CAP2MCI2_RE_SET_W {
        CAP2MCI2_RE_SET_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_fe_set(&mut self) -> CAP2MCI2_FE_SET_W {
        CAP2MCI2_FE_SET_W { w: self }
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt0_set(&mut self) -> RT0_SET_W {
        RT0_SET_W { w: self }
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt1_set(&mut self) -> RT1_SET_W {
        RT1_SET_W { w: self }
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt2_set(&mut self) -> RT2_SET_W {
        RT2_SET_W { w: self }
    }
}
