#[doc = "Writer for register CON_CLR"]
pub type W = crate::W<u32, super::CON_CLR>;
#[doc = "Register CON_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CON_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RUN0_CLR`"]
pub struct RUN0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN0_CLR_W<'a> {
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
#[doc = "Write proxy for field `CENTER0_CLR`"]
pub struct CENTER0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER0_CLR_W<'a> {
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
#[doc = "Write proxy for field `POLA0_CLR`"]
pub struct POLA0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA0_CLR_W<'a> {
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
#[doc = "Write proxy for field `DTE0_CLR`"]
pub struct DTE0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE0_CLR_W<'a> {
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
#[doc = "Write proxy for field `DISUP0_CLR`"]
pub struct DISUP0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP0_CLR_W<'a> {
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
#[doc = "Write proxy for field `RUN1_CLR`"]
pub struct RUN1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN1_CLR_W<'a> {
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
#[doc = "Write proxy for field `CENTER1_CLR`"]
pub struct CENTER1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER1_CLR_W<'a> {
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
#[doc = "Write proxy for field `POLA1_CLR`"]
pub struct POLA1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA1_CLR_W<'a> {
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
#[doc = "Write proxy for field `DTE1_CLR`"]
pub struct DTE1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE1_CLR_W<'a> {
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
#[doc = "Write proxy for field `DISUP1_CLR`"]
pub struct DISUP1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP1_CLR_W<'a> {
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
#[doc = "Write proxy for field `RUN2_CLR`"]
pub struct RUN2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN2_CLR_W<'a> {
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
#[doc = "Write proxy for field `CENTER2_CLR`"]
pub struct CENTER2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER2_CLR_W<'a> {
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
#[doc = "Write proxy for field `POLA2_CLR`"]
pub struct POLA2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA2_CLR_W<'a> {
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
#[doc = "Write proxy for field `DTE2_CLR`"]
pub struct DTE2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE2_CLR_W<'a> {
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
#[doc = "Write proxy for field `DISUP2_CLR`"]
pub struct DISUP2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISUP2_CLR_W<'a> {
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
#[doc = "Write proxy for field `INVBDC_CLR`"]
pub struct INVBDC_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INVBDC_CLR_W<'a> {
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
#[doc = "Write proxy for field `ACMOD_CLR`"]
pub struct ACMOD_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMOD_CLR_W<'a> {
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
#[doc = "Write proxy for field `DCMODE_CLR`"]
pub struct DCMODE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMODE_CLR_W<'a> {
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
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run0_clr(&mut self) -> RUN0_CLR_W {
        RUN0_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center0_clr(&mut self) -> CENTER0_CLR_W {
        CENTER0_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola0_clr(&mut self) -> POLA0_CLR_W {
        POLA0_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte0_clr(&mut self) -> DTE0_CLR_W {
        DTE0_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup0_clr(&mut self) -> DISUP0_CLR_W {
        DISUP0_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run1_clr(&mut self) -> RUN1_CLR_W {
        RUN1_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center1_clr(&mut self) -> CENTER1_CLR_W {
        CENTER1_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola1_clr(&mut self) -> POLA1_CLR_W {
        POLA1_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte1_clr(&mut self) -> DTE1_CLR_W {
        DTE1_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup1_clr(&mut self) -> DISUP1_CLR_W {
        DISUP1_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run2_clr(&mut self) -> RUN2_CLR_W {
        RUN2_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center2_clr(&mut self) -> CENTER2_CLR_W {
        CENTER2_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola2_clr(&mut self) -> POLA2_CLR_W {
        POLA2_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte2_clr(&mut self) -> DTE2_CLR_W {
        DTE2_CLR_W { w: self }
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup2_clr(&mut self) -> DISUP2_CLR_W {
        DISUP2_CLR_W { w: self }
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn invbdc_clr(&mut self) -> INVBDC_CLR_W {
        INVBDC_CLR_W { w: self }
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn acmod_clr(&mut self) -> ACMOD_CLR_W {
        ACMOD_CLR_W { w: self }
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dcmode_clr(&mut self) -> DCMODE_CLR_W {
        DCMODE_CLR_W { w: self }
    }
}
