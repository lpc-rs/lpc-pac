#[doc = "Register `CNTRLDVR1` reader"]
pub struct R(crate::R<CNTRLDVR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTRLDVR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTRLDVR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTRLDVR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTRLDVR1` writer"]
pub struct W(crate::W<CNTRLDVR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTRLDVR1_SPEC>;
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
impl From<crate::W<CNTRLDVR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTRLDVR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IntitialCount` reader - Initial count."]
pub struct INTITIALCOUNT_R(crate::FieldReader<u16, u16>);
impl INTITIALCOUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        INTITIALCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTITIALCOUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IntitialCount` writer - Initial count."]
pub struct INTITIALCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTITIALCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Initial count."]
    #[inline(always)]
    pub fn intitial_count(&self) -> INTITIALCOUNT_R {
        INTITIALCOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initial count."]
    #[inline(always)]
    pub fn intitial_count(&mut self) -> INTITIALCOUNT_W {
        INTITIALCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Free-running counter reload value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntrldvr1](index.html) module"]
pub struct CNTRLDVR1_SPEC;
impl crate::RegisterSpec for CNTRLDVR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntrldvr1::R](R) reader structure"]
impl crate::Readable for CNTRLDVR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntrldvr1::W](W) writer structure"]
impl crate::Writable for CNTRLDVR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTRLDVR1 to value 0"]
impl crate::Resettable for CNTRLDVR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
