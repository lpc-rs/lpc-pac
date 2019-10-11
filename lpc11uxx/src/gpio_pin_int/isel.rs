#[doc = "Reader of register ISEL"]
pub type R = crate::R<u32, super::ISEL>;
#[doc = "Writer for register ISEL"]
pub type W = crate::W<u32, super::ISEL>;
#[doc = "Register ISEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ISEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMODE0`"]
pub type PMODE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE0`"]
pub struct PMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE0_W<'a> {
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
#[doc = "Reader of field `PMODE1`"]
pub type PMODE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE1`"]
pub struct PMODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE1_W<'a> {
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
#[doc = "Reader of field `PMODE2`"]
pub type PMODE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE2`"]
pub struct PMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE2_W<'a> {
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
#[doc = "Reader of field `PMODE3`"]
pub type PMODE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE3`"]
pub struct PMODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE3_W<'a> {
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
#[doc = "Reader of field `PMODE4`"]
pub type PMODE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE4`"]
pub struct PMODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE4_W<'a> {
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
#[doc = "Reader of field `PMODE5`"]
pub type PMODE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE5`"]
pub struct PMODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE5_W<'a> {
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
#[doc = "Reader of field `PMODE6`"]
pub type PMODE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE6`"]
pub struct PMODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE6_W<'a> {
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
#[doc = "Reader of field `PMODE7`"]
pub type PMODE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMODE7`"]
pub struct PMODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE7_W<'a> {
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
    #[doc = "Bit 0 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode0(&self) -> PMODE0_R {
        PMODE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode1(&self) -> PMODE1_R {
        PMODE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode2(&self) -> PMODE2_R {
        PMODE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode3(&self) -> PMODE3_R {
        PMODE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode4(&self) -> PMODE4_R {
        PMODE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode5(&self) -> PMODE5_R {
        PMODE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode6(&self) -> PMODE6_R {
        PMODE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode7(&self) -> PMODE7_R {
        PMODE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode0(&mut self) -> PMODE0_W {
        PMODE0_W { w: self }
    }
    #[doc = "Bit 1 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode1(&mut self) -> PMODE1_W {
        PMODE1_W { w: self }
    }
    #[doc = "Bit 2 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode2(&mut self) -> PMODE2_W {
        PMODE2_W { w: self }
    }
    #[doc = "Bit 3 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode3(&mut self) -> PMODE3_W {
        PMODE3_W { w: self }
    }
    #[doc = "Bit 4 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode4(&mut self) -> PMODE4_W {
        PMODE4_W { w: self }
    }
    #[doc = "Bit 5 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode5(&mut self) -> PMODE5_W {
        PMODE5_W { w: self }
    }
    #[doc = "Bit 6 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode6(&mut self) -> PMODE6_W {
        PMODE6_W { w: self }
    }
    #[doc = "Bit 7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode7(&mut self) -> PMODE7_W {
        PMODE7_W { w: self }
    }
}
