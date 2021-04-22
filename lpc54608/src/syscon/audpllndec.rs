#[doc = "Register `AUDPLLNDEC` reader"]
pub struct R(crate::R<AUDPLLNDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDPLLNDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AUDPLLNDEC_SPEC>> for R {
    fn from(reader: crate::R<AUDPLLNDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDPLLNDEC` writer"]
pub struct W(crate::W<AUDPLLNDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDPLLNDEC_SPEC>;
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
impl core::convert::From<crate::W<AUDPLLNDEC_SPEC>> for W {
    fn from(writer: crate::W<AUDPLLNDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDEC` reader - Decoded N-divider coefficient value."]
pub struct NDEC_R(crate::FieldReader<u16, u16>);
impl NDEC_R {
    pub(crate) fn new(bits: u16) -> Self {
        NDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDEC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDEC` writer - Decoded N-divider coefficient value."]
pub struct NDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NDEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `NREQ` reader - NDEC reload request."]
pub struct NREQ_R(crate::FieldReader<bool, bool>);
impl NREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        NREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NREQ` writer - NDEC reload request."]
pub struct NREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Decoded N-divider coefficient value."]
    #[inline(always)]
    pub fn ndec(&self) -> NDEC_R {
        NDEC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - NDEC reload request."]
    #[inline(always)]
    pub fn nreq(&self) -> NREQ_R {
        NREQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Decoded N-divider coefficient value."]
    #[inline(always)]
    pub fn ndec(&mut self) -> NDEC_W {
        NDEC_W { w: self }
    }
    #[doc = "Bit 10 - NDEC reload request."]
    #[inline(always)]
    pub fn nreq(&mut self) -> NREQ_W {
        NREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllndec](index.html) module"]
pub struct AUDPLLNDEC_SPEC;
impl crate::RegisterSpec for AUDPLLNDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audpllndec::R](R) reader structure"]
impl crate::Readable for AUDPLLNDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audpllndec::W](W) writer structure"]
impl crate::Writable for AUDPLLNDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDPLLNDEC to value 0"]
impl crate::Resettable for AUDPLLNDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
