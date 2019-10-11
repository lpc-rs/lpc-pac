#[doc = "Reader of register IENR"]
pub type R = crate::R<u32, super::IENR>;
#[doc = "Writer for register IENR"]
pub type W = crate::W<u32, super::IENR>;
#[doc = "Register IENR `reset()`'s with value 0"]
impl crate::ResetValue for super::IENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENRL0`"]
pub type ENRL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL0`"]
pub struct ENRL0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL0_W<'a> {
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
#[doc = "Reader of field `ENRL1`"]
pub type ENRL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL1`"]
pub struct ENRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL1_W<'a> {
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
#[doc = "Reader of field `ENRL2`"]
pub type ENRL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL2`"]
pub struct ENRL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL2_W<'a> {
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
#[doc = "Reader of field `ENRL3`"]
pub type ENRL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL3`"]
pub struct ENRL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL3_W<'a> {
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
#[doc = "Reader of field `ENRL4`"]
pub type ENRL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL4`"]
pub struct ENRL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL4_W<'a> {
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
#[doc = "Reader of field `ENRL5`"]
pub type ENRL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL5`"]
pub struct ENRL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL5_W<'a> {
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
#[doc = "Reader of field `ENRL6`"]
pub type ENRL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL6`"]
pub struct ENRL6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL6_W<'a> {
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
#[doc = "Reader of field `ENRL7`"]
pub type ENRL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRL7`"]
pub struct ENRL7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl0(&self) -> ENRL0_R {
        ENRL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl1(&self) -> ENRL1_R {
        ENRL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl2(&self) -> ENRL2_R {
        ENRL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl3(&self) -> ENRL3_R {
        ENRL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl4(&self) -> ENRL4_R {
        ENRL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl5(&self) -> ENRL5_R {
        ENRL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl6(&self) -> ENRL6_R {
        ENRL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl7(&self) -> ENRL7_R {
        ENRL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl0(&mut self) -> ENRL0_W {
        ENRL0_W { w: self }
    }
    #[doc = "Bit 1 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl1(&mut self) -> ENRL1_W {
        ENRL1_W { w: self }
    }
    #[doc = "Bit 2 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl2(&mut self) -> ENRL2_W {
        ENRL2_W { w: self }
    }
    #[doc = "Bit 3 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl3(&mut self) -> ENRL3_W {
        ENRL3_W { w: self }
    }
    #[doc = "Bit 4 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl4(&mut self) -> ENRL4_W {
        ENRL4_W { w: self }
    }
    #[doc = "Bit 5 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl5(&mut self) -> ENRL5_W {
        ENRL5_W { w: self }
    }
    #[doc = "Bit 6 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl6(&mut self) -> ENRL6_W {
        ENRL6_W { w: self }
    }
    #[doc = "Bit 7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl7(&mut self) -> ENRL7_W {
        ENRL7_W { w: self }
    }
}
