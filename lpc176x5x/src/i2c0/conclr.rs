#[doc = "Writer for register CONCLR"]
pub type W = crate::W<u32, super::CONCLR>;
#[doc = "Register CONCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CONCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AAC`"]
pub struct AAC_W<'a> {
    w: &'a mut W,
}
impl<'a> AAC_W<'a> {
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
#[doc = "Write proxy for field `SIC`"]
pub struct SIC_W<'a> {
    w: &'a mut W,
}
impl<'a> SIC_W<'a> {
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
#[doc = "Write proxy for field `STAC`"]
pub struct STAC_W<'a> {
    w: &'a mut W,
}
impl<'a> STAC_W<'a> {
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
#[doc = "Write proxy for field `I2ENC`"]
pub struct I2ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2ENC_W<'a> {
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
impl W {
    #[doc = "Bit 2 - Assert acknowledge Clear bit."]
    #[inline(always)]
    pub fn aac(&mut self) -> AAC_W {
        AAC_W { w: self }
    }
    #[doc = "Bit 3 - I2C interrupt Clear bit."]
    #[inline(always)]
    pub fn sic(&mut self) -> SIC_W {
        SIC_W { w: self }
    }
    #[doc = "Bit 5 - START flag Clear bit."]
    #[inline(always)]
    pub fn stac(&mut self) -> STAC_W {
        STAC_W { w: self }
    }
    #[doc = "Bit 6 - I2C interface Disable bit."]
    #[inline(always)]
    pub fn i2enc(&mut self) -> I2ENC_W {
        I2ENC_W { w: self }
    }
}
