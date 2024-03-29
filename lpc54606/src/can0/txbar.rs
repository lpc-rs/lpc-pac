#[doc = "Register `TXBAR` reader"]
pub struct R(crate::R<TXBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBAR` writer"]
pub struct W(crate::W<TXBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBAR_SPEC>;
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
impl From<crate::W<TXBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR` reader - Add request."]
pub struct AR_R(crate::FieldReader<u32, u32>);
impl AR_R {
    pub(crate) fn new(bits: u32) -> Self {
        AR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR` writer - Add request."]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Add request."]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Add request."]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Add Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbar](index.html) module"]
pub struct TXBAR_SPEC;
impl crate::RegisterSpec for TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbar::R](R) reader structure"]
impl crate::Readable for TXBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbar::W](W) writer structure"]
impl crate::Writable for TXBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBAR to value 0"]
impl crate::Resettable for TXBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
