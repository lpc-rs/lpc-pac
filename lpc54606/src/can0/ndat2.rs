#[doc = "Register `NDAT2` reader"]
pub struct R(crate::R<NDAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NDAT2_SPEC>> for R {
    fn from(reader: crate::R<NDAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NDAT2` writer"]
pub struct W(crate::W<NDAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDAT2_SPEC>;
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
impl core::convert::From<crate::W<NDAT2_SPEC>> for W {
    fn from(writer: crate::W<NDAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ND` reader - New Data."]
pub struct ND_R(crate::FieldReader<u32, u32>);
impl ND_R {
    pub(crate) fn new(bits: u32) -> Self {
        ND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND` writer - New Data."]
pub struct ND_W<'a> {
    w: &'a mut W,
}
impl<'a> ND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - New Data."]
    #[inline(always)]
    pub fn nd(&self) -> ND_R {
        ND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - New Data."]
    #[inline(always)]
    pub fn nd(&mut self) -> ND_W {
        ND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "New Data 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat2](index.html) module"]
pub struct NDAT2_SPEC;
impl crate::RegisterSpec for NDAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndat2::R](R) reader structure"]
impl crate::Readable for NDAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ndat2::W](W) writer structure"]
impl crate::Writable for NDAT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NDAT2 to value 0"]
impl crate::Resettable for NDAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
