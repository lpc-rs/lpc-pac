#[doc = "Register `DYNAMICCONTROL` reader"]
pub struct R(crate::R<DYNAMICCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICCONTROL` writer"]
pub struct W(crate::W<DYNAMICCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICCONTROL_SPEC>;
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
impl From<crate::W<DYNAMICCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CE` reader - Dynamic memory clock enable."]
pub struct CE_R(crate::FieldReader<bool, bool>);
impl CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE` writer - Dynamic memory clock enable."]
pub struct CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CS` reader - Dynamic memory clock control."]
pub struct CS_R(crate::FieldReader<bool, bool>);
impl CS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS` writer - Dynamic memory clock control."]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SR` reader - Self-refresh request, EMCSREFREQ."]
pub struct SR_R(crate::FieldReader<bool, bool>);
impl SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - Self-refresh request, EMCSREFREQ."]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
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
#[doc = "Field `MMC` reader - Memory clock control."]
pub struct MMC_R(crate::FieldReader<bool, bool>);
impl MMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMC` writer - Memory clock control."]
pub struct MMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MMC_W<'a> {
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
#[doc = "Field `I` reader - SDRAM initialization."]
pub struct I_R(crate::FieldReader<u8, u8>);
impl I_R {
    pub(crate) fn new(bits: u8) -> Self {
        I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I` writer - SDRAM initialization."]
pub struct I_W<'a> {
    w: &'a mut W,
}
impl<'a> I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dynamic memory clock control."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ."]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W {
        CE_W { w: self }
    }
    #[doc = "Bit 1 - Dynamic memory clock control."]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ."]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W { w: self }
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&mut self) -> I_W {
        I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls dynamic memory operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamiccontrol](index.html) module"]
pub struct DYNAMICCONTROL_SPEC;
impl crate::RegisterSpec for DYNAMICCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamiccontrol::R](R) reader structure"]
impl crate::Readable for DYNAMICCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamiccontrol::W](W) writer structure"]
impl crate::Writable for DYNAMICCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICCONTROL to value 0x06"]
impl crate::Resettable for DYNAMICCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
