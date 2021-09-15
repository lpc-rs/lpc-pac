#[doc = "Register `CONCLR` writer"]
pub struct W(crate::W<CONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AAC` writer - Assert acknowledge Clear bit."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SIC` writer - I2C interrupt Clear bit."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `STAC` writer - START flag Clear bit."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `I2ENC` writer - I2C interface Disable bit."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Clear Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is cleared. Writing a zero has no effect on the corresponding bit in the I2C control register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conclr](index.html) module"]
pub struct CONCLR_SPEC;
impl crate::RegisterSpec for CONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [conclr::W](W) writer structure"]
impl crate::Writable for CONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONCLR to value 0"]
impl crate::Resettable for CONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
