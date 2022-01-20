#[doc = "Register `STATICWAITPAGE` reader"]
pub struct R(crate::R<STATICWAITPAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICWAITPAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICWAITPAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICWAITPAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICWAITPAGE` writer"]
pub struct W(crate::W<STATICWAITPAGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICWAITPAGE_SPEC>;
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
impl From<crate::W<STATICWAITPAGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICWAITPAGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITPAGE` reader - Asynchronous page mode read after the first read wait states."]
pub struct WAITPAGE_R(crate::FieldReader<u8, u8>);
impl WAITPAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAITPAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITPAGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITPAGE` writer - Asynchronous page mode read after the first read wait states."]
pub struct WAITPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITPAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Asynchronous page mode read after the first read wait states."]
    #[inline(always)]
    pub fn waitpage(&self) -> WAITPAGE_R {
        WAITPAGE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Asynchronous page mode read after the first read wait states."]
    #[inline(always)]
    pub fn waitpage(&mut self) -> WAITPAGE_W {
        WAITPAGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitpage](index.html) module"]
pub struct STATICWAITPAGE_SPEC;
impl crate::RegisterSpec for STATICWAITPAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticwaitpage::R](R) reader structure"]
impl crate::Readable for STATICWAITPAGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticwaitpage::W](W) writer structure"]
impl crate::Writable for STATICWAITPAGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICWAITPAGE to value 0x1f"]
impl crate::Resettable for STATICWAITPAGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
