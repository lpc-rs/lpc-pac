#[doc = "Register `PREAC2FSCOEF` reader"]
pub struct R(crate::R<PREAC2FSCOEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREAC2FSCOEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PREAC2FSCOEF_SPEC>> for R {
    fn from(reader: crate::R<PREAC2FSCOEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREAC2FSCOEF` writer"]
pub struct W(crate::W<PREAC2FSCOEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREAC2FSCOEF_SPEC>;
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
impl core::convert::From<crate::W<PREAC2FSCOEF_SPEC>> for W {
    fn from(writer: crate::W<PREAC2FSCOEF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Pre-emphasis filer coefficient for 2 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
pub struct COMP_R(crate::FieldReader<u8, u8>);
impl COMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP` writer - Pre-emphasis filer coefficient for 2 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pre-emphasis filer coefficient for 2 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pre-emphasis filer coefficient for 2 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pre-Emphasis Filter Coefficient for 2 FS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preac2fscoef](index.html) module"]
pub struct PREAC2FSCOEF_SPEC;
impl crate::RegisterSpec for PREAC2FSCOEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [preac2fscoef::R](R) reader structure"]
impl crate::Readable for PREAC2FSCOEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preac2fscoef::W](W) writer structure"]
impl crate::Writable for PREAC2FSCOEF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREAC2FSCOEF to value 0"]
impl crate::Resettable for PREAC2FSCOEF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
