#[doc = "Register `TXBTO` reader"]
pub struct R(crate::R<TXBTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBTO` writer"]
pub struct W(crate::W<TXBTO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBTO_SPEC>;
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
impl From<crate::W<TXBTO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBTO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - Transmission occurred."]
pub struct TO_R(crate::FieldReader<u32, u32>);
impl TO_R {
    pub(crate) fn new(bits: u32) -> Self {
        TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO` writer - Transmission occurred."]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmission occurred."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission occurred."]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Transmission Occurred\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](index.html) module"]
pub struct TXBTO_SPEC;
impl crate::RegisterSpec for TXBTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbto::R](R) reader structure"]
impl crate::Readable for TXBTO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbto::W](W) writer structure"]
impl crate::Writable for TXBTO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TXBTO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
