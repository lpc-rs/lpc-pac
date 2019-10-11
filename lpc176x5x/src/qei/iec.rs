#[doc = "Writer for register IEC"]
pub type W = crate::W<u32, super::IEC>;
#[doc = "Register IEC `reset()`'s with value 0"]
impl crate::ResetValue for super::IEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INX_INT`"]
pub struct INX_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INX_INT_W<'a> {
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
#[doc = "Write proxy for field `TIM_INT`"]
pub struct TIM_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM_INT_W<'a> {
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
#[doc = "Write proxy for field `VELC_INT`"]
pub struct VELC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> VELC_INT_W<'a> {
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
#[doc = "Write proxy for field `DIR_INT`"]
pub struct DIR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_INT_W<'a> {
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
#[doc = "Write proxy for field `ERR_INT`"]
pub struct ERR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_INT_W<'a> {
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
#[doc = "Write proxy for field `ENCLK_INT`"]
pub struct ENCLK_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCLK_INT_W<'a> {
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
#[doc = "Write proxy for field `POS0_INT`"]
pub struct POS0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS0_INT_W<'a> {
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
#[doc = "Write proxy for field `POS1_INT`"]
pub struct POS1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS1_INT_W<'a> {
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
#[doc = "Write proxy for field `POS2_INT`"]
pub struct POS2_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS2_INT_W<'a> {
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
#[doc = "Write proxy for field `REV0_INT`"]
pub struct REV0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV0_INT_W<'a> {
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
#[doc = "Write proxy for field `POS0REV_INT`"]
pub struct POS0REV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS0REV_INT_W<'a> {
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
#[doc = "Write proxy for field `POS1REV_INT`"]
pub struct POS1REV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS1REV_INT_W<'a> {
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
#[doc = "Write proxy for field `POS2REV_INT`"]
pub struct POS2REV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> POS2REV_INT_W<'a> {
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
#[doc = "Write proxy for field `REV1_INT`"]
pub struct REV1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV1_INT_W<'a> {
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
#[doc = "Write proxy for field `REV2_INT`"]
pub struct REV2_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV2_INT_W<'a> {
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
#[doc = "Write proxy for field `MAXPOS_INT`"]
pub struct MAXPOS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXPOS_INT_W<'a> {
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
    #[doc = "Bit 0 - Writing a 1 disables the INX_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn inx_int(&mut self) -> INX_INT_W {
        INX_INT_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 disables the TIN_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn tim_int(&mut self) -> TIM_INT_W {
        TIM_INT_W { w: self }
    }
    #[doc = "Bit 2 - Writing a 1 disables the VELC_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn velc_int(&mut self) -> VELC_INT_W {
        VELC_INT_W { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 disables the DIR_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn dir_int(&mut self) -> DIR_INT_W {
        DIR_INT_W { w: self }
    }
    #[doc = "Bit 4 - Writing a 1 disables the ERR_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn err_int(&mut self) -> ERR_INT_W {
        ERR_INT_W { w: self }
    }
    #[doc = "Bit 5 - Writing a 1 disables the ENCLK_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn enclk_int(&mut self) -> ENCLK_INT_W {
        ENCLK_INT_W { w: self }
    }
    #[doc = "Bit 6 - Writing a 1 disables the POS0_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos0_int(&mut self) -> POS0_INT_W {
        POS0_INT_W { w: self }
    }
    #[doc = "Bit 7 - Writing a 1 disables the POS1_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos1_int(&mut self) -> POS1_INT_W {
        POS1_INT_W { w: self }
    }
    #[doc = "Bit 8 - Writing a 1 disables the POS2_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos2_int(&mut self) -> POS2_INT_W {
        POS2_INT_W { w: self }
    }
    #[doc = "Bit 9 - Writing a 1 disables the REV0_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn rev0_int(&mut self) -> REV0_INT_W {
        REV0_INT_W { w: self }
    }
    #[doc = "Bit 10 - Writing a 1 disables the POS0REV_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos0rev_int(&mut self) -> POS0REV_INT_W {
        POS0REV_INT_W { w: self }
    }
    #[doc = "Bit 11 - Writing a 1 disables the POS1REV_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos1rev_int(&mut self) -> POS1REV_INT_W {
        POS1REV_INT_W { w: self }
    }
    #[doc = "Bit 12 - Writing a 1 disables the POS2REV_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos2rev_int(&mut self) -> POS2REV_INT_W {
        POS2REV_INT_W { w: self }
    }
    #[doc = "Bit 13 - Writing a 1 disables the REV1_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn rev1_int(&mut self) -> REV1_INT_W {
        REV1_INT_W { w: self }
    }
    #[doc = "Bit 14 - Writing a 1 disables the REV2_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn rev2_int(&mut self) -> REV2_INT_W {
        REV2_INT_W { w: self }
    }
    #[doc = "Bit 15 - Writing a 1 disables the MAXPOS_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn maxpos_int(&mut self) -> MAXPOS_INT_W {
        MAXPOS_INT_W { w: self }
    }
}
