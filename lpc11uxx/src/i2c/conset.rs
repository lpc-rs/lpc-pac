#[doc = "Register `CONSET` reader"]
pub struct R(crate::R<CONSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSET` writer"]
pub struct W(crate::W<CONSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSET_SPEC>;
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
impl From<crate::W<CONSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AA` reader - Assert acknowledge flag."]
pub struct AA_R(crate::FieldReader<bool, bool>);
impl AA_R {
    pub(crate) fn new(bits: bool) -> Self {
        AA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AA` writer - Assert acknowledge flag."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SI` reader - I2C interrupt flag."]
pub struct SI_R(crate::FieldReader<bool, bool>);
impl SI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SI` writer - I2C interrupt flag."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `STO` reader - STOP flag."]
pub struct STO_R(crate::FieldReader<bool, bool>);
impl STO_R {
    pub(crate) fn new(bits: bool) -> Self {
        STO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STO` writer - STOP flag."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `STA` reader - START flag."]
pub struct STA_R(crate::FieldReader<bool, bool>);
impl STA_R {
    pub(crate) fn new(bits: bool) -> Self {
        STA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STA` writer - START flag."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `I2EN` reader - I2C interface enable."]
pub struct I2EN_R(crate::FieldReader<bool, bool>);
impl I2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2EN` writer - I2C interface enable."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Set Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is set. Writing a zero has no effect on the corresponding bit in the I2C control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conset](index.html) module"]
pub struct CONSET_SPEC;
impl crate::RegisterSpec for CONSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conset::R](R) reader structure"]
impl crate::Readable for CONSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conset::W](W) writer structure"]
impl crate::Writable for CONSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONSET to value 0"]
impl crate::Resettable for CONSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
