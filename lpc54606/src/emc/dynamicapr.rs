#[doc = "Register `DYNAMICAPR` reader"]
pub struct R(crate::R<DYNAMICAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DYNAMICAPR_SPEC>> for R {
    fn from(reader: crate::R<DYNAMICAPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICAPR` writer"]
pub struct W(crate::W<DYNAMICAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICAPR_SPEC>;
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
impl core::convert::From<crate::W<DYNAMICAPR_SPEC>> for W {
    fn from(writer: crate::W<DYNAMICAPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAPR` reader - Last-data-out to active command time."]
pub struct TAPR_R(crate::FieldReader<u8, u8>);
impl TAPR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAPR` writer - Last-data-out to active command time."]
pub struct TAPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Last-data-out to active command time."]
    #[inline(always)]
    pub fn tapr(&self) -> TAPR_R {
        TAPR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Last-data-out to active command time."]
    #[inline(always)]
    pub fn tapr(&mut self) -> TAPR_W {
        TAPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last-data-out to active command time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicapr](index.html) module"]
pub struct DYNAMICAPR_SPEC;
impl crate::RegisterSpec for DYNAMICAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicapr::R](R) reader structure"]
impl crate::Readable for DYNAMICAPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicapr::W](W) writer structure"]
impl crate::Writable for DYNAMICAPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICAPR to value 0x0f"]
impl crate::Resettable for DYNAMICAPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
