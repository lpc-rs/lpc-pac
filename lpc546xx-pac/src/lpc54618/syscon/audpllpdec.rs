#[doc = "Register `AUDPLLPDEC` reader"]
pub struct R(crate::R<AUDPLLPDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDPLLPDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDPLLPDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDPLLPDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDPLLPDEC` writer"]
pub struct W(crate::W<AUDPLLPDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDPLLPDEC_SPEC>;
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
impl From<crate::W<AUDPLLPDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDPLLPDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDEC` reader - Decoded P-divider coefficient value."]
pub struct PDEC_R(crate::FieldReader<u8, u8>);
impl PDEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEC` writer - Decoded P-divider coefficient value."]
pub struct PDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PREQ` reader - PDEC reload request."]
pub struct PREQ_R(crate::FieldReader<bool, bool>);
impl PREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREQ` writer - PDEC reload request."]
pub struct PREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Decoded P-divider coefficient value."]
    #[inline(always)]
    pub fn pdec(&self) -> PDEC_R {
        PDEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - PDEC reload request."]
    #[inline(always)]
    pub fn preq(&self) -> PREQ_R {
        PREQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Decoded P-divider coefficient value."]
    #[inline(always)]
    pub fn pdec(&mut self) -> PDEC_W {
        PDEC_W { w: self }
    }
    #[doc = "Bit 7 - PDEC reload request."]
    #[inline(always)]
    pub fn preq(&mut self) -> PREQ_W {
        PREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllpdec](index.html) module"]
pub struct AUDPLLPDEC_SPEC;
impl crate::RegisterSpec for AUDPLLPDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audpllpdec::R](R) reader structure"]
impl crate::Readable for AUDPLLPDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audpllpdec::W](W) writer structure"]
impl crate::Writable for AUDPLLPDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDPLLPDEC to value 0"]
impl crate::Resettable for AUDPLLPDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
