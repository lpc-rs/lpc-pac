#[doc = "Register `HCPERIODCURRENTED` reader"]
pub struct R(crate::R<HCPERIODCURRENTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCPERIODCURRENTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCPERIODCURRENTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCPERIODCURRENTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCPERIODCURRENTED` writer"]
pub struct W(crate::W<HCPERIODCURRENTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCPERIODCURRENTED_SPEC>;
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
impl From<crate::W<HCPERIODCURRENTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCPERIODCURRENTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCED` reader - The content of this register is updated by HC after a periodic ED is processed."]
pub struct PCED_R(crate::FieldReader<u32, u32>);
impl PCED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PCED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCED` writer - The content of this register is updated by HC after a periodic ED is processed."]
pub struct PCED_W<'a> {
    w: &'a mut W,
}
impl<'a> PCED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub fn pced(&self) -> PCED_R {
        PCED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub fn pced(&mut self) -> PCED_W {
        PCED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcperiodcurrented](index.html) module"]
pub struct HCPERIODCURRENTED_SPEC;
impl crate::RegisterSpec for HCPERIODCURRENTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcperiodcurrented::R](R) reader structure"]
impl crate::Readable for HCPERIODCURRENTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcperiodcurrented::W](W) writer structure"]
impl crate::Writable for HCPERIODCURRENTED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCPERIODCURRENTED to value 0"]
impl crate::Resettable for HCPERIODCURRENTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
