#[doc = "Register `PLDMND` reader"]
pub struct R(crate::R<PLDMND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLDMND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PLDMND_SPEC>> for R {
    fn from(reader: crate::R<PLDMND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLDMND` writer"]
pub struct W(crate::W<PLDMND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLDMND_SPEC>;
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
impl core::convert::From<crate::W<PLDMND_SPEC>> for W {
    fn from(writer: crate::W<PLDMND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - Poll Demand."]
pub struct PD_R(crate::FieldReader<u32, u32>);
impl PD_R {
    pub(crate) fn new(bits: u32) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - Poll Demand."]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Poll Demand."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Poll Demand."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Poll Demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pldmnd](index.html) module"]
pub struct PLDMND_SPEC;
impl crate::RegisterSpec for PLDMND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pldmnd::R](R) reader structure"]
impl crate::Readable for PLDMND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pldmnd::W](W) writer structure"]
impl crate::Writable for PLDMND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLDMND to value 0"]
impl crate::Resettable for PLDMND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
