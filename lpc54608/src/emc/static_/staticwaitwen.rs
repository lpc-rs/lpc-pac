#[doc = "Register `STATICWAITWEN` reader"]
pub struct R(crate::R<STATICWAITWEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICWAITWEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICWAITWEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICWAITWEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICWAITWEN` writer"]
pub struct W(crate::W<STATICWAITWEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICWAITWEN_SPEC>;
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
impl From<crate::W<STATICWAITWEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICWAITWEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITWEN` reader - Wait write enable."]
pub struct WAITWEN_R(crate::FieldReader<u8, u8>);
impl WAITWEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITWEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITWEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITWEN` writer - Wait write enable."]
pub struct WAITWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITWEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait write enable."]
    #[inline(always)]
    pub fn waitwen(&self) -> WAITWEN_R {
        WAITWEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait write enable."]
    #[inline(always)]
    pub fn waitwen(&mut self) -> WAITWEN_W {
        WAITWEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay from EMC_CSx to write enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitwen](index.html) module"]
pub struct STATICWAITWEN_SPEC;
impl crate::RegisterSpec for STATICWAITWEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticwaitwen::R](R) reader structure"]
impl crate::Readable for STATICWAITWEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticwaitwen::W](W) writer structure"]
impl crate::Writable for STATICWAITWEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICWAITWEN to value 0"]
impl crate::Resettable for STATICWAITWEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
