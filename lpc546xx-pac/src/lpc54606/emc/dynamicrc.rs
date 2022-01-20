#[doc = "Register `DYNAMICRC` reader"]
pub struct R(crate::R<DYNAMICRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICRC` writer"]
pub struct W(crate::W<DYNAMICRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRC_SPEC>;
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
impl From<crate::W<DYNAMICRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRC` reader - Active to active command period."]
pub struct TRC_R(crate::FieldReader<u8, u8>);
impl TRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRC` writer - Active to active command period."]
pub struct TRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Active to active command period."]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Active to active command period."]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W {
        TRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the active to active command period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrc](index.html) module"]
pub struct DYNAMICRC_SPEC;
impl crate::RegisterSpec for DYNAMICRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicrc::R](R) reader structure"]
impl crate::Readable for DYNAMICRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicrc::W](W) writer structure"]
impl crate::Writable for DYNAMICRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICRC to value 0x1f"]
impl crate::Resettable for DYNAMICRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
