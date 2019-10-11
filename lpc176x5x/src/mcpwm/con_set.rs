#[doc = "Writer for register CON_SET"]
pub type W = crate::W<u32, super::CON_SET>;
#[doc = "Register CON_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CON_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RUN0_SET`"]
pub struct RUN0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN0_SET_W<'a> {
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
#[doc = "Write proxy for field `CENTER0_SET`"]
pub struct CENTER0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER0_SET_W<'a> {
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
#[doc = "Write proxy for field `POLA0_SET`"]
pub struct POLA0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA0_SET_W<'a> {
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
#[doc = "Write proxy for field `DTE0_SET`"]
pub struct DTE0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE0_SET_W<'a> {
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
#[doc = "Write proxy for field `DISUP0_SET`"]
pub struct DISUP0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP0_SET_W<'a> {
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
#[doc = "Write proxy for field `RUN1_SET`"]
pub struct RUN1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN1_SET_W<'a> {
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
#[doc = "Write proxy for field `CENTER1_SET`"]
pub struct CENTER1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER1_SET_W<'a> {
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
#[doc = "Write proxy for field `POLA1_SET`"]
pub struct POLA1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA1_SET_W<'a> {
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
#[doc = "Write proxy for field `DTE1_SET`"]
pub struct DTE1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE1_SET_W<'a> {
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
#[doc = "Write proxy for field `DISUP1_SET`"]
pub struct DISUP1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP1_SET_W<'a> {
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
#[doc = "Write proxy for field `RUN2_SET`"]
pub struct RUN2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN2_SET_W<'a> {
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
#[doc = "Write proxy for field `CENTER2_SET`"]
pub struct CENTER2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER2_SET_W<'a> {
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
#[doc = "Write proxy for field `POLA2_SET`"]
pub struct POLA2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA2_SET_W<'a> {
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
#[doc = "Write proxy for field `DTE2_SET`"]
pub struct DTE2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE2_SET_W<'a> {
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
#[doc = "Write proxy for field `DISUP2_SET`"]
pub struct DISUP2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP2_SET_W<'a> {
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
#[doc = "Write proxy for field `INVBDC_SET`"]
pub struct INVBDC_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> INVBDC_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Write proxy for field `ACMODE_SET`"]
pub struct ACMODE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMODE_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `DCMODE_SET`"]
pub struct DCMODE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMODE_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run0_set(&mut self) -> RUN0_SET_W {
        RUN0_SET_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center0_set(&mut self) -> CENTER0_SET_W {
        CENTER0_SET_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola0_set(&mut self) -> POLA0_SET_W {
        POLA0_SET_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte0_set(&mut self) -> DTE0_SET_W {
        DTE0_SET_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup0_set(&mut self) -> DISUP0_SET_W {
        DISUP0_SET_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run1_set(&mut self) -> RUN1_SET_W {
        RUN1_SET_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center1_set(&mut self) -> CENTER1_SET_W {
        CENTER1_SET_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola1_set(&mut self) -> POLA1_SET_W {
        POLA1_SET_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte1_set(&mut self) -> DTE1_SET_W {
        DTE1_SET_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup1_set(&mut self) -> DISUP1_SET_W {
        DISUP1_SET_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run2_set(&mut self) -> RUN2_SET_W {
        RUN2_SET_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center2_set(&mut self) -> CENTER2_SET_W {
        CENTER2_SET_W { w: self }
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola2_set(&mut self) -> POLA2_SET_W {
        POLA2_SET_W { w: self }
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte2_set(&mut self) -> DTE2_SET_W {
        DTE2_SET_W { w: self }
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup2_set(&mut self) -> DISUP2_SET_W {
        DISUP2_SET_W { w: self }
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn invbdc_set(&mut self) -> INVBDC_SET_W {
        INVBDC_SET_W { w: self }
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn acmode_set(&mut self) -> ACMODE_SET_W {
        ACMODE_SET_W { w: self }
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dcmode_set(&mut self) -> DCMODE_SET_W {
        DCMODE_SET_W { w: self }
    }
}
