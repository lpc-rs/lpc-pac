#[doc = "Register `CHANEN` reader"]
pub struct R(crate::R<CHANEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHANEN_SPEC>> for R {
    fn from(reader: crate::R<CHANEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANEN` writer"]
pub struct W(crate::W<CHANEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANEN_SPEC>;
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
impl core::convert::From<crate::W<CHANEN_SPEC>> for W {
    fn from(writer: crate::W<CHANEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_CH0` reader - Enable channel 0. When 1, PDM channel 0 is enabled."]
pub struct EN_CH0_R(crate::FieldReader<bool, bool>);
impl EN_CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_CH0` writer - Enable channel 0. When 1, PDM channel 0 is enabled."]
pub struct EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CH0_W<'a> {
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
#[doc = "Field `EN_CH1` reader - Enable channel 1. When 1, PDM channel 1 is enabled."]
pub struct EN_CH1_R(crate::FieldReader<bool, bool>);
impl EN_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_CH1` writer - Enable channel 1. When 1, PDM channel 1 is enabled."]
pub struct EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CH1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&self) -> EN_CH0_R {
        EN_CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&self) -> EN_CH1_R {
        EN_CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&mut self) -> EN_CH0_W {
        EN_CH0_W { w: self }
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&mut self) -> EN_CH1_W {
        EN_CH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chanen](index.html) module"]
pub struct CHANEN_SPEC;
impl crate::RegisterSpec for CHANEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chanen::R](R) reader structure"]
impl crate::Readable for CHANEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chanen::W](W) writer structure"]
impl crate::Writable for CHANEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANEN to value 0"]
impl crate::Resettable for CHANEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
