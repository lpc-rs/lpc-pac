#[doc = "Register `COMPVAL` reader"]
pub struct R(crate::R<COMPVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPVAL` writer"]
pub struct W(crate::W<COMPVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPVAL_SPEC>;
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
impl From<crate::W<COMPVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RICOMP` reader - ."]
pub struct RICOMP_R(crate::FieldReader<u32, u32>);
impl RICOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RICOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RICOMP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RICOMP` writer - ."]
pub struct RICOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn ricomp(&self) -> RICOMP_R {
        RICOMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn ricomp(&mut self) -> RICOMP_W {
        RICOMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare value LSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compval](index.html) module"]
pub struct COMPVAL_SPEC;
impl crate::RegisterSpec for COMPVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compval::R](R) reader structure"]
impl crate::Readable for COMPVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compval::W](W) writer structure"]
impl crate::Writable for COMPVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPVAL to value 0xffff_ffff"]
impl crate::Resettable for COMPVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
