#[doc = "Register `DYNAMICSREX` reader"]
pub struct R(crate::R<DYNAMICSREX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICSREX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DYNAMICSREX_SPEC>> for R {
    fn from(reader: crate::R<DYNAMICSREX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICSREX` writer"]
pub struct W(crate::W<DYNAMICSREX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICSREX_SPEC>;
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
impl core::convert::From<crate::W<DYNAMICSREX_SPEC>> for W {
    fn from(writer: crate::W<DYNAMICSREX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSREX` reader - Self-refresh exit time."]
pub struct TSREX_R(crate::FieldReader<u8, u8>);
impl TSREX_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSREX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSREX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSREX` writer - Self-refresh exit time."]
pub struct TSREX_W<'a> {
    w: &'a mut W,
}
impl<'a> TSREX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Self-refresh exit time."]
    #[inline(always)]
    pub fn tsrex(&self) -> TSREX_R {
        TSREX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Self-refresh exit time."]
    #[inline(always)]
    pub fn tsrex(&mut self) -> TSREX_W {
        TSREX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Self-refresh exit time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicsrex](index.html) module"]
pub struct DYNAMICSREX_SPEC;
impl crate::RegisterSpec for DYNAMICSREX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicsrex::R](R) reader structure"]
impl crate::Readable for DYNAMICSREX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicsrex::W](W) writer structure"]
impl crate::Writable for DYNAMICSREX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICSREX to value 0x0f"]
impl crate::Resettable for DYNAMICSREX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
