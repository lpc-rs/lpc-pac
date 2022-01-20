#[doc = "Register `PORTMODE` reader"]
pub struct R(crate::R<PORTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTMODE` writer"]
pub struct W(crate::W<PORTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTMODE_SPEC>;
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
impl From<crate::W<PORTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Port ID pin value."]
pub struct ID_R(crate::FieldReader<bool, bool>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - Port ID pin value."]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
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
#[doc = "Field `ID_EN` reader - Port ID pin pull-up enable."]
pub struct ID_EN_R(crate::FieldReader<bool, bool>);
impl ID_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID_EN` writer - Port ID pin pull-up enable."]
pub struct ID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DEV_ENABLE` reader - 1: device 0: host."]
pub struct DEV_ENABLE_R(crate::FieldReader<bool, bool>);
impl DEV_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_ENABLE` writer - 1: device 0: host."]
pub struct DEV_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    pub fn id_en(&self) -> ID_EN_R {
        ID_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    pub fn id_en(&mut self) -> ID_EN_W {
        ID_EN_W { w: self }
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    pub fn dev_enable(&mut self) -> DEV_ENABLE_W {
        DEV_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portmode](index.html) module"]
pub struct PORTMODE_SPEC;
impl crate::RegisterSpec for PORTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portmode::R](R) reader structure"]
impl crate::Readable for PORTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portmode::W](W) writer structure"]
impl crate::Writable for PORTMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTMODE to value 0"]
impl crate::Resettable for PORTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
