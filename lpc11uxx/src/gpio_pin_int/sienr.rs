#[doc = "Writer for register SIENR"]
pub type W = crate::W<u32, super::SIENR>;
#[doc = "Register SIENR `reset()`'s with value 0"]
impl crate::ResetValue for super::SIENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SETENRL0`"]
pub struct SETENRL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL0_W<'a> {
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
#[doc = "Write proxy for field `SETENRL1`"]
pub struct SETENRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL1_W<'a> {
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
#[doc = "Write proxy for field `SETENRL2`"]
pub struct SETENRL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL2_W<'a> {
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
#[doc = "Write proxy for field `SETENRL3`"]
pub struct SETENRL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL3_W<'a> {
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
#[doc = "Write proxy for field `SETENRL4`"]
pub struct SETENRL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL4_W<'a> {
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
#[doc = "Write proxy for field `SETENRL5`"]
pub struct SETENRL5_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL5_W<'a> {
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
#[doc = "Write proxy for field `SETENRL6`"]
pub struct SETENRL6_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL6_W<'a> {
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
#[doc = "Write proxy for field `SETENRL7`"]
pub struct SETENRL7_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENRL7_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl0(&mut self) -> SETENRL0_W {
        SETENRL0_W { w: self }
    }
    #[doc = "Bit 1 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl1(&mut self) -> SETENRL1_W {
        SETENRL1_W { w: self }
    }
    #[doc = "Bit 2 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl2(&mut self) -> SETENRL2_W {
        SETENRL2_W { w: self }
    }
    #[doc = "Bit 3 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl3(&mut self) -> SETENRL3_W {
        SETENRL3_W { w: self }
    }
    #[doc = "Bit 4 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl4(&mut self) -> SETENRL4_W {
        SETENRL4_W { w: self }
    }
    #[doc = "Bit 5 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl5(&mut self) -> SETENRL5_W {
        SETENRL5_W { w: self }
    }
    #[doc = "Bit 6 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl6(&mut self) -> SETENRL6_W {
        SETENRL6_W { w: self }
    }
    #[doc = "Bit 7 - Ones written to this address set bits in the PINTEN_R, thus enabling interrupts. Bit n sets bit n in the PINTEN_R register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn setenrl7(&mut self) -> SETENRL7_W {
        SETENRL7_W { w: self }
    }
}
