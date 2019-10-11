#[doc = "Writer for register CNTCON_CLR"]
pub type W = crate::W<u32, super::CNTCON_CLR>;
#[doc = "Register CNTCON_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTCON_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TC0MCI0_RE_CLR`"]
pub struct TC0MCI0_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI0_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC0MCI0_FE_CLR`"]
pub struct TC0MCI0_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI0_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC0MCI1_RE_CLR`"]
pub struct TC0MCI1_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI1_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC0MCI1_FE_CLR`"]
pub struct TC0MCI1_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI1_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC0MCI2_RE`"]
pub struct TC0MCI2_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI2_RE_W<'a> {
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
#[doc = "Write proxy for field `TC0MCI2_FE_CLR`"]
pub struct TC0MCI2_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0MCI2_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC1MCI0_RE_CLR`"]
pub struct TC1MCI0_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI0_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC1MCI0_FE_CLR`"]
pub struct TC1MCI0_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI0_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC1MCI1_RE_CLR`"]
pub struct TC1MCI1_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI1_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC1MCI1_FE_CLR`"]
pub struct TC1MCI1_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI1_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC1MCI2_RE_CLR`"]
pub struct TC1MCI2_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI2_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC1MCI2_FE_CLR`"]
pub struct TC1MCI2_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1MCI2_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC2MCI0_RE_CLR`"]
pub struct TC2MCI0_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI0_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC2MCI0_FE_CLR`"]
pub struct TC2MCI0_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI0_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC2MCI1_RE_CLR`"]
pub struct TC2MCI1_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI1_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC2MCI1_FE_CLR`"]
pub struct TC2MCI1_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI1_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC2MCI2_RE_CLR`"]
pub struct TC2MCI2_RE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI2_RE_CLR_W<'a> {
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
#[doc = "Write proxy for field `TC2MCI2_FE_CLR`"]
pub struct TC2MCI2_FE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2MCI2_FE_CLR_W<'a> {
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
#[doc = "Write proxy for field `CNTR0_CLR`"]
pub struct CNTR0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTR0_CLR_W<'a> {
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
#[doc = "Write proxy for field `CNTR1_CLR`"]
pub struct CNTR1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTR1_CLR_W<'a> {
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
#[doc = "Write proxy for field `CNTR2_CLR`"]
pub struct CNTR2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTR2_CLR_W<'a> {
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
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_re_clr(&mut self) -> TC0MCI0_RE_CLR_W {
        TC0MCI0_RE_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_fe_clr(&mut self) -> TC0MCI0_FE_CLR_W {
        TC0MCI0_FE_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_re_clr(&mut self) -> TC0MCI1_RE_CLR_W {
        TC0MCI1_RE_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_fe_clr(&mut self) -> TC0MCI1_FE_CLR_W {
        TC0MCI1_FE_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_re(&mut self) -> TC0MCI2_RE_W {
        TC0MCI2_RE_W { w: self }
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_fe_clr(&mut self) -> TC0MCI2_FE_CLR_W {
        TC0MCI2_FE_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_re_clr(&mut self) -> TC1MCI0_RE_CLR_W {
        TC1MCI0_RE_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_fe_clr(&mut self) -> TC1MCI0_FE_CLR_W {
        TC1MCI0_FE_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_re_clr(&mut self) -> TC1MCI1_RE_CLR_W {
        TC1MCI1_RE_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_fe_clr(&mut self) -> TC1MCI1_FE_CLR_W {
        TC1MCI1_FE_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_re_clr(&mut self) -> TC1MCI2_RE_CLR_W {
        TC1MCI2_RE_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_fe_clr(&mut self) -> TC1MCI2_FE_CLR_W {
        TC1MCI2_FE_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_re_clr(&mut self) -> TC2MCI0_RE_CLR_W {
        TC2MCI0_RE_CLR_W { w: self }
    }
    #[doc = "Bit 13 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_fe_clr(&mut self) -> TC2MCI0_FE_CLR_W {
        TC2MCI0_FE_CLR_W { w: self }
    }
    #[doc = "Bit 14 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_re_clr(&mut self) -> TC2MCI1_RE_CLR_W {
        TC2MCI1_RE_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_fe_clr(&mut self) -> TC2MCI1_FE_CLR_W {
        TC2MCI1_FE_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_re_clr(&mut self) -> TC2MCI2_RE_CLR_W {
        TC2MCI2_RE_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_fe_clr(&mut self) -> TC2MCI2_FE_CLR_W {
        TC2MCI2_FE_CLR_W { w: self }
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr0_clr(&mut self) -> CNTR0_CLR_W {
        CNTR0_CLR_W { w: self }
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr1_clr(&mut self) -> CNTR1_CLR_W {
        CNTR1_CLR_W { w: self }
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr2_clr(&mut self) -> CNTR2_CLR_W {
        CNTR2_CLR_W { w: self }
    }
}
