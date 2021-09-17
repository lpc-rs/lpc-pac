#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTAT` writer"]
pub struct W(crate::W<INTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTAT_SPEC>;
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
impl From<crate::W<INTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YESTOUCH` reader - the status of touch interrrupt"]
pub struct YESTOUCH_R(crate::FieldReader<bool, bool>);
impl YESTOUCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        YESTOUCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YESTOUCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOTOUCH` reader - the status of no-touch interrrupt"]
pub struct NOTOUCH_R(crate::FieldReader<bool, bool>);
impl NOTOUCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOTOUCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOTOUCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLLDONE` reader - the status of poll or pollnow completing interrupt"]
pub struct POLLDONE_R(crate::FieldReader<bool, bool>);
impl POLLDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        POLLDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLLDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` reader - the status of timeout interrupt"]
pub struct TIMEOUT_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERUN` reader - the status of overrun interrupt"]
pub struct OVERUN_R(crate::FieldReader<bool, bool>);
impl OVERUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - the status of touch interrrupt"]
    #[inline(always)]
    pub fn yestouch(&self) -> YESTOUCH_R {
        YESTOUCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - the status of no-touch interrrupt"]
    #[inline(always)]
    pub fn notouch(&self) -> NOTOUCH_R {
        NOTOUCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - the status of poll or pollnow completing interrupt"]
    #[inline(always)]
    pub fn polldone(&self) -> POLLDONE_R {
        POLLDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - the status of timeout interrupt"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - the status of overrun interrupt"]
    #[inline(always)]
    pub fn overun(&self) -> OVERUN_R {
        OVERUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status (mask of STATUS and INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstat::W](W) writer structure"]
impl crate::Writable for INTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
