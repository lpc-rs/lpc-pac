#[doc = "Register `IER` reader"]
pub struct R(crate::R<DLM_IER_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLM_IER_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLM_IER_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLM_IER_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<DLM_IER_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLM_IER_IER_SPEC>;
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
impl From<crate::W<DLM_IER_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLM_IER_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable."]
pub struct RBRIE_R(crate::FieldReader<bool, bool>);
impl RBRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable."]
pub struct RBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRIE_W<'a> {
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
#[doc = "Field `THREIE` reader - THRE Interrupt Enable."]
pub struct THREIE_R(crate::FieldReader<bool, bool>);
impl THREIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THREIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THREIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THREIE` writer - THRE Interrupt Enable."]
pub struct THREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREIE_W<'a> {
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
#[doc = "Field `RXIE` reader - RX Line Status Interrupt Enable."]
pub struct RXIE_R(crate::FieldReader<bool, bool>);
impl RXIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIE` writer - RX Line Status Interrupt Enable."]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable."]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable."]
    #[inline(always)]
    pub fn threie(&self) -> THREIE_R {
        THREIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable."]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RBRIE_W {
        RBRIE_W { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable."]
    #[inline(always)]
    pub fn threie(&mut self) -> THREIE_W {
        THREIE_W { w: self }
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable."]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlm_ier_ier](index.html) module"]
pub struct DLM_IER_IER_SPEC;
impl crate::RegisterSpec for DLM_IER_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlm_ier_ier::R](R) reader structure"]
impl crate::Readable for DLM_IER_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlm_ier_ier::W](W) writer structure"]
impl crate::Writable for DLM_IER_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for DLM_IER_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
