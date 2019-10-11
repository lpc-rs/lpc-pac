#[doc = "Reader of register CONSET"]
pub type R = crate::R<u32, super::CONSET>;
#[doc = "Writer for register CONSET"]
pub type W = crate::W<u32, super::CONSET>;
#[doc = "Register CONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AA`"]
pub type AA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AA`"]
pub struct AA_W<'a> {
    w: &'a mut W,
}
impl<'a> AA_W<'a> {
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
#[doc = "Reader of field `SI`"]
pub type SI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SI`"]
pub struct SI_W<'a> {
    w: &'a mut W,
}
impl<'a> SI_W<'a> {
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
#[doc = "Reader of field `STO`"]
pub type STO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STO`"]
pub struct STO_W<'a> {
    w: &'a mut W,
}
impl<'a> STO_W<'a> {
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
#[doc = "Reader of field `STA`"]
pub type STA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STA`"]
pub struct STA_W<'a> {
    w: &'a mut W,
}
impl<'a> STA_W<'a> {
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
#[doc = "Reader of field `I2EN`"]
pub type I2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2EN`"]
pub struct I2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2EN_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Assert acknowledge flag."]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C interrupt flag."]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C interface enable."]
    #[inline(always)]
    pub fn i2en(&self) -> I2EN_R {
        I2EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Assert acknowledge flag."]
    #[inline(always)]
    pub fn aa(&mut self) -> AA_W {
        AA_W { w: self }
    }
    #[doc = "Bit 3 - I2C interrupt flag."]
    #[inline(always)]
    pub fn si(&mut self) -> SI_W {
        SI_W { w: self }
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W {
        STO_W { w: self }
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W {
        STA_W { w: self }
    }
    #[doc = "Bit 6 - I2C interface enable."]
    #[inline(always)]
    pub fn i2en(&mut self) -> I2EN_W {
        I2EN_W { w: self }
    }
}
