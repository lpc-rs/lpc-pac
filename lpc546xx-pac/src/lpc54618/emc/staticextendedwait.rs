#[doc = "Register `STATICEXTENDEDWAIT` reader"]
pub struct R(crate::R<STATICEXTENDEDWAIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICEXTENDEDWAIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICEXTENDEDWAIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICEXTENDEDWAIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICEXTENDEDWAIT` writer"]
pub struct W(crate::W<STATICEXTENDEDWAIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICEXTENDEDWAIT_SPEC>;
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
impl From<crate::W<STATICEXTENDEDWAIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICEXTENDEDWAIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTENDEDWAIT` reader - Extended wait time out."]
pub struct EXTENDEDWAIT_R(crate::FieldReader<u16, u16>);
impl EXTENDEDWAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EXTENDEDWAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTENDEDWAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTENDEDWAIT` writer - Extended wait time out."]
pub struct EXTENDEDWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTENDEDWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Extended wait time out."]
    #[inline(always)]
    pub fn extendedwait(&self) -> EXTENDEDWAIT_R {
        EXTENDEDWAIT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Extended wait time out."]
    #[inline(always)]
    pub fn extendedwait(&mut self) -> EXTENDEDWAIT_W {
        EXTENDEDWAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time for long static memory read and write transfers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticextendedwait](index.html) module"]
pub struct STATICEXTENDEDWAIT_SPEC;
impl crate::RegisterSpec for STATICEXTENDEDWAIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticextendedwait::R](R) reader structure"]
impl crate::Readable for STATICEXTENDEDWAIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticextendedwait::W](W) writer structure"]
impl crate::Writable for STATICEXTENDEDWAIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICEXTENDEDWAIT to value 0"]
impl crate::Resettable for STATICEXTENDEDWAIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
